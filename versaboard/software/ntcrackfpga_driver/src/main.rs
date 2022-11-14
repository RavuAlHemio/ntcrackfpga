#![no_std]
#![no_main]


mod byte_buffer;


use core::panic::PanicInfo;

use atsaml21g18b::{interrupt, Peripherals};
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use ntcrackfpga_versaboard::{board_pin, uart};

use crate::byte_buffer::ByteBuffer;
use crate::max7300::{Max7300, PinBankConfig, PinConfig};

const HASH_LENGTH: usize = 16;
const PASSWORD_LENGTH: usize = 20;

static mut UART_BUFFER: ByteBuffer<64> = ByteBuffer::new();


enum CrackState {
    AwaitingHashInput,
    SendingHash,
    Cracking,
    PasswordOutput,
    Finished,
}
impl CrackState {
    #[inline]
    pub fn may_input_hashes(&self) -> bool {
        match self {
            Self::AwaitingHashInput|Self::Finished => true,
            _ => false,
        }
    }

    #[inline]
    pub fn may_go(&self) -> bool {
        match self {
            Self::AwaitingHashInput|Self::Finished => true,
            _ => false,
        }
    }
}


#[panic_handler]
fn panic_handler(_why: &PanicInfo) -> ! {
    // set PA24 (on-board LED) to output
    let mut peripherals = unsafe { Peripherals::steal() };
    board_pin!(set_io, &mut peripherals, PA, 24);
    board_pin!(make_output, &mut peripherals, PA, 24);

    loop {
        // make blink
        board_pin!(set_high, &mut peripherals, PA, 24);
        sleepiness();
        board_pin!(set_low, &mut peripherals, PA, 24);
        sleepiness();
    }
}

#[inline]
fn nibble_to_hex(nibble: u8) -> u8 {
    match nibble {
        0x0..=0x9 => b'0' + nibble,
        0xA..=0xF => b'A' + nibble - 10,
        _ => b'?',
    }
}
fn byte_to_hex(byte: u8) -> [u8; 2] {
    [
        nibble_to_hex(byte >> 4),
        nibble_to_hex(byte & 0xF),
    ]
}
#[inline]
fn hex_to_nibble(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'A'..=b'F' => hex - b'A' + 0xA,
        b'a'..=b'f' => hex - b'a' + 0xA,
        _ => 0xFF,
    }
}

fn sleepiness() {
    for _ in 0..0xFFFFF {
        nop();
    }
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take()
        .expect("failed to obtain peripherals");

    // set up chip
    ntcrackfpga_versaboard::init::initialize(&mut peripherals);

    // turn on LED
    board_pin!(set_io, &mut peripherals, PA, 24);
    board_pin!(make_output, &mut peripherals, PA, 24);
    board_pin!(set_high, &mut peripherals, PA, 24);

    // set up I/O pins fo FPGA communication
    board_pin!(set_io, &mut peripherals, PA, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23);
    board_pin!(set_io, &mut peripherals, PB, 8, 9);

    board_pin!(set_low, &mut peripherals, PA, 8, 9, 10, 11, 12, 13, 14, 15);
    board_pin!(set_low, &mut peripherals, PB, 8, 9);
    board_pin!(disable_pull, &mut peripherals, PA, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23);

    board_pin!(make_output, &mut peripherals, PA, 8, 9, 10, 11, 12, 13, 14, 15);
    board_pin!(make_output, &mut peripherals, PB, 8, 9);
    board_pin!(make_input, &mut peripherals, PA, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23);

    // set up UART for communicating with host, including interrupt
    uart::set_up(&mut peripherals);
    uart::enable_receive_interrupt(&mut peripherals);

    uart::send(&mut peripherals, b"\r\nntcrackfpga driver\r\n>");

    let mut state = CrackState::AwaitingHashInput;
    let mut cmdline_buffer: ByteBuffer<128> = ByteBuffer::new();
    let mut ignoring_until_enter = false;
    let mut hash_buffer = [0u8; HASH_LENGTH];
    let mut password_buffer = [0u8; PASSWORD_LENGTH + 1];
    let mut current_index: usize = 0;
    loop {
        // wait for an interrupt
        cortex_m::asm::wfi();

        // check UART
        let have_bytes = unsafe { UART_BUFFER.len() > 0 };
        if have_bytes {
            // process whatever the UART dragged in
            let uart_buf = unsafe { UART_BUFFER.critical_take() };

            for &b in uart_buf.as_slice() {
                if b == 0x0C {
                    // carriage return
                    if ignoring_until_enter {
                        ignoring_until_enter = false;
                    } else if cmdline_buffer.len() > 0 {
                        // process the contents of the commandline buffer
                        let buf_slice = cmdline_buffer.as_slice();
                        if buf_slice == b"help" {
                            uart::send(
                                &mut peripherals,
                                concat!(
                                    "\r\nAvailable commands are:",
                                    "\r\n  help",
                                    "\r\n            this text",
                                    "\r\n  hXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
                                    "\r\n            add an MD4 hash to the database of known hashes",
                                    "\r\n            (X = hex digit)",
                                    "\r\n  g",
                                    "\r\n            start (go)",
                                    "\r\n  s",
                                    "\r\n            output status information",
                                ).as_bytes(),
                            );
                        } else if buf_slice[0] == b'h' {
                            let mut bad = false;
                            if !bad && !state.may_input_hashes() {
                                uart::send(&mut peripherals, b"\r\nHashes cannot be added in the current state.");
                                bad = true;
                            }

                            if !bad && buf_slice.len() != 33 {
                                uart::send(&mut peripherals, b"\r\nHash following 'h' must be 32 hex digits long.");
                                bad = true;
                            }

                            if !bad {
                                for i in 0..hash_buffer.len() {
                                    let top_nibble = hex_to_nibble(buf_slice[1 + 2*i]);
                                    if top_nibble == 0xFF {
                                        bad = true;
                                        let mut buf: [u8; 24] = *b"\r\nInvalid hex digit '_'.";
                                        buf[21] = buf_slice[1 + 2*i];
                                        uart::send(&mut peripherals, &buf);
                                        break;
                                    }
                                    let bottom_nibble = hex_to_nibble(buf_slice[1 + 2*i + 1]);
                                    if bottom_nibble == 0xFF {
                                        bad = true;
                                        let mut buf: [u8; 24] = *b"\r\nInvalid hex digit '_'.";
                                        buf[21] = buf_slice[1 + 2*i + 1];
                                        uart::send(&mut peripherals, &buf);
                                        break;
                                    }

                                    hash_buffer[i] = (top_nibble << 4) | bottom_nibble;
                                }
                            }

                            if !bad {
                                // okay, send the hash over
                                current_index = 0;
                                state = CrackState::SendingHash;
                            }
                        } else if buf_slice == b"g" {
                            let mut bad = false;
                            if !bad && !state.may_go() {
                                uart::send(&mut peripherals, b"\r\nCracking cannot be started in the current state.");
                                bad = true;
                            }

                            if !bad {
                                // pull "go" high
                                expander.write_pins(&mut peripherals, EXPANDER_GO_PIN, &[true]);

                                // pull "go" low
                                expander.write_pins(&mut peripherals, EXPANDER_GO_PIN, &[false]);

                                state = CrackState::Cracking;
                                uart::send(&mut peripherals, b"\r\nCracking started.");
                            }
                        } else if buf_slice == b"s" {
                            uart::send(
                                &mut peripherals,
                                match state {
                                    CrackState::AwaitingHashInput => b"\r\nawaiting hash input",
                                    CrackState::SendingHash => b"\r\nsending hash to FPGA",
                                    CrackState::Cracking => b"\r\nFPGA is cracking",
                                    CrackState::PasswordOutput => b"\r\npassword is being output",
                                    CrackState::Finished => b"\r\ncracking finished",
                                }
                            );
                        } else {
                            uart::send(
                                &mut peripherals, b"\r\nUnknown command; type \"help\" for help.",
                            );
                        }
                    }
                    cmdline_buffer.clear();
                    uart::send(&mut peripherals, b"\r\n>");
                    continue;
                }

                if ignoring_until_enter {
                    continue;
                }

                match b {
                    0x08 => {
                        // backspace
                        if cmdline_buffer.len() > 0 {
                            cmdline_buffer.pop();
                            // wipe out the previous character by replacing it with a space and going back
                            uart::send(&mut peripherals, b"\x08 \x08");
                        }
                    },
                    0x0C => unreachable!(), // handled above
                    0x09|0x20 => {
                        // tab or space; silently ignore it
                    },
                    0x21..=0x7E => {
                        // printable character
                        let stored = cmdline_buffer.push(b);
                        if !stored {
                            // we've run out of space
                            uart::send(&mut peripherals, b"\r\ndude, I'm a microcontroller, max command line length is 128");
                            uart::send(&mut peripherals, b"\r\nsince you might have inadvertently pasted something,\r\nI'll be ignoring anything you type until the next carriage return (Enter)");
                            ignoring_until_enter = true;
                        }
                        // echo back
                        uart::send(&mut peripherals, &[b]);
                    },
                    _ => {
                        uart::send(&mut peripherals, b"\r\ndude, I'm a microcontroller, I can't deal with weird control characters");
                        uart::send(&mut peripherals, b"\r\nsince you might have inadvertently pasted something,\r\nI'll be ignoring anything you type until the next carriage return (Enter)");
                    },
                }
            }
        }

        // anything to do over I2C?
        let my_turn = expander.read_pin(&mut peripherals, EXPANDER_YOUR_TURN_PIN);
        if my_turn {
            match state {
                CrackState::SendingHash => {
                    // set new_hash_byte
                    expander.write_pin_bank(
                        &mut peripherals,
                        EXPANDER_NEW_HASH_BYTE_FIRST_PIN,
                        hash_buffer[current_index],
                    );
                    current_index += 1;

                    // pull "store_hash_byte" high
                    expander.write_pins(&mut peripherals, EXPANDER_STORE_HASH_BYTE_PIN, &[true]);

                    // give it a sec
                    sleepiness();

                    // pull "store_hash_byte" low
                    expander.write_pins(&mut peripherals, EXPANDER_STORE_HASH_BYTE_PIN, &[false]);

                    if current_index == HASH_LENGTH {
                        // full hash transmitted; we're idle again
                        current_index = 0;
                        state = CrackState::AwaitingHashInput;
                    }
                },
                CrackState::PasswordOutput => {
                    // read password_byte
                    password_buffer[current_index] = expander.read_pin_bank(
                        &mut peripherals,
                        EXPANDER_PASSWORD_BYTE_FIRST_PIN,
                    );
                    current_index += 1;

                    if current_index == PASSWORD_LENGTH + 2 {
                        // full password read; slice and output it
                        let password_length: usize = password_buffer[PASSWORD_LENGTH].into();
                        uart::send(&mut peripherals, b"\r\nPassword found: \"");
                        uart::send(&mut peripherals, &password_buffer[0..password_length]);
                        uart::send(&mut peripherals, b"\"\r\n>");
                        uart::send(&mut peripherals, cmdline_buffer.as_slice());

                        current_index = 0;
                        state = CrackState::Cracking;
                    }

                    // pull "go" high
                    expander.write_pins(&mut peripherals, EXPANDER_GO_PIN, &[true]);

                    // give it a sec
                    sleepiness();

                    // pull "go" low
                    expander.write_pins(&mut peripherals, EXPANDER_GO_PIN, &[false]);
                },
                CrackState::Cracking => {
                    // ooh, a state change
                    let match_found = expander.read_pin(&mut peripherals, EXPANDER_MATCH_FOUND_PIN);
                    state = if match_found {
                        CrackState::PasswordOutput
                    } else {
                        uart::send(&mut peripherals, b"\r\nCracking finished!\r\n>");
                        uart::send(&mut peripherals, cmdline_buffer.as_slice());
                        CrackState::Finished
                    };
                },
                _ => {},
            }
        }
    }
}

/// UART interrupt handler
#[interrupt]
fn SERCOM0() {
    let mut peripherals = unsafe { Peripherals::steal() };
    board_pin!(set_low, peripherals, PA, 17);
    if !uart::has_received_byte(&mut peripherals) {
        return;
    }

    let b = uart::read_byte(&mut peripherals);
    unsafe { UART_BUFFER.push(b) };
}

#![no_std]
#![no_main]


mod byte_buffer;


use core::panic::PanicInfo;

use atsaml21g18b::{interrupt, Peripherals};
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use ntcrackfpga_versaboard::{board_pin, uart};

use crate::byte_buffer::ByteBuffer;

const HASH_LENGTH: usize = 16;
const PASSWORD_LENGTH: usize = 20;

static mut UART_BUFFER: ByteBuffer<64> = ByteBuffer::new();


enum CrackState {
    Unknown,
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

    #[inline]
    pub fn can_wait_for_interrupt(&self) -> bool {
        // return false if the microcontroller has to perform the next step
        // return true if the next step is performed by the user or the FPGA
        match self {
            Self::SendingHash|Self::PasswordOutput => false,
            _ => true,
        }
    }
}


#[panic_handler]
fn panic_handler(why: &PanicInfo) -> ! {
    let mut peripherals = unsafe { Peripherals::steal() };

    // try mentioning it on the UART
    uart::send(&mut peripherals, b"PANIC! (not at the disco)\r\n");
    if let Some(s) = why.payload().downcast_ref::<&str>() {
        uart::send(&mut peripherals, b"reason: ");
        uart::send(&mut peripherals, s.as_bytes());
        uart::send(&mut peripherals, b"\r\n");
    }
    if let Some(loc) = why.location() {
        uart::send(&mut peripherals, b"file: ");
        uart::send(&mut peripherals, loc.file().as_bytes());
        uart::send(&mut peripherals, b" line (reversed digits): ");
        let mut line = loc.line();
        while line > 0 {
            let line_digit = (line % 10) as u8;
            line /= 10;
            uart::send(&mut peripherals, &[b'0' + line_digit]);
        }
        uart::send(&mut peripherals, b"\r\n");
    }

    // set PA24 (on-board LED) to output
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

macro_rules! send_uart_pin {
    ($peri:expr, $pinbank:ident, $pinnum:expr) => {
        if board_pin!(read_pin, $peri, $pinbank, $pinnum) {
            uart::send($peri, b"1")
        } else {
            uart::send($peri, b"0")
        }
    };
}

fn set_new_hash_byte(peripherals: &mut Peripherals, byte: u8) {
    // bit 0 = PA08, ... bit 7 = PA15
    const BIT_OFFSET: u32 = 8;

    let mut set_bits: u32 = 0;
    let mut clear_bits: u32 = 0;

    for i in 0..8 {
        if byte & (1 << i) == 0 {
            clear_bits |= 1 << (BIT_OFFSET + i);
        } else {
            set_bits |= 1 << (BIT_OFFSET + i);
        }
    }

    peripherals.PORT.outset0.write(|w| w
        .outset().variant(set_bits)
    );
    peripherals.PORT.outclr0.write(|w| w
        .outclr().variant(clear_bits)
    );
}

#[inline]
fn get_output_byte(peripherals: &mut Peripherals) -> u8 {
    // bit 0 = PA16, ... bit 7 = PA23
    const BIT_OFFSET: u32 = 16;

    ((board_pin!(read_pins, peripherals, PA) >> BIT_OFFSET) & 0xFF).try_into().unwrap()
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

    let mut state = CrackState::Unknown;
    let mut cmdline_buffer: ByteBuffer<128> = ByteBuffer::new();
    let mut ignoring_until_enter = false;
    let mut hash_buffer = [0u8; HASH_LENGTH];
    let mut password_buffer = [0u8; PASSWORD_LENGTH + 1];
    let mut current_index: usize = 0;
    loop {
        if state.can_wait_for_interrupt() {
            // it's not us who has to do the work right now
            cortex_m::asm::wfi();
        }

        // check UART
        let have_bytes = unsafe { UART_BUFFER.len() > 0 };
        if have_bytes {
            // process whatever the UART dragged in
            let uart_buf = unsafe { UART_BUFFER.critical_take() };

            for &b in uart_buf.as_slice() {
                if b == 0x0D {
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
                                    "\r\n  p",
                                    "\r\n            output status of input pins",
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
                                board_pin!(set_high, &mut peripherals, PB, 9);

                                // wait a bit
                                sleepiness();

                                // pull "go" low
                                board_pin!(set_low, &mut peripherals, PB, 9);

                                state = CrackState::Cracking;
                                uart::send(&mut peripherals, b"\r\nCracking started.");
                            }
                        } else if buf_slice == b"s" {
                            uart::send(
                                &mut peripherals,
                                match state {
                                    CrackState::Unknown => b"\r\nunknown",
                                    CrackState::AwaitingHashInput => b"\r\nawaiting hash input",
                                    CrackState::SendingHash => b"\r\nsending hash to FPGA",
                                    CrackState::Cracking => b"\r\nFPGA is cracking",
                                    CrackState::PasswordOutput => b"\r\npassword is being output",
                                    CrackState::Finished => b"\r\ncracking finished",
                                }
                            );
                        } else if buf_slice == b"p" {
                            uart::send(&mut peripherals, b"\r\nmatch found: ");
                            send_uart_pin!(&mut peripherals, PA, 6);
                            uart::send(&mut peripherals, b"\r\nmy turn: ");
                            send_uart_pin!(&mut peripherals, PA, 7);

                            let output_byte = get_output_byte(&mut peripherals);
                            uart::send(&mut peripherals, b"\r\noutput byte: 0b");
                            for i in 0..8 {
                                if output_byte & (1 << (7 - i)) == 0 {
                                    uart::send(&mut peripherals, b"0");
                                } else {
                                    uart::send(&mut peripherals, b"1");
                                }
                            }
                            uart::send(&mut peripherals, b" == 0x");
                            uart::send_iter(&mut peripherals, byte_to_hex(output_byte));
                            if output_byte >= 0x20 && output_byte <= 0x7E {
                                let mut buf: [u8; 7] = *b" == ' '";
                                buf[5] = output_byte;
                                uart::send(&mut peripherals, &buf);
                            }

                            uart::send(&mut peripherals, b"\r\n");
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
                    0x08|0x7F => {
                        // backspace/delete
                        if cmdline_buffer.len() > 0 {
                            cmdline_buffer.pop();
                            // wipe out the previous character by replacing it with a space and going back
                            uart::send(&mut peripherals, b"\x08 \x08");
                        }
                    },
                    0x0D => unreachable!(), // handled above
                    0x09|0x20 => {
                        // tab or space; silently ignore it
                    },
                    0x21..=0x7E => {
                        // printable character
                        let stored = cmdline_buffer.push(b);
                        if !stored {
                            // we've run out of space
                            uart::send(&mut peripherals, b"\r\ndude, I'm a microcontroller, max command line length is 128");
                            uart::send(&mut peripherals, b"\r\nsince you might have inadvertently pasted something, I'll forget the current\r\ncommand line and I'll be ignoring anything you type until the next\r\ncarriage return (Enter)");
                            cmdline_buffer.clear();
                            ignoring_until_enter = true;
                        }
                        // echo back
                        uart::send(&mut peripherals, &[b]);
                    },
                    other => {
                        let hexed = byte_to_hex(other);
                        uart::send(&mut peripherals, b"\r\ndude, I'm a microcontroller, I can't deal with weird control characters like 0x");
                        uart::send(&mut peripherals, &hexed);
                        uart::send(&mut peripherals, b"\r\nsince you might have inadvertently pasted something, I'll forget the current\r\ncommand line and I'll be ignoring anything you type until the next\r\ncarriage return (Enter)");
                        cmdline_buffer.clear();
                        ignoring_until_enter = true;
                    },
                }
            }
        }

        // anything to do with the FPGA?
        let my_turn = board_pin!(read_pin, &mut peripherals, PA, 7);
        if my_turn {
            match state {
                CrackState::Unknown => {
                    // assume we are now waiting for hash input
                    state = CrackState::AwaitingHashInput;
                },
                CrackState::SendingHash => {
                    // set new_hash_byte
                    // note: the FPGA expects it right-to-left
                    set_new_hash_byte(&mut peripherals, hash_buffer[hash_buffer.len() - current_index - 1]);
                    current_index += 1;

                    // give it a sec
                    sleepiness();

                    // pull "store_hash_byte" high
                    board_pin!(set_high, &mut peripherals, PB, 8);

                    // give it a sec
                    sleepiness();

                    // pull "store_hash_byte" low
                    board_pin!(set_low, &mut peripherals, PB, 8);

                    if current_index == HASH_LENGTH {
                        // full hash transmitted; we're idle again
                        current_index = 0;
                        state = CrackState::AwaitingHashInput;
                    }
                },
                CrackState::PasswordOutput => {
                    // read password_byte
                    let password_byte = get_output_byte(&mut peripherals);
                    password_buffer[current_index] = password_byte;
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
                    board_pin!(set_high, &mut peripherals, PB, 9);

                    // give it a sec
                    sleepiness();

                    // pull "go" low
                    board_pin!(set_low, &mut peripherals, PB, 9);
                },
                CrackState::Cracking => {
                    // ooh, a state change
                    let match_found = board_pin!(read_pin, &mut peripherals, PA, 6);
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
    if !uart::has_received_byte(&mut peripherals) {
        return;
    }

    let b = uart::read_byte(&mut peripherals);
    unsafe { UART_BUFFER.push(b) };
}

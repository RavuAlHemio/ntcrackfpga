#![no_std]
#![no_main]


mod byte_buffer;


use core::panic::PanicInfo;

use ardzero::board_pin;
use atsamd21g18a::{interrupt, Peripherals};
use cortex_m::asm::nop;
use cortex_m_rt::entry;

use crate::byte_buffer::ByteBuffer;


static mut UART_BUFFER: ByteBuffer<64> = ByteBuffer::new();
static mut UART_FILLED: bool = false;
static mut UART_DROPPING: bool = false;


#[panic_handler]
fn panic_handler(_why: &PanicInfo) -> ! {
    const NOP_COUNT: usize = 16;

    // set A17 (built-in LED) to output
    let mut peripherals = unsafe { Peripherals::steal() };
    board_pin!(set_io, &mut peripherals, PA, 17);
    board_pin!(make_output, &mut peripherals, PA, 17);

    loop {
        // make blink

        board_pin!(set_high, &mut peripherals, PA, 17);

        for _ in 0..NOP_COUNT {
            nop();
        }

        board_pin!(set_low, &mut peripherals, PA, 17);

        for _ in 0..NOP_COUNT {
            nop();
        }
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

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take()
        .expect("failed to obtain peripherals");

    // set up chip
    ardzero::init::initialize(&mut peripherals);

    board_pin!(set_io, &mut peripherals, PA, 17);
    board_pin!(make_output, &mut peripherals, PA, 17);
    board_pin!(set_high, &mut peripherals, PA, 17);

    // set up UART for communicating with host, including interrupt
    ardzero::uart::set_up(&mut peripherals);
    ardzero::uart::enable_receive_interrupt(&mut peripherals);

    // set up I2C for communicating with port expander (Mikroe Expand 12 Click/MAX7300)
    ardzero::i2c::set_up_host(&mut peripherals);

    loop {
        // wait for an interrupt
        cortex_m::asm::wfi();

        // debug: readback and hex-dump the UART bytes
        let mut buf_hex: ByteBuffer<128> = ByteBuffer::new();
        let buf_clone = unsafe {
            let c = UART_BUFFER.clone();
            UART_BUFFER.clear();
            c
        };
        for b in buf_clone.as_slice() {
            let hex = byte_to_hex(*b);
            buf_hex.push(hex[0]);
            buf_hex.push(hex[1]);
        }
        ardzero::uart::send(&mut peripherals, buf_hex.as_slice());
    }
}

/// UART interrupt handler
#[interrupt]
fn SERCOM0() {
    let mut peripherals = unsafe { Peripherals::steal() };
    board_pin!(set_low, peripherals, PA, 17);
    if !ardzero::uart::has_received_byte(&mut peripherals) {
        return;
    }

    let b = ardzero::uart::read_byte(&mut peripherals);

    let is_full = unsafe { UART_FILLED || UART_DROPPING };
    if !is_full {
        let added = unsafe { UART_BUFFER.push(b) };
        if !added {
            unsafe { UART_FILLED = true };
        }
    }
}

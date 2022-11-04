//! UART (Universal Asynchronous Receiver/Transmitter) for communication with a host computer.


use atsamd21g18a::{interrupt, Peripherals};

use crate::board_pin;
use crate::pin::PeripheralIndex;


/// Performs the necessary setup to configure the UART (D0=RX, D1=TX) with 115200 b/s 8N1.
///
/// (115200 bits per second, 8 bits per byte, no parity bit, 1 stop bit)
///
/// | Arduino pin | SAMD21 pin | peripheral usage   | direction |
/// | ----------- | ---------- | ------------------ | --------- |
/// | D0          | PA11       | SERCOM0/PAD[3] (C) | RX        |
/// | D1          | PA10       | SERCOM0/PAD[2] (C) | TX        |
pub fn set_up(peripherals: &mut Peripherals) {
    // enable SERCOM0 core clock (use GCLK0 = 48MHz)
    peripherals.GCLK.clkctrl.write(|w| w
        .id().sercom0_core()
        .gen().gclk0()
        .clken().set_bit()
    );
    while peripherals.GCLK.status.read().syncbusy().bit_is_set() {
    }

    // enable SERCOM0 bus clock
    peripherals.PM.apbcmask.modify(|_, w| w
        .sercom0_().set_bit()
    );

    // hand over pins to SERCOM0
    board_pin!(set_peripheral, peripherals, PA, 10);
    board_pin!(set_peripheral, peripherals, PA, 11);
    board_pin!(select_peripheral, peripherals, PA, 10, PeripheralIndex::C);
    board_pin!(select_peripheral, peripherals, PA, 11, PeripheralIndex::C);

    // some of the following operations require synchronization
    // (i.e. waiting for the peripheral to accept the changes)
    // this is annotated in square brackets

    // reset SERCOM0
    let usart0 = peripherals.SERCOM0.usart();
    usart0.ctrla.write(|w| w
        .swrst().set_bit() // software reset [synchronized]
    );
    while usart0.ctrla.read().swrst().bit_is_set() || usart0.syncbusy.read().swrst().bit_is_set() {
    }

    // basic USART setup
    usart0.ctrla.write(|w| w
        .mode().usart_int_clk() // set mode to USART [no sync]
        .cmode().clear_bit() // asynchronous communication [no sync]
        .rxpo().variant(3) // receive data on pad 3 (PA11) [no sync]
        .txpo().variant(1) // transmit data on pad 2 (PA10) [no sync]
        .dord().set_bit() // LSB-first (specified in RS232) [no sync]
        .form().variant(0) // USART frames without parity [no sync]
        .enable().set_bit() // enable the USART [synchronized]
    );
    while usart0.ctrla.read().enable().bit_is_clear() || usart0.syncbusy.read().enable().bit_is_set() {
    }

    usart0.ctrlb.write(|w| w
        .chsize().variant(0) // 8 bits per byte [no sync]
        .sbmode().clear_bit() // one stop bit [no sync]
    );

    // set baud rate
    // according to datasheet table 25-2 in § 25.6.2.3 (we are operating in async arithmetic)
    // baud_register = 65_536 * (1 - S * f_BAUD/f_ref)
    //               = 65_536 * (1 - 8 * 115_200 / 48_000_000)
    //               = 64_277.7088
    usart0.baud().write(|w| w
        .baud().variant(64_278)
    );

    // enable read interrupt
    usart0.intenset.write(|w| w
        .rxc().set_bit()
    );

    // start USART
    usart0.ctrlb.write(|w| w
        .rxen().set_bit() // enable Rx [synchronized]
        .txen().set_bit() // enable Tx [synchronized]
    );
    while usart0.ctrlb.read().rxen().bit_is_clear() || usart0.ctrlb.read().txen().bit_is_clear() || usart0.syncbusy.read().ctrlb().bit_is_set() {
    }
}

pub fn send(peripherals: &mut Peripherals, data: &[u8]) {
    let usart0 = &peripherals.SERCOM0.usart();

    for b in data {
        // wait for buffer to empty
        while usart0.intflag.read().dre().bit_is_clear() {
        }

        usart0.data.write(|w| w.data().variant(*b as u16));
    }

    // wait for buffer to empty and transmission to finish
    while usart0.intflag.read().dre().bit_is_clear() || usart0.intflag.read().txc().bit_is_clear() {
    }
}

#[interrupt]
fn SERCOM0() {
    // TODO: read data register and append to some buffer
    todo!();
}

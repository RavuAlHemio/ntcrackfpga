//! UART (Universal Asynchronous Receiver/Transmitter) for communication with a host computer.


use atsaml21g18b::{Interrupt, Peripherals};

use crate::board_pin;
use crate::pin::PeripheralIndex;


/// Performs the necessary setup to configure the UART (D0=RX, D1=TX) with 115200 b/s 8N1.
///
/// The UART is configured on SERCOM0 since that matches the pinout of the Versa board.
///
/// (115200 bits per second, 8 bits per byte, no parity bit, 1 stop bit)
///
/// | SAML21 pin | peripheral usage   | direction |
/// | ---------- | ------------------ | --------- |
/// | PA04       | SERCOM0/PAD[0] (D) | TX        |
/// | PA05       | SERCOM0/PAD[1] (D) | RX        |
pub fn set_up(peripherals: &mut Peripherals) {
    // enable SERCOM0 core clock (use GCLK0 = 48MHz)
    // index taken from datasheet § 17.10.4 (table 17-9)
    const PCHCTRL_SERCOM0_CORE: usize = 18;
    peripherals.GCLK.pchctrl[PCHCTRL_SERCOM0_CORE].write(|w| w
        .gen().gclk0()
        .chen().set_bit()
    );
    // readback synchronization (datasheet § 17.6.3)
    while peripherals.GCLK.pchctrl[PCHCTRL_SERCOM0_CORE].read().chen().bit_is_clear() {
    }

    // slow clock is not needed for UART
    // (only for I2C controller [formerly master], if I recall correctly)

    // enable SERCOM0 bus clock
    peripherals.MCLK.apbcmask.modify(|_, w| w
        .sercom0_().set_bit()
    );

    // hand over pins to SERCOM0
    board_pin!(set_peripheral, peripherals, PA, 4, 5);
    board_pin!(select_peripheral, peripherals, PeripheralIndex::D, PA, 4, 5);

    // some of the following operations require synchronization
    // (i.e. waiting for the peripheral to accept the changes)
    // this is annotated in square brackets

    // reset SERCOM0
    let usart0 = peripherals.SERCOM0.usart();
    usart0.ctrla.modify(|_, w| w
        .swrst().set_bit() // software reset [synchronized]
    );
    while usart0.ctrla.read().swrst().bit_is_set() || usart0.syncbusy.read().swrst().bit_is_set() {
    }

    // basic USART setup
    usart0.ctrla.modify(|_, w| w
        .mode().variant(0x1) // set mode to USART with internal clock [no sync]
        .cmode().clear_bit() // asynchronous communication [no sync]
        .rxpo().variant(1) // receive data on pad 1 (PA05) [no sync]
        .txpo().variant(0) // transmit data on pad 0 (PA04) [no sync]
        .dord().set_bit() // LSB-first (specified in RS232) [no sync]
        .form().variant(0) // USART frames without parity [no sync]
        .sampr().variant(0) // 16x oversampling, arithmetic baud rate [no sync]
    );
    usart0.ctrlb.modify(|_, w| w
        .chsize().variant(0) // 8 bits per byte [no sync]
        .sbmode().clear_bit() // one stop bit [no sync]
    );

    // set baud rate
    // according to datasheet table 25-2 in § 25.6.2.3 (we are operating in async arithmetic)
    // baud_register = 65_536 * (1 - S * f_BAUD/f_ref)
    //               = 65_536 * (1 - 16 * 115_200 / 48_000_000)
    //               = 63_019.4176
    usart0.baud().modify(|_, w| w
        .baud().variant(63_019)
    );

    // start it up
    usart0.ctrla.modify(|_, w| w
        .enable().set_bit() // enable the USART [synchronized]
    );
    while usart0.ctrla.read().enable().bit_is_clear() || usart0.syncbusy.read().enable().bit_is_set() {
    }

    // enable Tx and Rx
    usart0.ctrlb.modify(|_, w| w
        .rxen().set_bit() // enable Rx [synchronized]
        .txen().set_bit() // enable Tx [synchronized]
    );
    while usart0.ctrlb.read().txen().bit_is_clear() || usart0.ctrlb.read().rxen().bit_is_clear() || usart0.syncbusy.read().ctrlb().bit_is_set() {
    }
}

pub fn send(peripherals: &mut Peripherals, slice: &[u8]) {
    send_iter(peripherals, slice.into_iter().map(|b| *b))
}

pub fn send_iter<I: IntoIterator<Item = u8>>(peripherals: &mut Peripherals, data: I) {
    let mut peekable_data = data.into_iter().peekable();
    if peekable_data.peek().is_none() {
        // empty iterator; don't bother
        return;
    }

    let usart0 = peripherals.SERCOM0.usart();

    for b in peekable_data {
        // wait for buffer to empty
        while usart0.intflag.read().dre().bit_is_clear() {
        }

        usart0.data.write(|w| w.data().variant(b as u16));
    }

    // wait for buffer to empty and transmission to finish
    while usart0.intflag.read().dre().bit_is_clear() || usart0.intflag.read().txc().bit_is_clear() {
    }
}

pub fn read_byte(peripherals: &mut Peripherals) -> u8 {
    let usart0 = peripherals.SERCOM0.usart();
    (usart0.data.read().data().bits() & 0xFF) as u8
}

/// Enables the SERCOM0 interrupt to be raised when a byte is received.
#[inline]
pub fn enable_receive_interrupt(peripherals: &mut Peripherals) {
    unsafe { cortex_m::peripheral::NVIC::unmask(Interrupt::SERCOM0) };

    // enable read interrupt
    let usart0 = peripherals.SERCOM0.usart();
    usart0.intenset.modify(|_, w| w
        .rxc().set_bit()
    );
}

/// Returns whether the SERCOM0 interrupt was raised because a byte was received.
#[inline]
pub fn has_received_byte(peripherals: &mut Peripherals) -> bool {
    let usart0 = peripherals.SERCOM0.usart();
    usart0.intflag.read().rxc().bit_is_set()
}

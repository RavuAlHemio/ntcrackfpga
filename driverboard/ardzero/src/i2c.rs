//! I<sup>2</sup>C (Inter-Integrated Circuit) interface.


use atsamd21g18a::{Interrupt, Peripherals};

use crate::board_pin;
use crate::pin::PeripheralIndex;


/// Performs the necessary setup to configure the I<sup>2</sup>C (D20=SDA, D21=SCL) in host (master)
/// mode.
///
/// The I<sup>2</sup>C host is set up on SERCOM3 since that matches the pinout of the Zero board.
///
/// | Arduino pin | SAMD21 pin | peripheral usage   | signal |
/// | ----------- | ---------- | ------------------ | ------ |
/// | D20         | PA22       | SERCOM3/PAD[0] (C) | SDA    |
/// | D21         | PA23       | SERCOM3/PAD[1] (C) | SCL    |
pub fn set_up_host(peripherals: &mut Peripherals) {
    // enable SERCOM3 core clock (use GCLK0 = 48MHz)
    peripherals.GCLK.clkctrl.write(|w| w
        .id().sercom3_core()
        .gen().gclk0()
        .clken().set_bit()
    );
    while peripherals.GCLK.status.read().syncbusy().bit_is_set() {
    }

    // enable SERCOM3 bus clock
    peripherals.PM.apbcmask.modify(|_, w| w
        .sercom3_().set_bit()
    );

    // hand over pins to SERCOM3
    board_pin!(set_peripheral, peripherals, PA, 22);
    board_pin!(set_peripheral, peripherals, PA, 23);
    board_pin!(select_peripheral, peripherals, PA, 22, PeripheralIndex::C);
    board_pin!(select_peripheral, peripherals, PA, 23, PeripheralIndex::C);

    // some of the following operations require synchronization
    // (i.e. waiting for the peripheral to accept the changes)
    // this is annotated in square brackets

    // reset SERCOM3
    let i2ch3 = peripherals.SERCOM3.i2cm();
    i2ch3.ctrla.modify(|_, w| w
        .swrst().set_bit() // software reset [synchronized]
    );
    while i2ch3.ctrla.read().swrst().bit_is_set() || i2ch3.syncbusy.read().swrst().bit_is_set() {
    }

    // basic I2C host setup
    const SDA_HOLD_TIME_DISABLED: u8 = 0;
    const SPEED_STANDARD_MODE: u8 = 0;
    i2ch3.ctrla.modify(|_, w| w
        .mode().i2c_master() // set mode to I2C host [no sync]
        .pinout().clear_bit() // two-wire (not four-wire) operation [no sync]
        .sdahold().variant(SDA_HOLD_TIME_DISABLED) // SDA hold time disabled [no sync]
        .speed().variant(SPEED_STANDARD_MODE) // standard mode speed [no sync]
    );

    // set baud rate to 100_000 [no sync]
    // according to datasheet table 25-2 in § 25.6.2.3 (we are operating in sync mode)
    // baud_register = (f_ref / (2 * f_BAUD)) - 1
    //               = (48_000_000 / (2 * 100_000)) - 1
    //               = 239
    i2ch3.baud.modify(|_, w| w
        .baud().variant(239)
    );

    // start the I2C host
    i2ch3.ctrla.modify(|_, w| w.enable().set_bit());
    while i2ch3.ctrla.read().enable().bit_is_clear() || i2ch3.syncbusy.read().enable().bit_is_set() {
    }

    // assert the bus
    const BUS_STATE_IDLE: u8 = 1;
    i2ch3.status.modify(|_, w| w.busstate().variant(BUS_STATE_IDLE));
    while i2ch3.syncbusy.read().sysop().bit_is_set() {
    }
}

/// Write data to an I2C client.
pub fn write(peripherals: &mut Peripherals, target_address: u8, data: &[u8]) {
    assert_eq!(target_address & 0b1000_0000, 0);

    let i2ch3 = peripherals.SERCOM3.i2cm();

    let address_and_read = (target_address << 1) | 0b0;

    i2ch3.addr.modify(|_, w| w
        .addr().variant(address_and_read.into())
    );
    while i2ch3.syncbusy.read().sysop().bit_is_set() {
    }
    while i2ch3.intflag.read().mb().bit_is_clear() {
    }

    const I2C_CMD_STOP: u8 = 0x3;
    for b in data {
        i2ch3.data.write(|w| w.data().variant(*b));
        while i2ch3.syncbusy.read().sysop().bit_is_set() {
        }
        while i2ch3.intflag.read().mb().bit_is_clear() {
        }
        if i2ch3.status.read().rxnack().bit_is_set() {
            // free up the bus (send stop condition) before we panic
            i2ch3.ctrlb.modify(|_, w| w.cmd().variant(I2C_CMD_STOP));
            while i2ch3.syncbusy.read().sysop().bit_is_set() {
            }

            panic!("I2C NAK");
        }
    }

    // all bytes sent; send stop condition
    i2ch3.ctrlb.modify(|_, w| w.cmd().variant(I2C_CMD_STOP));
    while i2ch3.syncbusy.read().sysop().bit_is_set() {
    }
}

/// Read data from an I2C client.
pub fn read<F: FnMut(u8) -> bool>(peripherals: &mut Peripherals, target_address: u8, mut process_byte: F) {
    assert_eq!(target_address & 0b1000_0000, 0);

    let i2ch3 = peripherals.SERCOM3.i2cm();

    let address_and_read = (target_address << 1) | 0b1;

    i2ch3.addr.modify(|_, w| w
        .addr().variant(address_and_read.into())
    );
    while i2ch3.syncbusy.read().sysop().bit_is_set() {
    }
    while i2ch3.intflag.read().mb().bit_is_clear() {
    }

    const I2C_CMD_READ_OK: u8 = 0x2;
    const I2C_CMD_STOP: u8 = 0x3;
    loop {
        // wait for next byte to appear
        while i2ch3.intflag.read().sb().bit_is_clear() {
        }

        // read it
        let read_byte = i2ch3.data.read().data().bits();

        // process it
        let keep_reading = process_byte(read_byte);

        if keep_reading {
            i2ch3.ctrlb.modify(|_, w| w.cmd().variant(I2C_CMD_READ_OK));
            while i2ch3.syncbusy.read().sysop().bit_is_set() {
            }
        } else {
            break;
        }
    }

    // free up the bus
    i2ch3.ctrlb.modify(|_, w| w.cmd().variant(I2C_CMD_STOP));
    while i2ch3.syncbusy.read().sysop().bit_is_set() {
    }
}

/// Enables the triggering of the SERCOM3 interrupt when a byte is received.
pub fn enable_receive_interrupt(peripherals: &mut Peripherals) {
    unsafe { cortex_m::peripheral::NVIC::unmask(Interrupt::SERCOM3) };

    let i2ch3 = peripherals.SERCOM3.i2cm();
    i2ch3.intenset.write(|w| w
        .sb().set_bit()
    );
}

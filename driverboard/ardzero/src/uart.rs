//! UART (Universal Asynchronous Receiver/Transmitter) for communication with a host computer.


use atsamd21g18a::Peripherals;

use crate::board_pin;
use crate::pin::PeripheralIndex;


/// Performs the necessary setup to configure the UART (D0=PA11=RX, D1=PA10=TX) with 115200 b/s 8N1.
///
/// (115200 bits per second, 8 bits per byte, no parity bit, 1 stop bit)
pub fn set_up(peripherals: &mut Peripherals) {
    // enable SERCOM0 core clock
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

    // TODO: USART configuration according to datasheet § 26.6.2.1
}

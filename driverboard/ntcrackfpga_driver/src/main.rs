#![no_std]
#![no_main]


use core::panic::PanicInfo;

use ardzero::board_pin;
use atsamd21g18a::Peripherals;


#[panic_handler]
fn panic_handler(_why: &PanicInfo) -> ! {
    // set A17 (built-in LED) to output
    let mut peripherals = unsafe { Peripherals::steal() };
    board_pin!(set_io, &mut peripherals, PA, 17);

    loop {
        // make blink
        board_pin!(set_high, &mut peripherals, PA, 17);
    }
}

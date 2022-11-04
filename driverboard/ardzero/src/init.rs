//! Initialization routines.


use atsamd21g18a::Peripherals;

use crate::calibration::{adc_bias_cal, adc_linearity_cal};


/// This sets up the relevant clocks.
///
/// The following steps are taken:
///
/// 1. Set up and enable XOSC32K (32_768Hz external crystal) in crystal mode.
///
/// 2. Set XOSC32K as source for GCG1 (Generic Clock Generator 1).
///
/// 3. Set up and enable DFLL48M (with GCG1 as source).
///
/// 4. Set DFLL48M as source for GCG0.
///
/// 5. Set OSCxM prescaler to 8MHz.
///
/// 6. Set OSC8M as source for GCG3.
fn clock_setup(peripherals: &mut Peripherals) {
    // step 1

    // set up XOSC32K
    peripherals.SYSCTRL.xosc32k.modify(|_, w| w
        .startup().variant(0x7) // longest startup delay, just in case
        .en32k().set_bit() // enable 32kHz output
        .xtalen().set_bit() // what's connected is a crystal, not a clock
    );

    // enable it separately
    // (datasheet § 17.6.3: "Writing to the XOSC32K.ENABLE bit while writing to other bits may
    // result in unpredictable behavior.")
    peripherals.SYSCTRL.xosc32k.modify(|_, w| w
        .enable().set_bit()
    );

    // wait until XOSC32K has started
    while peripherals.SYSCTRL.pclksr.read().xosc32krdy().bit_is_clear() {
    }

    // accesses to GCLK must be synchronized (two different clocks);
    // this is the reason for all the syncbusy loops

    // step 2

    // reset the generic clock controller, just in case
    peripherals.GCLK.ctrl.modify(|_, w| w
        .swrst().set_bit()
    );
    while peripherals.GCLK.ctrl.read().swrst().bit_is_set() && peripherals.GCLK.status.read().syncbusy().bit_is_set() {
    }

    // GCG1 will be the raw clock divided by 1
    peripherals.GCLK.gendiv.modify(|_, w| w
        .id().variant(1) // target GCG1
        .div().variant(1) // divisor = 1
    );
    while peripherals.GCLK.status.read().syncbusy().bit_is_set() {
    }

    // setup GCG1
    peripherals.GCLK.genctrl.modify(|_, w| w
        .id().variant(1) // target GCG1
        .src().xosc32k() // take clock from XOSC32K
        .idc().set_bit() // improve duty cycle
        .genen().set_bit() // enable this generator
    );
    while peripherals.GCLK.status.read().syncbusy().bit_is_set() {
    }

    // step 3

    // set GCG1 as source of DFLL48M (Digital Frequency Locked Loop, 48 MHz)
    peripherals.GCLK.clkctrl.modify(|_, w| w
        .id().dfll48() // target DFLL48M
        .gen().gclk1() // set GCG1 as source
        .clken().set_bit() // enable this clock
    );
    while peripherals.GCLK.status.read().syncbusy().bit_is_set() {
    }

    // closed-loop config for DFLL48M -- here we also have to wait for synchronization each time

    // work around erratum 1.2.1
    peripherals.SYSCTRL.dfllctrl.modify(|_, w| w
        .ondemand().clear_bit()
    );
    while peripherals.SYSCTRL.pclksr.read().dfllrdy().bit_is_clear() {
    }

    // set the max adjustment steps during closed-loop adjustments to half of each max value
    const MULTIPLIER: u32 = 48_000_000 / 32_768;
    peripherals.SYSCTRL.dfllmul.modify(|_, w| w
        .cstep().variant(31) // max coarse step = 31
        .fstep().variant(511) // max fine step = 511
        .mul().variant(MULTIPLIER.try_into().unwrap()) // 48MHz CPU frequency / 32.768kHz oscillator frequency
    );
    while peripherals.SYSCTRL.pclksr.read().dfllrdy().bit_is_clear() {
    }

    // set up DFLL48M
    peripherals.SYSCTRL.dfllctrl.modify(|_, w| w
        .mode().set_bit() // closed-loop mode
        .waitlock().set_bit() // wait for frequency lock before broadcasting clock
        .qldis().set_bit() // disable quick-lock
    );
    while peripherals.SYSCTRL.pclksr.read().dfllrdy().bit_is_clear() {
    }

    // enable it
    peripherals.SYSCTRL.dfllctrl.modify(|_, w| w
        .enable().set_bit()
    );

    // wait for frequency to stabilize
    let pclksr = &peripherals.SYSCTRL.pclksr;
    while pclksr.read().dflllckc().bit_is_clear() || pclksr.read().dflllckf().bit_is_clear() {
    }

    // wait for synchronization
    while pclksr.read().dfllrdy().bit_is_clear() {
    }

    // step 4

    peripherals.GCLK.genctrl.modify(|_, w| w
        .id().variant(0) // target GCG0
        .src().dfll48m() // take clock from DFLL48M
        .idc().set_bit() // improve duty cycle
        .genen().set_bit() // enable this generator
    );
    while peripherals.GCLK.status.read().syncbusy().bit_is_set() {
    }

    // step 5

    peripherals.SYSCTRL.osc8m.modify(|_, w| w
        .presc()._0() // prescaler factor 1
        .ondemand().clear_bit() // always run this clock
    );

    // step 6
    peripherals.GCLK.genctrl.modify(|_, w| w
        .id().variant(3) // target GCG3
        .src().osc8m() // take clock from OSC8M
        .genen().set_bit() // enable this generator
    );
    while peripherals.GCLK.status.read().syncbusy().bit_is_set() {
    }

    // set bus clocks
    peripherals.PM.cpusel.write(|w| w.cpudiv().div1());
    peripherals.PM.apbasel.write(|w| w.apbadiv().div1());
    peripherals.PM.apbbsel.write(|w| w.apbbdiv().div1());
    peripherals.PM.apbcsel.write(|w| w.apbcdiv().div1());
}

fn adc_calibration(peripherals: &mut Peripherals) {
    let bias = adc_bias_cal();
    let linearity = adc_linearity_cal();
    peripherals.ADC.calib.modify(|_, w| w
        .bias_cal().variant(bias)
        .linearity_cal().variant(linearity)
    );
}


/// Performs the necessary initialization steps for this board.
pub fn initialize(peripherals: &mut Peripherals) {
    use atsamd21g18a::nvmctrl::ctrlb::RWSSELECT_A;

    // slow down flash access according to datasheet table 37-42 (section 37.12)
    // 48MHz @ 3.3V = 1 wait state (described as HALF in the .svd)
    peripherals.NVMCTRL.ctrlb.modify(|_, w| w
        .rws().variant(RWSSELECT_A::HALF)
    );

    clock_setup(peripherals);
    adc_calibration(peripherals);

    // disable automatic NVM writes
    peripherals.NVMCTRL.ctrlb.modify(|_, w| w
        .manw().set_bit()
    );
}

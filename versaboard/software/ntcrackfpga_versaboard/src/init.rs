//! Initialization routines.


use atsaml21g18b::Peripherals;

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
/// 4. Set DFLL48M as source for GCG0 (= CPU clock).
///
/// In contrast to Arduino, GCG3 is not set up (from OSCxM); it is apparently only used for I2S
/// anyway.
fn clock_setup(peripherals: &mut Peripherals) {
    // step 1

    // set up XOSC32K
    peripherals.OSC32KCTRL.xosc32k.modify(|_, w| w
        .startup().variant(0x6) // longest startup delay, just in case
        .en32k().set_bit() // enable 32kHz output
        .xtalen().set_bit() // what's connected is a crystal, not a clock
    );

    // enable it separately
    // (SAM D21/DA1 datasheet § 17.6.3: "Writing to the XOSC32K.ENABLE bit while writing to other
    // bits may result in unpredictable behavior."; this may not be true for the L21, but better
    // safe than sorry)
    peripherals.OSC32KCTRL.xosc32k.modify(|_, w| w
        .enable().set_bit()
    );

    // wait until XOSC32K has started
    while peripherals.OSC32KCTRL.status.read().xosc32krdy().bit_is_clear() {
    }

    // accesses to GCLK must be synchronized (two different clocks);
    // this is the reason for all the syncbusy loops

    // step 2

    // reset the generic clock controller, just in case
    peripherals.GCLK.ctrla.modify(|_, w| w
        .swrst().set_bit()
    );
    while peripherals.GCLK.ctrla.read().swrst().bit_is_set() && peripherals.GCLK.syncbusy.read().swrst().bit_is_set() {
    }

    // setup GCG1
    peripherals.GCLK.genctrl[1].modify(|_, w| w
        .src().xosc32k() // take clock from XOSC32K
        .idc().set_bit() // improve duty cycle
        .divsel().clear_bit() // divide raw clock by n, not 2**n
        .div().variant(1) // raw clock divided by 1
        .genen().set_bit() // enable this generator
    );
    while peripherals.GCLK.syncbusy.read().genctrl1().bit_is_set() {
    }

    // step 3

    // set GCG1 as source of DFLL48M (Digital Frequency Locked Loop, 48 MHz)
    const PCHCTRL_DFLL48M_REF: usize = 0;
    peripherals.GCLK.pchctrl[PCHCTRL_DFLL48M_REF].modify(|_, w| w
        .gen().gclk1() // set GCG1 as source
        .chen().set_bit() // enable this peripheral channel
    );
    // readback synchronization (datasheet § 17.6.3)
    while peripherals.GCLK.pchctrl[PCHCTRL_DFLL48M_REF].read().chen().bit_is_clear() {
    }

    // closed-loop config for DFLL48M -- here we also have to wait for synchronization each time

    // work around erratum 1.3.1
    peripherals.OSCCTRL.dfllctrl.modify(|_, w| w
        .ondemand().clear_bit()
    );
    while peripherals.OSCCTRL.status.read().dfllrdy().bit_is_clear() {
    }

    // set the max adjustment steps during closed-loop adjustments to half of each max value
    const MULTIPLIER: u32 = 48_000_000 / 32_768;
    peripherals.OSCCTRL.dfllmul.modify(|_, w| w
        .cstep().variant(31) // max coarse step = 31 (out of 63)
        .fstep().variant(511) // max fine step = 511 (out of 1023)
        .mul().variant(MULTIPLIER.try_into().unwrap()) // 48MHz CPU frequency / 32.768kHz oscillator frequency
    );
    while peripherals.OSCCTRL.status.read().dfllrdy().bit_is_clear() {
    }

    // set up DFLL48M
    peripherals.OSCCTRL.dfllctrl.modify(|_, w| w
        .mode().set_bit() // closed-loop mode
        .waitlock().set_bit() // wait for frequency lock before broadcasting clock
        .qldis().set_bit() // disable quick-lock
    );
    while peripherals.OSCCTRL.status.read().dfllrdy().bit_is_clear() {
    }

    // enable it
    peripherals.OSCCTRL.dfllctrl.modify(|_, w| w
        .enable().set_bit()
    );

    // wait for frequency to stabilize
    let oscstatus = &peripherals.OSCCTRL.status;
    while oscstatus.read().dflllckc().bit_is_clear() || oscstatus.read().dflllckf().bit_is_clear() {
    }

    // wait for synchronization
    while oscstatus.read().dfllrdy().bit_is_clear() {
    }

    // step 4

    peripherals.GCLK.genctrl[0].modify(|_, w| w
        .src().dfll48m() // take clock from DFLL48M
        .idc().set_bit() // improve duty cycle
        .genen().set_bit() // enable this generator
    );
    while peripherals.GCLK.syncbusy.read().genctrl0().bit_is_set() {
    }

    // set bus clocks
    peripherals.MCLK.cpudiv.write(|w| w.cpudiv().div1());
    peripherals.MCLK.lpdiv.write(|w| w.lpdiv().div1());
    peripherals.MCLK.bupdiv.write(|w| w.bupdiv().div1());
}


/// Calibrates the analog-digital converter from the values stored in one-time programmable memory
/// during the manufacturing process.
fn adc_calibration(peripherals: &mut Peripherals) {
    let bias = adc_bias_cal();
    let linearity = adc_linearity_cal();
    peripherals.ADC.calib.modify(|_, w| w
        .biasrefbuf().variant(bias)
        .biascomp().variant(linearity)
    );
}


/// Performs the necessary initialization steps for this board.
pub fn initialize(peripherals: &mut Peripherals) {
    use atsaml21g18b::nvmctrl::ctrlb::RWSSELECT_A;

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

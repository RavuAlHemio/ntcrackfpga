//! Functionality to read calibration data.

const CAL0: *const u64 = 0x0080_6020 as *const u64;
//const CAL1: *const u64 = 0x0080_6028 as *const u64;


/// ADC linearity. See `linearity_cal` in [atsamd21g18a::adc::calib].
#[inline]
pub fn adc_linearity_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 27) & 0xF) as u8
}

/// ADC bias calibration. See `bias_cal` in [atsamd21g18a::adc::calib].
#[inline]
pub fn adc_bias_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 35) & 0x3) as u8
}

/// OSC32K calibration. See `calib` in [atsamd21g18a::sysctrl::osc32k].
#[inline]
pub fn osc32k_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 38) & 0x7F) as u8
}

/// USB TRANSN calibration value. See `transn` in [atsamd21g18a::usb::device::padcal] and
/// [atsamd21g18a::usb::host::padcal].
#[inline]
pub fn usb_transn_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 45) & 0x1F) as u8
}

/// USB TRANSP calibration value. See `transp` in [atsamd21g18a::usb::device::padcal] and
/// [atsamd21g18a::usb::host::padcal].
#[inline]
pub fn usb_transp_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 50) & 0x1F) as u8
}

/// USB TRIM calibration value. See `trim` in [atsamd21g18a::usb::device::padcal] and
/// [atsamd21g18a::usb::host::padcal].
#[inline]
pub fn usb_trim_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 55) & 0x7) as u8
}

/// DFLL48M coarse calibration value. See `coarse` in [atsamd21g18a::sysctrl::dfllval].
#[inline]
pub fn dfll48m_coarse_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 58) & 0x3F) as u8
}

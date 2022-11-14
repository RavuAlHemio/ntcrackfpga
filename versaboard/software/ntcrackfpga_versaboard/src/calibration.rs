//! Functionality to read calibration data.

const CAL0: *const u32 = 0x0080_6020 as *const u32;


/// ADC linearity. See `biasrefbuf` in [atsaml21g18b::adc::calib].
#[inline]
pub fn adc_linearity_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 0) & 0x7) as u8
}

/// ADC bias calibration. See `biascomp` in [atsaml21g18b::adc::calib].
#[inline]
pub fn adc_bias_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 3) & 0x7) as u8
}

/// OSC32K calibration. See `calib` in [atsaml21g18b::osc32kctrl::osc32k].
#[inline]
pub fn osc32k_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 6) & 0x7F) as u8
}

/// USB TRANSN calibration value. See `transn` in [atsaml21g18b::usb::device::padcal] and
/// [atsaml21g18b::usb::host::padcal].
#[inline]
pub fn usb_transn_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 13) & 0x1F) as u8
}

/// USB TRANSP calibration value. See `transp` in [atsaml21g18b::usb::device::padcal] and
/// [atsaml21g18b::usb::host::padcal].
#[inline]
pub fn usb_transp_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 18) & 0x1F) as u8
}

/// USB TRIM calibration value. See `trim` in [atsaml21g18b::usb::device::padcal] and
/// [atsaml21g18b::usb::host::padcal].
#[inline]
pub fn usb_trim_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 23) & 0x7) as u8
}

/// DFLL48M coarse calibration value. See `coarse` in [atsaml21g18b::oscctrl::dfllval].
#[inline]
pub fn dfll48m_coarse_cal() -> u8 {
    let cal0 = unsafe { CAL0.read_volatile() };
    ((cal0 >> 26) & 0x3F) as u8
}

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    #[doc = "0x04 - Control B Clear"]
    pub ctrlbclr: CTRLBCLR,
    #[doc = "0x05 - Control B Set"]
    pub ctrlbset: CTRLBSET,
    _reserved3: [u8; 0x02],
    #[doc = "0x08 - Synchronization Busy"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - Recoverable Fault A Configuration"]
    pub fctrla: FCTRLA,
    #[doc = "0x10 - Recoverable Fault B Configuration"]
    pub fctrlb: FCTRLB,
    #[doc = "0x14 - Waveform Extension Configuration"]
    pub wexctrl: WEXCTRL,
    #[doc = "0x18 - Driver Control"]
    pub drvctrl: DRVCTRL,
    _reserved8: [u8; 0x02],
    #[doc = "0x1e - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved9: [u8; 0x01],
    #[doc = "0x20 - Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x24 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x28 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x2c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x30 - Status"]
    pub status: STATUS,
    _reserved_14_count: [u8; 0x04],
    #[doc = "0x38 - Pattern"]
    pub patt: PATT,
    _reserved16: [u8; 0x02],
    #[doc = "0x3c - Waveform Control"]
    pub wave: WAVE,
    _reserved_17_per: [u8; 0x04],
    _reserved_18_cc: [u8; 0x10],
    _reserved19: [u8; 0x10],
    #[doc = "0x64 - Pattern Buffer"]
    pub pattb: PATTB,
    _reserved20: [u8; 0x02],
    #[doc = "0x68 - Waveform Control Buffer"]
    pub waveb: WAVEB,
    _reserved_21_perb: [u8; 0x04],
    _reserved_22_ccb: [u8; 0x10],
}
impl RegisterBlock {
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith6(&self) -> &COUNT_DITH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH6) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith5(&self) -> &COUNT_DITH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH5) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count_dith4(&self) -> &COUNT_DITH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT_DITH4) }
    }
    #[doc = "0x34 - Count"]
    #[inline(always)]
    pub fn count(&self) -> &COUNT {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const COUNT) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith6(&self) -> &PER_DITH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH6) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith5(&self) -> &PER_DITH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH5) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per_dith4(&self) -> &PER_DITH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER_DITH4) }
    }
    #[doc = "0x40 - Period"]
    #[inline(always)]
    pub fn per(&self) -> &PER {
        unsafe { &*(((self as *const Self) as *const u8).add(64usize) as *const PER) }
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith6(&self) -> &[CC_DITH6; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH6; 4]) }
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith5(&self) -> &[CC_DITH5; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH5; 4]) }
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc_dith4(&self) -> &[CC_DITH4; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC_DITH4; 4]) }
    }
    #[doc = "0x44..0x54 - Compare and Capture"]
    #[inline(always)]
    pub fn cc(&self) -> &[CC; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(68usize) as *const [CC; 4]) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith6(&self) -> &PERB_DITH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB_DITH6) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith5(&self) -> &PERB_DITH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB_DITH5) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb_dith4(&self) -> &PERB_DITH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB_DITH4) }
    }
    #[doc = "0x6c - Period Buffer"]
    #[inline(always)]
    pub fn perb(&self) -> &PERB {
        unsafe { &*(((self as *const Self) as *const u8).add(108usize) as *const PERB) }
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith6(&self) -> &[CCB_DITH6; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB_DITH6; 4]) }
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith5(&self) -> &[CCB_DITH5; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB_DITH5; 4]) }
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb_dith4(&self) -> &[CCB_DITH4; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB_DITH4; 4]) }
    }
    #[doc = "0x70..0x80 - Compare and Capture Buffer"]
    #[inline(always)]
    pub fn ccb(&self) -> &[CCB; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const [CCB; 4]) }
    }
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLBCLR (rw) register accessor: an alias for `Reg<CTRLBCLR_SPEC>`"]
pub type CTRLBCLR = crate::Reg<ctrlbclr::CTRLBCLR_SPEC>;
#[doc = "Control B Clear"]
pub mod ctrlbclr;
#[doc = "CTRLBSET (rw) register accessor: an alias for `Reg<CTRLBSET_SPEC>`"]
pub type CTRLBSET = crate::Reg<ctrlbset::CTRLBSET_SPEC>;
#[doc = "Control B Set"]
pub mod ctrlbset;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy"]
pub mod syncbusy;
#[doc = "FCTRLA (rw) register accessor: an alias for `Reg<FCTRLA_SPEC>`"]
pub type FCTRLA = crate::Reg<fctrla::FCTRLA_SPEC>;
#[doc = "Recoverable Fault A Configuration"]
pub mod fctrla;
#[doc = "FCTRLB (rw) register accessor: an alias for `Reg<FCTRLB_SPEC>`"]
pub type FCTRLB = crate::Reg<fctrlb::FCTRLB_SPEC>;
#[doc = "Recoverable Fault B Configuration"]
pub mod fctrlb;
#[doc = "WEXCTRL (rw) register accessor: an alias for `Reg<WEXCTRL_SPEC>`"]
pub type WEXCTRL = crate::Reg<wexctrl::WEXCTRL_SPEC>;
#[doc = "Waveform Extension Configuration"]
pub mod wexctrl;
#[doc = "DRVCTRL (rw) register accessor: an alias for `Reg<DRVCTRL_SPEC>`"]
pub type DRVCTRL = crate::Reg<drvctrl::DRVCTRL_SPEC>;
#[doc = "Driver Control"]
pub mod drvctrl;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "COUNT (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "Count"]
pub mod count;
#[doc = "COUNT_DITH4 (rw) register accessor: an alias for `Reg<COUNT_DITH4_SPEC>`"]
pub type COUNT_DITH4 = crate::Reg<count_dith4::COUNT_DITH4_SPEC>;
#[doc = "Count"]
pub mod count_dith4;
#[doc = "COUNT_DITH5 (rw) register accessor: an alias for `Reg<COUNT_DITH5_SPEC>`"]
pub type COUNT_DITH5 = crate::Reg<count_dith5::COUNT_DITH5_SPEC>;
#[doc = "Count"]
pub mod count_dith5;
#[doc = "COUNT_DITH6 (rw) register accessor: an alias for `Reg<COUNT_DITH6_SPEC>`"]
pub type COUNT_DITH6 = crate::Reg<count_dith6::COUNT_DITH6_SPEC>;
#[doc = "Count"]
pub mod count_dith6;
#[doc = "PATT (rw) register accessor: an alias for `Reg<PATT_SPEC>`"]
pub type PATT = crate::Reg<patt::PATT_SPEC>;
#[doc = "Pattern"]
pub mod patt;
#[doc = "WAVE (rw) register accessor: an alias for `Reg<WAVE_SPEC>`"]
pub type WAVE = crate::Reg<wave::WAVE_SPEC>;
#[doc = "Waveform Control"]
pub mod wave;
#[doc = "PER (rw) register accessor: an alias for `Reg<PER_SPEC>`"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "Period"]
pub mod per;
#[doc = "PER_DITH4 (rw) register accessor: an alias for `Reg<PER_DITH4_SPEC>`"]
pub type PER_DITH4 = crate::Reg<per_dith4::PER_DITH4_SPEC>;
#[doc = "Period"]
pub mod per_dith4;
#[doc = "PER_DITH5 (rw) register accessor: an alias for `Reg<PER_DITH5_SPEC>`"]
pub type PER_DITH5 = crate::Reg<per_dith5::PER_DITH5_SPEC>;
#[doc = "Period"]
pub mod per_dith5;
#[doc = "PER_DITH6 (rw) register accessor: an alias for `Reg<PER_DITH6_SPEC>`"]
pub type PER_DITH6 = crate::Reg<per_dith6::PER_DITH6_SPEC>;
#[doc = "Period"]
pub mod per_dith6;
#[doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc;
#[doc = "CC_DITH4 (rw) register accessor: an alias for `Reg<CC_DITH4_SPEC>`"]
pub type CC_DITH4 = crate::Reg<cc_dith4::CC_DITH4_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc_dith4;
#[doc = "CC_DITH5 (rw) register accessor: an alias for `Reg<CC_DITH5_SPEC>`"]
pub type CC_DITH5 = crate::Reg<cc_dith5::CC_DITH5_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc_dith5;
#[doc = "CC_DITH6 (rw) register accessor: an alias for `Reg<CC_DITH6_SPEC>`"]
pub type CC_DITH6 = crate::Reg<cc_dith6::CC_DITH6_SPEC>;
#[doc = "Compare and Capture"]
pub mod cc_dith6;
#[doc = "PATTB (rw) register accessor: an alias for `Reg<PATTB_SPEC>`"]
pub type PATTB = crate::Reg<pattb::PATTB_SPEC>;
#[doc = "Pattern Buffer"]
pub mod pattb;
#[doc = "WAVEB (rw) register accessor: an alias for `Reg<WAVEB_SPEC>`"]
pub type WAVEB = crate::Reg<waveb::WAVEB_SPEC>;
#[doc = "Waveform Control Buffer"]
pub mod waveb;
#[doc = "PERB (rw) register accessor: an alias for `Reg<PERB_SPEC>`"]
pub type PERB = crate::Reg<perb::PERB_SPEC>;
#[doc = "Period Buffer"]
pub mod perb;
#[doc = "PERB_DITH4 (rw) register accessor: an alias for `Reg<PERB_DITH4_SPEC>`"]
pub type PERB_DITH4 = crate::Reg<perb_dith4::PERB_DITH4_SPEC>;
#[doc = "Period Buffer"]
pub mod perb_dith4;
#[doc = "PERB_DITH5 (rw) register accessor: an alias for `Reg<PERB_DITH5_SPEC>`"]
pub type PERB_DITH5 = crate::Reg<perb_dith5::PERB_DITH5_SPEC>;
#[doc = "Period Buffer"]
pub mod perb_dith5;
#[doc = "PERB_DITH6 (rw) register accessor: an alias for `Reg<PERB_DITH6_SPEC>`"]
pub type PERB_DITH6 = crate::Reg<perb_dith6::PERB_DITH6_SPEC>;
#[doc = "Period Buffer"]
pub mod perb_dith6;
#[doc = "CCB (rw) register accessor: an alias for `Reg<CCB_SPEC>`"]
pub type CCB = crate::Reg<ccb::CCB_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb;
#[doc = "CCB_DITH4 (rw) register accessor: an alias for `Reg<CCB_DITH4_SPEC>`"]
pub type CCB_DITH4 = crate::Reg<ccb_dith4::CCB_DITH4_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith4;
#[doc = "CCB_DITH5 (rw) register accessor: an alias for `Reg<CCB_DITH5_SPEC>`"]
pub type CCB_DITH5 = crate::Reg<ccb_dith5::CCB_DITH5_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith5;
#[doc = "CCB_DITH6 (rw) register accessor: an alias for `Reg<CCB_DITH6_SPEC>`"]
pub type CCB_DITH6 = crate::Reg<ccb_dith6::CCB_DITH6_SPEC>;
#[doc = "Compare and Capture Buffer"]
pub mod ccb_dith6;

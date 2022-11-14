#[doc = r"Register block"]
#[repr(C)]
pub struct MODE1 {
    #[doc = "0x00 - MODE1 Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - MODE1 Event Control"]
    pub evctrl: EVCTRL,
    #[doc = "0x08 - MODE1 Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x0a - MODE1 Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0c - MODE1 Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x0e - Debug Control"]
    pub dbgctrl: DBGCTRL,
    _reserved6: [u8; 0x01],
    #[doc = "0x10 - MODE1 Synchronization Busy Status"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x14 - Frequency Correction"]
    pub freqcorr: FREQCORR,
    _reserved8: [u8; 0x03],
    #[doc = "0x18 - MODE1 Counter Value"]
    pub count: COUNT,
    _reserved9: [u8; 0x02],
    #[doc = "0x1c - MODE1 Counter Period"]
    pub per: PER,
    _reserved10: [u8; 0x02],
    #[doc = "0x20 - MODE1 Compare n Value"]
    pub comp: [COMP; 2],
    _reserved11: [u8; 0x1c],
    #[doc = "0x40..0x50 - General Purpose"]
    pub gp: [GP; 4],
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "MODE1 Control A"]
pub mod ctrla;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "MODE1 Event Control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "MODE1 Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "MODE1 Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: an alias for `Reg<INTFLAG_SPEC>`"]
pub type INTFLAG = crate::Reg<intflag::INTFLAG_SPEC>;
#[doc = "MODE1 Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "DBGCTRL (rw) register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "MODE1 Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "FREQCORR (rw) register accessor: an alias for `Reg<FREQCORR_SPEC>`"]
pub type FREQCORR = crate::Reg<freqcorr::FREQCORR_SPEC>;
#[doc = "Frequency Correction"]
pub mod freqcorr;
#[doc = "COUNT (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "MODE1 Counter Value"]
pub mod count;
#[doc = "PER (rw) register accessor: an alias for `Reg<PER_SPEC>`"]
pub type PER = crate::Reg<per::PER_SPEC>;
#[doc = "MODE1 Counter Period"]
pub mod per;
#[doc = "COMP (rw) register accessor: an alias for `Reg<COMP_SPEC>`"]
pub type COMP = crate::Reg<comp::COMP_SPEC>;
#[doc = "MODE1 Compare n Value"]
pub mod comp;
#[doc = "GP (rw) register accessor: an alias for `Reg<GP_SPEC>`"]
pub type GP = crate::Reg<gp::GP_SPEC>;
#[doc = "General Purpose"]
pub mod gp;

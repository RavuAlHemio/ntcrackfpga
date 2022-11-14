#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Status"]
    pub status: STATUS,
    _reserved2: [u8; 0x01],
    #[doc = "0x04..0x10 - OPAMP n Control"]
    pub opampctrl: [OPAMPCTRL; 3],
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "OPAMPCTRL (rw) register accessor: an alias for `Reg<OPAMPCTRL_SPEC>`"]
pub type OPAMPCTRL = crate::Reg<opampctrl::OPAMPCTRL_SPEC>;
#[doc = "OPAMP n Control"]
pub mod opampctrl;

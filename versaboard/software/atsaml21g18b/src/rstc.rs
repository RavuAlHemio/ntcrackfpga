#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    pub rcause: RCAUSE,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Backup Exit Source"]
    pub bkupexit: BKUPEXIT,
    _reserved2: [u8; 0x01],
    #[doc = "0x04 - Wakeup Debounce Configuration"]
    pub wkdbconf: WKDBCONF,
    _reserved3: [u8; 0x03],
    #[doc = "0x08 - Wakeup Polarity"]
    pub wkpol: WKPOL,
    _reserved4: [u8; 0x02],
    #[doc = "0x0c - Wakeup Enable"]
    pub wken: WKEN,
    _reserved5: [u8; 0x02],
    #[doc = "0x10 - Wakeup Cause"]
    pub wkcause: WKCAUSE,
}
#[doc = "RCAUSE (r) register accessor: an alias for `Reg<RCAUSE_SPEC>`"]
pub type RCAUSE = crate::Reg<rcause::RCAUSE_SPEC>;
#[doc = "Reset Cause"]
pub mod rcause;
#[doc = "BKUPEXIT (r) register accessor: an alias for `Reg<BKUPEXIT_SPEC>`"]
pub type BKUPEXIT = crate::Reg<bkupexit::BKUPEXIT_SPEC>;
#[doc = "Backup Exit Source"]
pub mod bkupexit;
#[doc = "WKDBCONF (rw) register accessor: an alias for `Reg<WKDBCONF_SPEC>`"]
pub type WKDBCONF = crate::Reg<wkdbconf::WKDBCONF_SPEC>;
#[doc = "Wakeup Debounce Configuration"]
pub mod wkdbconf;
#[doc = "WKPOL (rw) register accessor: an alias for `Reg<WKPOL_SPEC>`"]
pub type WKPOL = crate::Reg<wkpol::WKPOL_SPEC>;
#[doc = "Wakeup Polarity"]
pub mod wkpol;
#[doc = "WKEN (rw) register accessor: an alias for `Reg<WKEN_SPEC>`"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "Wakeup Enable"]
pub mod wken;
#[doc = "WKCAUSE (rw) register accessor: an alias for `Reg<WKCAUSE_SPEC>`"]
pub type WKCAUSE = crate::Reg<wkcause::WKCAUSE_SPEC>;
#[doc = "Wakeup Cause"]
pub mod wkcause;

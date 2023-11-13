#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - SECCTRL"]
    pub secctrl: SECCTRL,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - SCFGB"]
    pub scfgb: SCFGB,
    #[doc = "0x08 - SCFGA"]
    pub scfga: SCFGA,
    #[doc = "0x0c - SCFGR"]
    pub scfgr: SCFGR,
}
#[doc = "SECCTRL (rw) register accessor: an alias for `Reg<SECCTRL_SPEC>`"]
pub type SECCTRL = crate::Reg<secctrl::SECCTRL_SPEC>;
#[doc = "SECCTRL"]
pub mod secctrl;
#[doc = "SCFGB (rw) register accessor: an alias for `Reg<SCFGB_SPEC>`"]
pub type SCFGB = crate::Reg<scfgb::SCFGB_SPEC>;
#[doc = "SCFGB"]
pub mod scfgb;
#[doc = "SCFGA (rw) register accessor: an alias for `Reg<SCFGA_SPEC>`"]
pub type SCFGA = crate::Reg<scfga::SCFGA_SPEC>;
#[doc = "SCFGA"]
pub mod scfga;
#[doc = "SCFGR (rw) register accessor: an alias for `Reg<SCFGR_SPEC>`"]
pub type SCFGR = crate::Reg<scfgr::SCFGR_SPEC>;
#[doc = "SCFGR"]
pub mod scfgr;

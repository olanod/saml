#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Control B"]
    pub ctrlb: CTRLB,
    #[doc = "0x08 - Control C"]
    pub ctrlc: CTRLC,
    _reserved3: [u8; 0x01],
    #[doc = "0x0a - Event Control"]
    pub evctrl: EVCTRL,
    _reserved4: [u8; 0x01],
    #[doc = "0x0c - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0x03],
    #[doc = "0x10 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    _reserved6: [u8; 0x03],
    #[doc = "0x14 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved7: [u8; 0x03],
    #[doc = "0x18 - Status"]
    pub status: STATUS,
    _reserved8: [u8; 0x02],
    #[doc = "0x1c - Address"]
    pub addr: ADDR,
    #[doc = "0x20 - Secure Unlock Register"]
    pub sulck: SULCK,
    #[doc = "0x22 - Non-Secure Unlock Register"]
    pub nsulck: NSULCK,
    #[doc = "0x24 - NVM Parameter"]
    pub param: PARAM,
    _reserved12: [u8; 0x08],
    #[doc = "0x30 - Data Scramble Configuration"]
    pub dscc: DSCC,
    #[doc = "0x34 - Security Control"]
    pub secctrl: SECCTRL,
    #[doc = "0x38 - Secure Boot Configuration"]
    pub scfgb: SCFGB,
    #[doc = "0x3c - Secure Application and Data Configuration"]
    pub scfgad: SCFGAD,
    #[doc = "0x40 - Non-secure Write Enable"]
    pub nonsec: NONSEC,
    #[doc = "0x44 - Non-secure Write Reference Value"]
    pub nschk: NSCHK,
}
#[doc = "CTRLA (w) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB (rw) register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "CTRLC (rw) register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
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
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address"]
pub mod addr;
#[doc = "SULCK (rw) register accessor: an alias for `Reg<SULCK_SPEC>`"]
pub type SULCK = crate::Reg<sulck::SULCK_SPEC>;
#[doc = "Secure Unlock Register"]
pub mod sulck;
#[doc = "NSULCK (rw) register accessor: an alias for `Reg<NSULCK_SPEC>`"]
pub type NSULCK = crate::Reg<nsulck::NSULCK_SPEC>;
#[doc = "Non-Secure Unlock Register"]
pub mod nsulck;
#[doc = "PARAM (rw) register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "NVM Parameter"]
pub mod param;
#[doc = "DSCC (w) register accessor: an alias for `Reg<DSCC_SPEC>`"]
pub type DSCC = crate::Reg<dscc::DSCC_SPEC>;
#[doc = "Data Scramble Configuration"]
pub mod dscc;
#[doc = "SECCTRL (rw) register accessor: an alias for `Reg<SECCTRL_SPEC>`"]
pub type SECCTRL = crate::Reg<secctrl::SECCTRL_SPEC>;
#[doc = "Security Control"]
pub mod secctrl;
#[doc = "SCFGB (rw) register accessor: an alias for `Reg<SCFGB_SPEC>`"]
pub type SCFGB = crate::Reg<scfgb::SCFGB_SPEC>;
#[doc = "Secure Boot Configuration"]
pub mod scfgb;
#[doc = "SCFGAD (rw) register accessor: an alias for `Reg<SCFGAD_SPEC>`"]
pub type SCFGAD = crate::Reg<scfgad::SCFGAD_SPEC>;
#[doc = "Secure Application and Data Configuration"]
pub mod scfgad;
#[doc = "NONSEC (rw) register accessor: an alias for `Reg<NONSEC_SPEC>`"]
pub type NONSEC = crate::Reg<nonsec::NONSEC_SPEC>;
#[doc = "Non-secure Write Enable"]
pub mod nonsec;
#[doc = "NSCHK (rw) register accessor: an alias for `Reg<NSCHK_SPEC>`"]
pub type NSCHK = crate::Reg<nschk::NSCHK_SPEC>;
#[doc = "Non-secure Write Reference Value"]
pub mod nschk;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Sleep Configuration"]
    pub sleepcfg: SLEEPCFG,
    #[doc = "0x02 - Performance Level Configuration"]
    pub plcfg: PLCFG,
    #[doc = "0x03 - Power Configuration"]
    pub pwcfg: PWCFG,
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved6: [u8; 0x01],
    #[doc = "0x08 - Standby Configuration"]
    pub stdbycfg: STDBYCFG,
}
#[doc = "SLEEPCFG (rw) register accessor: an alias for `Reg<SLEEPCFG_SPEC>`"]
pub type SLEEPCFG = crate::Reg<sleepcfg::SLEEPCFG_SPEC>;
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "PLCFG (rw) register accessor: an alias for `Reg<PLCFG_SPEC>`"]
pub type PLCFG = crate::Reg<plcfg::PLCFG_SPEC>;
#[doc = "Performance Level Configuration"]
pub mod plcfg;
#[doc = "PWCFG (rw) register accessor: an alias for `Reg<PWCFG_SPEC>`"]
pub type PWCFG = crate::Reg<pwcfg::PWCFG_SPEC>;
#[doc = "Power Configuration"]
pub mod pwcfg;
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
#[doc = "STDBYCFG (rw) register accessor: an alias for `Reg<STDBYCFG_SPEC>`"]
pub type STDBYCFG = crate::Reg<stdbycfg::STDBYCFG_SPEC>;
#[doc = "Standby Configuration"]
pub mod stdbycfg;

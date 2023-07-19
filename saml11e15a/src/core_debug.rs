#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug Halting Control and Status Register"]
    pub dhcsr: DHCSR,
    #[doc = "0x04 - Debug Core Register Select Register"]
    pub dcrsr: DCRSR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Debug Exception and Monitor Control Register"]
    pub demcr: DEMCR,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Debug Authentication Control Register"]
    pub dauthctrl: DAUTHCTRL,
    #[doc = "0x18 - Debug Security Control and Status Register"]
    pub dscsr: DSCSR,
}
#[doc = "DHCSR (rw) register accessor: an alias for `Reg<DHCSR_SPEC>`"]
pub type DHCSR = crate::Reg<dhcsr::DHCSR_SPEC>;
#[doc = "Debug Halting Control and Status Register"]
pub mod dhcsr;
#[doc = "DCRSR (w) register accessor: an alias for `Reg<DCRSR_SPEC>`"]
pub type DCRSR = crate::Reg<dcrsr::DCRSR_SPEC>;
#[doc = "Debug Core Register Select Register"]
pub mod dcrsr;
#[doc = "DEMCR (rw) register accessor: an alias for `Reg<DEMCR_SPEC>`"]
pub type DEMCR = crate::Reg<demcr::DEMCR_SPEC>;
#[doc = "Debug Exception and Monitor Control Register"]
pub mod demcr;
#[doc = "DAUTHCTRL (rw) register accessor: an alias for `Reg<DAUTHCTRL_SPEC>`"]
pub type DAUTHCTRL = crate::Reg<dauthctrl::DAUTHCTRL_SPEC>;
#[doc = "Debug Authentication Control Register"]
pub mod dauthctrl;
#[doc = "DSCSR (rw) register accessor: an alias for `Reg<DSCSR_SPEC>`"]
pub type DSCSR = crate::Reg<dscsr::DSCSR_SPEC>;
#[doc = "Debug Security Control and Status Register"]
pub mod dscsr;

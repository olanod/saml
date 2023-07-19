#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick Control and Status Register"]
    pub syst_csr: SYST_CSR,
    #[doc = "0x04 - SysTick Reload Value Register"]
    pub syst_rvr: SYST_RVR,
    #[doc = "0x08 - SysTick Current Value Register"]
    pub syst_cvr: SYST_CVR,
    #[doc = "0x0c - SysTick Calibration Value Register"]
    pub syst_calib: SYST_CALIB,
}
#[doc = "SYST_CSR (rw) register accessor: an alias for `Reg<SYST_CSR_SPEC>`"]
pub type SYST_CSR = crate::Reg<syst_csr::SYST_CSR_SPEC>;
#[doc = "SysTick Control and Status Register"]
pub mod syst_csr;
#[doc = "SYST_RVR (rw) register accessor: an alias for `Reg<SYST_RVR_SPEC>`"]
pub type SYST_RVR = crate::Reg<syst_rvr::SYST_RVR_SPEC>;
#[doc = "SysTick Reload Value Register"]
pub mod syst_rvr;
#[doc = "SYST_CVR (rw) register accessor: an alias for `Reg<SYST_CVR_SPEC>`"]
pub type SYST_CVR = crate::Reg<syst_cvr::SYST_CVR_SPEC>;
#[doc = "SysTick Current Value Register"]
pub mod syst_cvr;
#[doc = "SYST_CALIB (r) register accessor: an alias for `Reg<SYST_CALIB_SPEC>`"]
pub type SYST_CALIB = crate::Reg<syst_calib::SYST_CALIB_SPEC>;
#[doc = "SysTick Calibration Value Register"]
pub mod syst_calib;

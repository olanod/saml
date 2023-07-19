#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write control"]
    pub wrctrl: WRCTRL,
    #[doc = "0x04 - Event control"]
    pub evctrl: EVCTRL,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Interrupt enable clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x09 - Interrupt enable set"]
    pub intenset: INTENSET,
    _reserved4: [u8; 0x06],
    #[doc = "0x10 - Bridge interrupt flag status"]
    pub intflagahb: INTFLAGAHB,
    #[doc = "0x14 - Peripheral interrupt flag status - Bridge A"]
    pub intflaga: INTFLAGA,
    #[doc = "0x18 - Peripheral interrupt flag status - Bridge B"]
    pub intflagb: INTFLAGB,
    #[doc = "0x1c - Peripheral interrupt flag status - Bridge C"]
    pub intflagc: INTFLAGC,
    _reserved8: [u8; 0x14],
    #[doc = "0x34 - Peripheral write protection status - Bridge A"]
    pub statusa: STATUSA,
    #[doc = "0x38 - Peripheral write protection status - Bridge B"]
    pub statusb: STATUSB,
    #[doc = "0x3c - Peripheral write protection status - Bridge C"]
    pub statusc: STATUSC,
    _reserved11: [u8; 0x14],
    #[doc = "0x54 - Peripheral non-secure status - Bridge A"]
    pub nonseca: NONSECA,
    #[doc = "0x58 - Peripheral non-secure status - Bridge B"]
    pub nonsecb: NONSECB,
    #[doc = "0x5c - Peripheral non-secure status - Bridge C"]
    pub nonsecc: NONSECC,
    _reserved14: [u8; 0x14],
    #[doc = "0x74 - Peripheral secure status locked - Bridge A"]
    pub seclocka: SECLOCKA,
    #[doc = "0x78 - Peripheral secure status locked - Bridge B"]
    pub seclockb: SECLOCKB,
    #[doc = "0x7c - Peripheral secure status locked - Bridge C"]
    pub seclockc: SECLOCKC,
}
#[doc = "WRCTRL (rw) register accessor: an alias for `Reg<WRCTRL_SPEC>`"]
pub type WRCTRL = crate::Reg<wrctrl::WRCTRL_SPEC>;
#[doc = "Write control"]
pub mod wrctrl;
#[doc = "EVCTRL (rw) register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event control"]
pub mod evctrl;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set"]
pub mod intenset;
#[doc = "INTFLAGAHB (rw) register accessor: an alias for `Reg<INTFLAGAHB_SPEC>`"]
pub type INTFLAGAHB = crate::Reg<intflagahb::INTFLAGAHB_SPEC>;
#[doc = "Bridge interrupt flag status"]
pub mod intflagahb;
#[doc = "INTFLAGA (rw) register accessor: an alias for `Reg<INTFLAGA_SPEC>`"]
pub type INTFLAGA = crate::Reg<intflaga::INTFLAGA_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge A"]
pub mod intflaga;
#[doc = "INTFLAGB (rw) register accessor: an alias for `Reg<INTFLAGB_SPEC>`"]
pub type INTFLAGB = crate::Reg<intflagb::INTFLAGB_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge B"]
pub mod intflagb;
#[doc = "INTFLAGC (rw) register accessor: an alias for `Reg<INTFLAGC_SPEC>`"]
pub type INTFLAGC = crate::Reg<intflagc::INTFLAGC_SPEC>;
#[doc = "Peripheral interrupt flag status - Bridge C"]
pub mod intflagc;
#[doc = "STATUSA (r) register accessor: an alias for `Reg<STATUSA_SPEC>`"]
pub type STATUSA = crate::Reg<statusa::STATUSA_SPEC>;
#[doc = "Peripheral write protection status - Bridge A"]
pub mod statusa;
#[doc = "STATUSB (r) register accessor: an alias for `Reg<STATUSB_SPEC>`"]
pub type STATUSB = crate::Reg<statusb::STATUSB_SPEC>;
#[doc = "Peripheral write protection status - Bridge B"]
pub mod statusb;
#[doc = "STATUSC (r) register accessor: an alias for `Reg<STATUSC_SPEC>`"]
pub type STATUSC = crate::Reg<statusc::STATUSC_SPEC>;
#[doc = "Peripheral write protection status - Bridge C"]
pub mod statusc;
#[doc = "NONSECA (r) register accessor: an alias for `Reg<NONSECA_SPEC>`"]
pub type NONSECA = crate::Reg<nonseca::NONSECA_SPEC>;
#[doc = "Peripheral non-secure status - Bridge A"]
pub mod nonseca;
#[doc = "NONSECB (r) register accessor: an alias for `Reg<NONSECB_SPEC>`"]
pub type NONSECB = crate::Reg<nonsecb::NONSECB_SPEC>;
#[doc = "Peripheral non-secure status - Bridge B"]
pub mod nonsecb;
#[doc = "NONSECC (r) register accessor: an alias for `Reg<NONSECC_SPEC>`"]
pub type NONSECC = crate::Reg<nonsecc::NONSECC_SPEC>;
#[doc = "Peripheral non-secure status - Bridge C"]
pub mod nonsecc;
#[doc = "SECLOCKA (r) register accessor: an alias for `Reg<SECLOCKA_SPEC>`"]
pub type SECLOCKA = crate::Reg<seclocka::SECLOCKA_SPEC>;
#[doc = "Peripheral secure status locked - Bridge A"]
pub mod seclocka;
#[doc = "SECLOCKB (r) register accessor: an alias for `Reg<SECLOCKB_SPEC>`"]
pub type SECLOCKB = crate::Reg<seclockb::SECLOCKB_SPEC>;
#[doc = "Peripheral secure status locked - Bridge B"]
pub mod seclockb;
#[doc = "SECLOCKC (r) register accessor: an alias for `Reg<SECLOCKC_SPEC>`"]
pub type SECLOCKC = crate::Reg<seclockc::SECLOCKC_SPEC>;
#[doc = "Peripheral secure status locked - Bridge C"]
pub mod seclockc;

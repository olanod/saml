#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Control"]
    pub evctrl: EVCTRL,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x08 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x10 - Status"]
    pub status: STATUS,
    #[doc = "0x14 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    pub xoscctrl: XOSCCTRL,
    #[doc = "0x16 - Clock Failure Detector Prescaler"]
    pub cfdpresc: CFDPRESC,
    _reserved7: [u8; 0x01],
    #[doc = "0x18 - 16MHz Internal Oscillator (OSC16M) Control"]
    pub osc16mctrl: OSC16MCTRL,
    _reserved8: [u8; 0x03],
    #[doc = "0x1c - DFLLULP Control"]
    pub dfllulpctrl: DFLLULPCTRL,
    #[doc = "0x1e - DFLLULP Dither Control"]
    pub dfllulpdither: DFLLULPDITHER,
    #[doc = "0x1f - DFLLULP Read Request"]
    pub dfllulprreq: DFLLULPRREQ,
    #[doc = "0x20 - DFLLULP Delay Value"]
    pub dfllulpdly: DFLLULPDLY,
    #[doc = "0x24 - DFLLULP Target Ratio"]
    pub dfllulpratio: DFLLULPRATIO,
    #[doc = "0x28 - DFLLULP Synchronization Busy"]
    pub dfllulpsyncbusy: DFLLULPSYNCBUSY,
    #[doc = "0x2c - DPLL Control A"]
    pub dpllctrla: DPLLCTRLA,
    _reserved15: [u8; 0x03],
    #[doc = "0x30 - DPLL Ratio Control"]
    pub dpllratio: DPLLRATIO,
    #[doc = "0x34 - DPLL Control B"]
    pub dpllctrlb: DPLLCTRLB,
    #[doc = "0x38 - DPLL Prescaler"]
    pub dpllpresc: DPLLPRESC,
    _reserved18: [u8; 0x03],
    #[doc = "0x3c - DPLL Synchronization Busy"]
    pub dpllsyncbusy: DPLLSYNCBUSY,
    _reserved19: [u8; 0x03],
    #[doc = "0x40 - DPLL Status"]
    pub dpllstatus: DPLLSTATUS,
}
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
#[doc = "XOSCCTRL (rw) register accessor: an alias for `Reg<XOSCCTRL_SPEC>`"]
pub type XOSCCTRL = crate::Reg<xoscctrl::XOSCCTRL_SPEC>;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xoscctrl;
#[doc = "CFDPRESC (rw) register accessor: an alias for `Reg<CFDPRESC_SPEC>`"]
pub type CFDPRESC = crate::Reg<cfdpresc::CFDPRESC_SPEC>;
#[doc = "Clock Failure Detector Prescaler"]
pub mod cfdpresc;
#[doc = "OSC16MCTRL (rw) register accessor: an alias for `Reg<OSC16MCTRL_SPEC>`"]
pub type OSC16MCTRL = crate::Reg<osc16mctrl::OSC16MCTRL_SPEC>;
#[doc = "16MHz Internal Oscillator (OSC16M) Control"]
pub mod osc16mctrl;
#[doc = "DFLLULPCTRL (rw) register accessor: an alias for `Reg<DFLLULPCTRL_SPEC>`"]
pub type DFLLULPCTRL = crate::Reg<dfllulpctrl::DFLLULPCTRL_SPEC>;
#[doc = "DFLLULP Control"]
pub mod dfllulpctrl;
#[doc = "DFLLULPDITHER (rw) register accessor: an alias for `Reg<DFLLULPDITHER_SPEC>`"]
pub type DFLLULPDITHER = crate::Reg<dfllulpdither::DFLLULPDITHER_SPEC>;
#[doc = "DFLLULP Dither Control"]
pub mod dfllulpdither;
#[doc = "DFLLULPRREQ (rw) register accessor: an alias for `Reg<DFLLULPRREQ_SPEC>`"]
pub type DFLLULPRREQ = crate::Reg<dfllulprreq::DFLLULPRREQ_SPEC>;
#[doc = "DFLLULP Read Request"]
pub mod dfllulprreq;
#[doc = "DFLLULPDLY (rw) register accessor: an alias for `Reg<DFLLULPDLY_SPEC>`"]
pub type DFLLULPDLY = crate::Reg<dfllulpdly::DFLLULPDLY_SPEC>;
#[doc = "DFLLULP Delay Value"]
pub mod dfllulpdly;
#[doc = "DFLLULPRATIO (rw) register accessor: an alias for `Reg<DFLLULPRATIO_SPEC>`"]
pub type DFLLULPRATIO = crate::Reg<dfllulpratio::DFLLULPRATIO_SPEC>;
#[doc = "DFLLULP Target Ratio"]
pub mod dfllulpratio;
#[doc = "DFLLULPSYNCBUSY (r) register accessor: an alias for `Reg<DFLLULPSYNCBUSY_SPEC>`"]
pub type DFLLULPSYNCBUSY = crate::Reg<dfllulpsyncbusy::DFLLULPSYNCBUSY_SPEC>;
#[doc = "DFLLULP Synchronization Busy"]
pub mod dfllulpsyncbusy;
#[doc = "DPLLCTRLA (rw) register accessor: an alias for `Reg<DPLLCTRLA_SPEC>`"]
pub type DPLLCTRLA = crate::Reg<dpllctrla::DPLLCTRLA_SPEC>;
#[doc = "DPLL Control A"]
pub mod dpllctrla;
#[doc = "DPLLRATIO (rw) register accessor: an alias for `Reg<DPLLRATIO_SPEC>`"]
pub type DPLLRATIO = crate::Reg<dpllratio::DPLLRATIO_SPEC>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB (rw) register accessor: an alias for `Reg<DPLLCTRLB_SPEC>`"]
pub type DPLLCTRLB = crate::Reg<dpllctrlb::DPLLCTRLB_SPEC>;
#[doc = "DPLL Control B"]
pub mod dpllctrlb;
#[doc = "DPLLPRESC (rw) register accessor: an alias for `Reg<DPLLPRESC_SPEC>`"]
pub type DPLLPRESC = crate::Reg<dpllpresc::DPLLPRESC_SPEC>;
#[doc = "DPLL Prescaler"]
pub mod dpllpresc;
#[doc = "DPLLSYNCBUSY (r) register accessor: an alias for `Reg<DPLLSYNCBUSY_SPEC>`"]
pub type DPLLSYNCBUSY = crate::Reg<dpllsyncbusy::DPLLSYNCBUSY_SPEC>;
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLLSTATUS (r) register accessor: an alias for `Reg<DPLLSTATUS_SPEC>`"]
pub type DPLLSTATUS = crate::Reg<dpllstatus::DPLLSTATUS_SPEC>;
#[doc = "DPLL Status"]
pub mod dpllstatus;

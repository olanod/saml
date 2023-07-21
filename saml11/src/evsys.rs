#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Software Event"]
    pub swevt: SWEVT,
    #[doc = "0x08 - Priority Control"]
    pub prictrl: PRICTRL,
    _reserved3: [u8; 0x07],
    #[doc = "0x10 - Channel Pending Interrupt"]
    pub intpend: INTPEND,
    _reserved4: [u8; 0x02],
    #[doc = "0x14 - Interrupt Status"]
    pub intstatus: INTSTATUS,
    #[doc = "0x18 - Busy Channels"]
    pub busych: BUSYCH,
    #[doc = "0x1c - Ready Users"]
    pub readyusr: READYUSR,
    #[doc = "0x20..0x60 - CHANNEL\\[%s\\]"]
    pub channel: [CHANNEL; 8],
    _reserved8: [u8; 0xc0],
    #[doc = "0x120..0x137 - User Multiplexer n"]
    pub user: [USER; 23],
    _reserved9: [u8; 0x9d],
    #[doc = "0x1d4 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x1d5 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x1d6 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    _reserved12: [u8; 0x01],
    #[doc = "0x1d8 - Channels Security Attribution"]
    pub nonsecchan: NONSECCHAN,
    #[doc = "0x1dc - Non-Secure Channels Check"]
    pub nschkchan: NSCHKCHAN,
    #[doc = "0x1e0 - Users Security Attribution"]
    pub nonsecuser: [NONSECUSER; 1],
    _reserved15: [u8; 0x0c],
    #[doc = "0x1f0 - Non-Secure Users Check"]
    pub nschkuser: [NSCHKUSER; 1],
}
#[doc = "CTRLA (w) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "SWEVT (w) register accessor: an alias for `Reg<SWEVT_SPEC>`"]
pub type SWEVT = crate::Reg<swevt::SWEVT_SPEC>;
#[doc = "Software Event"]
pub mod swevt;
#[doc = "PRICTRL (rw) register accessor: an alias for `Reg<PRICTRL_SPEC>`"]
pub type PRICTRL = crate::Reg<prictrl::PRICTRL_SPEC>;
#[doc = "Priority Control"]
pub mod prictrl;
#[doc = "INTPEND (rw) register accessor: an alias for `Reg<INTPEND_SPEC>`"]
pub type INTPEND = crate::Reg<intpend::INTPEND_SPEC>;
#[doc = "Channel Pending Interrupt"]
pub mod intpend;
#[doc = "INTSTATUS (r) register accessor: an alias for `Reg<INTSTATUS_SPEC>`"]
pub type INTSTATUS = crate::Reg<intstatus::INTSTATUS_SPEC>;
#[doc = "Interrupt Status"]
pub mod intstatus;
#[doc = "BUSYCH (r) register accessor: an alias for `Reg<BUSYCH_SPEC>`"]
pub type BUSYCH = crate::Reg<busych::BUSYCH_SPEC>;
#[doc = "Busy Channels"]
pub mod busych;
#[doc = "READYUSR (r) register accessor: an alias for `Reg<READYUSR_SPEC>`"]
pub type READYUSR = crate::Reg<readyusr::READYUSR_SPEC>;
#[doc = "Ready Users"]
pub mod readyusr;
#[doc = "CHANNEL\\[%s\\]"]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "CHANNEL\\[%s\\]"]
pub mod channel;
#[doc = "USER (rw) register accessor: an alias for `Reg<USER_SPEC>`"]
pub type USER = crate::Reg<user::USER_SPEC>;
#[doc = "User Multiplexer n"]
pub mod user;
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
#[doc = "NONSECCHAN (rw) register accessor: an alias for `Reg<NONSECCHAN_SPEC>`"]
pub type NONSECCHAN = crate::Reg<nonsecchan::NONSECCHAN_SPEC>;
#[doc = "Channels Security Attribution"]
pub mod nonsecchan;
#[doc = "NSCHKCHAN (rw) register accessor: an alias for `Reg<NSCHKCHAN_SPEC>`"]
pub type NSCHKCHAN = crate::Reg<nschkchan::NSCHKCHAN_SPEC>;
#[doc = "Non-Secure Channels Check"]
pub mod nschkchan;
#[doc = "NONSECUSER (rw) register accessor: an alias for `Reg<NONSECUSER_SPEC>`"]
pub type NONSECUSER = crate::Reg<nonsecuser::NONSECUSER_SPEC>;
#[doc = "Users Security Attribution"]
pub mod nonsecuser;
#[doc = "NSCHKUSER (rw) register accessor: an alias for `Reg<NSCHKUSER_SPEC>`"]
pub type NSCHKUSER = crate::Reg<nschkuser::NSCHKUSER_SPEC>;
#[doc = "Non-Secure Users Check"]
pub mod nschkuser;

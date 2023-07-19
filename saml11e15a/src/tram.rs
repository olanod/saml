#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctrla: CTRLA,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Interrupt Enable Clear"]
    pub intenclr: INTENCLR,
    #[doc = "0x05 - Interrupt Enable Set"]
    pub intenset: INTENSET,
    #[doc = "0x06 - Interrupt Flag Status and Clear"]
    pub intflag: INTFLAG,
    #[doc = "0x07 - Status"]
    pub status: STATUS,
    #[doc = "0x08 - Synchronization Busy Status"]
    pub syncbusy: SYNCBUSY,
    #[doc = "0x0c - Data Scramble Control"]
    pub dscc: DSCC,
    #[doc = "0x10 - Permutation Write"]
    pub permw: PERMW,
    #[doc = "0x11 - Permutation Read"]
    pub permr: PERMR,
    _reserved9: [u8; 0xee],
    #[doc = "0x100..0x200 - TrustRAM"]
    pub ram: [RAM; 64],
}
#[doc = "CTRLA (rw) register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control"]
pub mod ctrla;
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
#[doc = "SYNCBUSY (r) register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "Synchronization Busy Status"]
pub mod syncbusy;
#[doc = "DSCC (w) register accessor: an alias for `Reg<DSCC_SPEC>`"]
pub type DSCC = crate::Reg<dscc::DSCC_SPEC>;
#[doc = "Data Scramble Control"]
pub mod dscc;
#[doc = "PERMW (w) register accessor: an alias for `Reg<PERMW_SPEC>`"]
pub type PERMW = crate::Reg<permw::PERMW_SPEC>;
#[doc = "Permutation Write"]
pub mod permw;
#[doc = "PERMR (r) register accessor: an alias for `Reg<PERMR_SPEC>`"]
pub type PERMR = crate::Reg<permr::PERMR_SPEC>;
#[doc = "Permutation Read"]
pub mod permr;
#[doc = "RAM (rw) register accessor: an alias for `Reg<RAM_SPEC>`"]
pub type RAM = crate::Reg<ram::RAM_SPEC>;
#[doc = "TrustRAM"]
pub mod ram;

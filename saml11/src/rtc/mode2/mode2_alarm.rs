#[doc = r"Register block"]
#[repr(C)]
pub struct MODE2_ALARM {
    #[doc = "0x00 - MODE2_ALARM Alarm n Value"]
    pub alarm: ALARM,
    #[doc = "0x04 - MODE2_ALARM Alarm n Mask"]
    pub mask: MASK,
}
#[doc = "ALARM (rw) register accessor: an alias for `Reg<ALARM_SPEC>`"]
pub type ALARM = crate::Reg<alarm::ALARM_SPEC>;
#[doc = "MODE2_ALARM Alarm n Value"]
pub mod alarm;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "MODE2_ALARM Alarm n Mask"]
pub mod mask;

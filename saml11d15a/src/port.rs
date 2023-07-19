#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x74 - GROUP\\[%s\\]"]
    pub group: [GROUP; 1],
}
#[doc = "GROUP\\[%s\\]"]
pub use self::group::GROUP;
#[doc = r"Cluster"]
#[doc = "GROUP\\[%s\\]"]
pub mod group;

#[doc = "Register `SECLOCKB` reader"]
pub struct R(crate::R<SECLOCKB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECLOCKB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECLOCKB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECLOCKB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDAU_` reader - IDAU Secure Status Locked"]
pub type IDAU__R = crate::BitReader;
#[doc = "Field `DSU_` reader - DSU Secure Status Locked"]
pub type DSU__R = crate::BitReader;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL Secure Status Locked"]
pub type NVMCTRL__R = crate::BitReader;
#[doc = "Field `DMAC_` reader - DMAC Secure Status Locked"]
pub type DMAC__R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IDAU Secure Status Locked"]
    #[inline(always)]
    pub fn idau_(&self) -> IDAU__R {
        IDAU__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU Secure Status Locked"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL Secure Status Locked"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Secure Status Locked"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Peripheral secure status locked - Bridge B\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seclockb](index.html) module"]
pub struct SECLOCKB_SPEC;
impl crate::RegisterSpec for SECLOCKB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seclockb::R](R) reader structure"]
impl crate::Readable for SECLOCKB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECLOCKB to value 0x03"]
impl crate::Resettable for SECLOCKB_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}

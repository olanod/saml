#[doc = "Register `DLSR` reader"]
pub struct R(crate::R<DLSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLI` reader - Software Lock implemented"]
pub type SLI_R = crate::BitReader;
#[doc = "Field `SLK` reader - Software Lock status"]
pub type SLK_R = crate::BitReader;
#[doc = "Field `nTT` reader - Not thirty-two bit"]
pub type N_TT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Lock implemented"]
    #[inline(always)]
    pub fn sli(&self) -> SLI_R {
        SLI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Lock status"]
    #[inline(always)]
    pub fn slk(&self) -> SLK_R {
        SLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Not thirty-two bit"]
    #[inline(always)]
    pub fn n_tt(&self) -> N_TT_R {
        N_TT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "SCS Software Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlsr](index.html) module"]
pub struct DLSR_SPEC;
impl crate::RegisterSpec for DLSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlsr::R](R) reader structure"]
impl crate::Readable for DLSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLSR to value 0"]
impl crate::Resettable for DLSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

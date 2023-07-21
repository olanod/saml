#[doc = "Register `SYST_CALIB` reader"]
pub struct R(crate::R<SYST_CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TENMS` reader - Ten milliseconds"]
pub type TENMS_R = crate::FieldReader<u32>;
#[doc = "Field `SKEW` reader - Skew"]
pub type SKEW_R = crate::BitReader;
#[doc = "Field `NOREF` reader - No reference"]
pub type NOREF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - Ten milliseconds"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - Skew"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - No reference"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SysTick Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_calib](index.html) module"]
pub struct SYST_CALIB_SPEC;
impl crate::RegisterSpec for SYST_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_calib::R](R) reader structure"]
impl crate::Readable for SYST_CALIB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYST_CALIB to value 0"]
impl crate::Resettable for SYST_CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

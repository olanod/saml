#[doc = "Register `DFLLULPSYNCBUSY` reader"]
pub struct R(crate::R<DFLLULPSYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLULPSYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLULPSYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLULPSYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENABLE` reader - Enable Bit Synchronization Busy"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `TUNE` reader - Tune Bit Synchronization Busy"]
pub type TUNE_R = crate::BitReader;
#[doc = "Field `DELAY` reader - Delay Register Synchronization Busy"]
pub type DELAY_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Enable Bit Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tune Bit Synchronization Busy"]
    #[inline(always)]
    pub fn tune(&self) -> TUNE_R {
        TUNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Delay Register Synchronization Busy"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DFLLULP Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllulpsyncbusy](index.html) module"]
pub struct DFLLULPSYNCBUSY_SPEC;
impl crate::RegisterSpec for DFLLULPSYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfllulpsyncbusy::R](R) reader structure"]
impl crate::Readable for DFLLULPSYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFLLULPSYNCBUSY to value 0"]
impl crate::Resettable for DFLLULPSYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

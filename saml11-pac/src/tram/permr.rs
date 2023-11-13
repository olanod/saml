#[doc = "Register `PERMR` reader"]
pub struct R(crate::R<PERMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Permutation Scrambler Data Output"]
pub type DATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Permutation Scrambler Data Output"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 7)
    }
}
#[doc = "Permutation Read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [permr](index.html) module"]
pub struct PERMR_SPEC;
impl crate::RegisterSpec for PERMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [permr::R](R) reader structure"]
impl crate::Readable for PERMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERMR to value 0"]
impl crate::Resettable for PERMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

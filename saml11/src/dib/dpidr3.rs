#[doc = "Register `DPIDR3` reader"]
pub struct R(crate::R<DPIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMOD` reader - Customer Modified"]
pub type CMOD_R = crate::FieldReader;
#[doc = "Field `REVAND` reader - RevAnd"]
pub type REVAND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Customer Modified"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RevAnd"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SCS Peripheral Identification Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpidr3](index.html) module"]
pub struct DPIDR3_SPEC;
impl crate::RegisterSpec for DPIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpidr3::R](R) reader structure"]
impl crate::Readable for DPIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DPIDR3 to value 0"]
impl crate::Resettable for DPIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `PID6` reader"]
pub struct R(crate::R<PID6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID6_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PID6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Peripheral Identification 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid6](index.html) module"]
pub struct PID6_SPEC;
impl crate::RegisterSpec for PID6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid6::R](R) reader structure"]
impl crate::Readable for PID6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PID6 to value 0"]
impl crate::Resettable for PID6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `ACTLR` reader"]
pub struct R(crate::R<ACTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTLR` writer"]
pub struct W(crate::W<ACTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ACTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTLR_SPEC>) -> Self {
        W(writer)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ACTLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](index.html) module"]
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actlr::R](R) reader structure"]
impl crate::Readable for ACTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actlr::W](W) writer structure"]
impl crate::Writable for ACTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ACTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

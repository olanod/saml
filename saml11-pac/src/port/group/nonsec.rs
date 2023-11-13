#[doc = "Register `NONSEC` reader"]
pub struct R(crate::R<NONSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NONSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NONSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NONSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NONSEC` writer"]
pub struct W(crate::W<NONSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NONSEC_SPEC>;
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
impl From<crate::W<NONSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NONSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONSEC` reader - Port Security Attribution"]
pub type NONSEC_R = crate::FieldReader<u32>;
#[doc = "Field `NONSEC` writer - Port Security Attribution"]
pub type NONSEC_W<'a, const O: u8> = crate::FieldWriter<'a, NONSEC_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Security Attribution"]
    #[inline(always)]
    pub fn nonsec(&self) -> NONSEC_R {
        NONSEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Security Attribution"]
    #[inline(always)]
    #[must_use]
    pub fn nonsec(&mut self) -> NONSEC_W<0> {
        NONSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Attribution\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nonsec](index.html) module"]
pub struct NONSEC_SPEC;
impl crate::RegisterSpec for NONSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nonsec::R](R) reader structure"]
impl crate::Readable for NONSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nonsec::W](W) writer structure"]
impl crate::Writable for NONSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NONSEC to value 0"]
impl crate::Resettable for NONSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

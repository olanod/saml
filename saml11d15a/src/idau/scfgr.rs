#[doc = "Register `SCFGR` reader"]
pub struct R(crate::R<SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFGR` writer"]
pub struct W(crate::W<SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFGR_SPEC>;
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
impl From<crate::W<SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS` reader - RAM Secure"]
pub type RS_R = crate::FieldReader;
#[doc = "Field `RS` writer - RAM Secure"]
pub type RS_W<'a, const O: u8> = crate::FieldWriter<'a, SCFGR_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - RAM Secure"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - RAM Secure"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<0> {
        RS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgr](index.html) module"]
pub struct SCFGR_SPEC;
impl crate::RegisterSpec for SCFGR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scfgr::R](R) reader structure"]
impl crate::Readable for SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfgr::W](W) writer structure"]
impl crate::Writable for SCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCFGR to value 0"]
impl crate::Resettable for SCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

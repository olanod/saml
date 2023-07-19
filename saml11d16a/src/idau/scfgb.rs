#[doc = "Register `SCFGB` reader"]
pub struct R(crate::R<SCFGB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFGB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFGB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFGB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFGB` writer"]
pub struct W(crate::W<SCFGB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFGB_SPEC>;
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
impl From<crate::W<SCFGB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFGB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BS` reader - Boot Secure"]
pub type BS_R = crate::FieldReader;
#[doc = "Field `BS` writer - Boot Secure"]
pub type BS_W<'a, const O: u8> = crate::FieldWriter<'a, SCFGB_SPEC, 8, O>;
#[doc = "Field `BNSC` reader - Boot Secure, Non-secure Callable"]
pub type BNSC_R = crate::FieldReader;
#[doc = "Field `BNSC` writer - Boot Secure, Non-secure Callable"]
pub type BNSC_W<'a, const O: u8> = crate::FieldWriter<'a, SCFGB_SPEC, 6, O>;
#[doc = "Field `BOOTPROT` reader - Boot Protection"]
pub type BOOTPROT_R = crate::FieldReader;
#[doc = "Field `BOOTPROT` writer - Boot Protection"]
pub type BOOTPROT_W<'a, const O: u8> = crate::FieldWriter<'a, SCFGB_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Boot Secure"]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Boot Secure, Non-secure Callable"]
    #[inline(always)]
    pub fn bnsc(&self) -> BNSC_R {
        BNSC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Boot Protection"]
    #[inline(always)]
    pub fn bootprot(&self) -> BOOTPROT_R {
        BOOTPROT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Boot Secure"]
    #[inline(always)]
    #[must_use]
    pub fn bs(&mut self) -> BS_W<0> {
        BS_W::new(self)
    }
    #[doc = "Bits 8:13 - Boot Secure, Non-secure Callable"]
    #[inline(always)]
    #[must_use]
    pub fn bnsc(&mut self) -> BNSC_W<8> {
        BNSC_W::new(self)
    }
    #[doc = "Bits 16:23 - Boot Protection"]
    #[inline(always)]
    #[must_use]
    pub fn bootprot(&mut self) -> BOOTPROT_W<16> {
        BOOTPROT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCFGB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgb](index.html) module"]
pub struct SCFGB_SPEC;
impl crate::RegisterSpec for SCFGB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfgb::R](R) reader structure"]
impl crate::Readable for SCFGB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfgb::W](W) writer structure"]
impl crate::Writable for SCFGB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCFGB to value 0"]
impl crate::Resettable for SCFGB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

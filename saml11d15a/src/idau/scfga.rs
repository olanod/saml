#[doc = "Register `SCFGA` reader"]
pub struct R(crate::R<SCFGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFGA` writer"]
pub struct W(crate::W<SCFGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFGA_SPEC>;
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
impl From<crate::W<SCFGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AS` reader - Application Secure"]
pub type AS_R = crate::FieldReader;
#[doc = "Field `AS` writer - Application Secure"]
pub type AS_W<'a, const O: u8> = crate::FieldWriter<'a, SCFGA_SPEC, 8, O>;
#[doc = "Field `ANSC` reader - Application Secure, Non-secure Callable"]
pub type ANSC_R = crate::FieldReader;
#[doc = "Field `ANSC` writer - Application Secure, Non-secure Callable"]
pub type ANSC_W<'a, const O: u8> = crate::FieldWriter<'a, SCFGA_SPEC, 6, O>;
#[doc = "Field `DS` reader - DATAFLASH Data Secure"]
pub type DS_R = crate::FieldReader;
#[doc = "Field `DS` writer - DATAFLASH Data Secure"]
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, SCFGA_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Application Secure"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Application Secure, Non-secure Callable"]
    #[inline(always)]
    pub fn ansc(&self) -> ANSC_R {
        ANSC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - DATAFLASH Data Secure"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Application Secure"]
    #[inline(always)]
    #[must_use]
    pub fn as_(&mut self) -> AS_W<0> {
        AS_W::new(self)
    }
    #[doc = "Bits 8:13 - Application Secure, Non-secure Callable"]
    #[inline(always)]
    #[must_use]
    pub fn ansc(&mut self) -> ANSC_W<8> {
        ANSC_W::new(self)
    }
    #[doc = "Bits 16:19 - DATAFLASH Data Secure"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<16> {
        DS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCFGA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfga](index.html) module"]
pub struct SCFGA_SPEC;
impl crate::RegisterSpec for SCFGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfga::R](R) reader structure"]
impl crate::Readable for SCFGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfga::W](W) writer structure"]
impl crate::Writable for SCFGA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCFGA to value 0"]
impl crate::Resettable for SCFGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

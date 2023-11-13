#[doc = "Register `NSCHK` reader"]
pub struct R(crate::R<NSCHK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSCHK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSCHK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSCHK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSCHK` writer"]
pub struct W(crate::W<NSCHK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSCHK_SPEC>;
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
impl From<crate::W<NSCHK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSCHK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTINT` reader - External Interrupt Nonsecure Check Enable"]
pub type EXTINT_R = crate::FieldReader;
#[doc = "Field `EXTINT` writer - External Interrupt Nonsecure Check Enable"]
pub type EXTINT_W<'a, const O: u8> = crate::FieldWriter<'a, NSCHK_SPEC, 8, O>;
#[doc = "Field `NMI` reader - Non-Maskable External Interrupt Nonsecure Check Enable"]
pub type NMI_R = crate::BitReader;
#[doc = "Field `NMI` writer - Non-Maskable External Interrupt Nonsecure Check Enable"]
pub type NMI_W<'a, const O: u8> = crate::BitWriter<'a, NSCHK_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn extint(&self) -> EXTINT_R {
        EXTINT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Non-Maskable External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extint(&mut self) -> EXTINT_W<0> {
        EXTINT_W::new(self)
    }
    #[doc = "Bit 31 - Non-Maskable External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmi(&mut self) -> NMI_W<31> {
        NMI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-secure Interrupt Check Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nschk](index.html) module"]
pub struct NSCHK_SPEC;
impl crate::RegisterSpec for NSCHK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nschk::R](R) reader structure"]
impl crate::Readable for NSCHK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nschk::W](W) writer structure"]
impl crate::Writable for NSCHK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSCHK to value 0"]
impl crate::Resettable for NSCHK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

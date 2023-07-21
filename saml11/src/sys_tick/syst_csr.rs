#[doc = "Register `SYST_CSR` reader"]
pub struct R(crate::R<SYST_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_CSR` writer"]
pub struct W(crate::W<SYST_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_CSR_SPEC>;
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
impl From<crate::W<SYST_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYST_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - SysTick enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - SysTick enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SYST_CSR_SPEC, O>;
#[doc = "Field `TICKINT` reader - Tick interrupt"]
pub type TICKINT_R = crate::BitReader;
#[doc = "Field `TICKINT` writer - Tick interrupt"]
pub type TICKINT_W<'a, const O: u8> = crate::BitWriter<'a, SYST_CSR_SPEC, O>;
#[doc = "Field `CLKSOURCE` reader - Clock source"]
pub type CLKSOURCE_R = crate::BitReader;
#[doc = "Field `CLKSOURCE` writer - Clock source"]
pub type CLKSOURCE_W<'a, const O: u8> = crate::BitWriter<'a, SYST_CSR_SPEC, O>;
#[doc = "Field `COUNTFLAG` reader - Count flag"]
pub type COUNTFLAG_R = crate::BitReader;
#[doc = "Field `COUNTFLAG` writer - Count flag"]
pub type COUNTFLAG_W<'a, const O: u8> = crate::BitWriter<'a, SYST_CSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SysTick enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tick interrupt"]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock source"]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Count flag"]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SysTick enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Tick interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tickint(&mut self) -> TICKINT_W<1> {
        TICKINT_W::new(self)
    }
    #[doc = "Bit 2 - Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn clksource(&mut self) -> CLKSOURCE_W<2> {
        CLKSOURCE_W::new(self)
    }
    #[doc = "Bit 16 - Count flag"]
    #[inline(always)]
    #[must_use]
    pub fn countflag(&mut self) -> COUNTFLAG_W<16> {
        COUNTFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_csr](index.html) module"]
pub struct SYST_CSR_SPEC;
impl crate::RegisterSpec for SYST_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_csr::R](R) reader structure"]
impl crate::Readable for SYST_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_csr::W](W) writer structure"]
impl crate::Writable for SYST_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYST_CSR to value 0"]
impl crate::Resettable for SYST_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

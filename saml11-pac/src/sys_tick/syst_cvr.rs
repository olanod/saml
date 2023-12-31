#[doc = "Register `SYST_CVR` reader"]
pub struct R(crate::R<SYST_CVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_CVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_CVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_CVR` writer"]
pub struct W(crate::W<SYST_CVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_CVR_SPEC>;
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
impl From<crate::W<SYST_CVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYST_CVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURRENT` reader - Current counter value"]
pub type CURRENT_R = crate::BitReader;
#[doc = "Field `CURRENT` writer - Current counter value"]
pub type CURRENT_W<'a, const O: u8> = crate::BitWriter<'a, SYST_CVR_SPEC, O>;
impl R {
    #[doc = "Bit 24 - Current counter value"]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Current counter value"]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CURRENT_W<24> {
        CURRENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SysTick Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syst_cvr](index.html) module"]
pub struct SYST_CVR_SPEC;
impl crate::RegisterSpec for SYST_CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_cvr::R](R) reader structure"]
impl crate::Readable for SYST_CVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_cvr::W](W) writer structure"]
impl crate::Writable for SYST_CVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYST_CVR to value 0"]
impl crate::Resettable for SYST_CVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

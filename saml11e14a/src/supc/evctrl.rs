#[doc = "Register `EVCTRL` reader"]
pub struct R(crate::R<EVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRL` writer"]
pub struct W(crate::W<EVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRL_SPEC>;
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
impl From<crate::W<EVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOD33DETEO` reader - BOD33 Detection Event Output Enable"]
pub type BOD33DETEO_R = crate::BitReader;
#[doc = "Field `BOD33DETEO` writer - BOD33 Detection Event Output Enable"]
pub type BOD33DETEO_W<'a, const O: u8> = crate::BitWriter<'a, EVCTRL_SPEC, O>;
#[doc = "Field `BOD12DETEO` reader - BOD12 Detection Event Output Enable"]
pub type BOD12DETEO_R = crate::BitReader;
#[doc = "Field `BOD12DETEO` writer - BOD12 Detection Event Output Enable"]
pub type BOD12DETEO_W<'a, const O: u8> = crate::BitWriter<'a, EVCTRL_SPEC, O>;
impl R {
    #[doc = "Bit 1 - BOD33 Detection Event Output Enable"]
    #[inline(always)]
    pub fn bod33deteo(&self) -> BOD33DETEO_R {
        BOD33DETEO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - BOD12 Detection Event Output Enable"]
    #[inline(always)]
    pub fn bod12deteo(&self) -> BOD12DETEO_R {
        BOD12DETEO_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - BOD33 Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bod33deteo(&mut self) -> BOD33DETEO_W<1> {
        BOD33DETEO_W::new(self)
    }
    #[doc = "Bit 4 - BOD12 Detection Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bod12deteo(&mut self) -> BOD12DETEO_W<4> {
        BOD12DETEO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrl](index.html) module"]
pub struct EVCTRL_SPEC;
impl crate::RegisterSpec for EVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evctrl::R](R) reader structure"]
impl crate::Readable for EVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrl::W](W) writer structure"]
impl crate::Writable for EVCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EVCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

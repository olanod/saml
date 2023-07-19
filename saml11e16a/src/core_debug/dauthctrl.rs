#[doc = "Register `DAUTHCTRL` reader"]
pub struct R(crate::R<DAUTHCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAUTHCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAUTHCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAUTHCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAUTHCTRL` writer"]
pub struct W(crate::W<DAUTHCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAUTHCTRL_SPEC>;
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
impl From<crate::W<DAUTHCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAUTHCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIDENSEL` reader - Secure invasive debug enable select"]
pub type SPIDENSEL_R = crate::BitReader;
#[doc = "Field `SPIDENSEL` writer - Secure invasive debug enable select"]
pub type SPIDENSEL_W<'a, const O: u8> = crate::BitWriter<'a, DAUTHCTRL_SPEC, O>;
#[doc = "Field `INTSPIDEN` reader - Internal Secure invasive debug enable"]
pub type INTSPIDEN_R = crate::BitReader;
#[doc = "Field `INTSPIDEN` writer - Internal Secure invasive debug enable"]
pub type INTSPIDEN_W<'a, const O: u8> = crate::BitWriter<'a, DAUTHCTRL_SPEC, O>;
#[doc = "Field `SPNIDENSEL` reader - Secure non-invasive debug enable select"]
pub type SPNIDENSEL_R = crate::BitReader;
#[doc = "Field `SPNIDENSEL` writer - Secure non-invasive debug enable select"]
pub type SPNIDENSEL_W<'a, const O: u8> = crate::BitWriter<'a, DAUTHCTRL_SPEC, O>;
#[doc = "Field `INTSPNIDEN` reader - Internal Secure non-invasive debug enable"]
pub type INTSPNIDEN_R = crate::BitReader;
#[doc = "Field `INTSPNIDEN` writer - Internal Secure non-invasive debug enable"]
pub type INTSPNIDEN_W<'a, const O: u8> = crate::BitWriter<'a, DAUTHCTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Secure invasive debug enable select"]
    #[inline(always)]
    pub fn spidensel(&self) -> SPIDENSEL_R {
        SPIDENSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Secure invasive debug enable"]
    #[inline(always)]
    pub fn intspiden(&self) -> INTSPIDEN_R {
        INTSPIDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure non-invasive debug enable select"]
    #[inline(always)]
    pub fn spnidensel(&self) -> SPNIDENSEL_R {
        SPNIDENSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal Secure non-invasive debug enable"]
    #[inline(always)]
    pub fn intspniden(&self) -> INTSPNIDEN_R {
        INTSPNIDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure invasive debug enable select"]
    #[inline(always)]
    #[must_use]
    pub fn spidensel(&mut self) -> SPIDENSEL_W<0> {
        SPIDENSEL_W::new(self)
    }
    #[doc = "Bit 1 - Internal Secure invasive debug enable"]
    #[inline(always)]
    #[must_use]
    pub fn intspiden(&mut self) -> INTSPIDEN_W<1> {
        INTSPIDEN_W::new(self)
    }
    #[doc = "Bit 2 - Secure non-invasive debug enable select"]
    #[inline(always)]
    #[must_use]
    pub fn spnidensel(&mut self) -> SPNIDENSEL_W<2> {
        SPNIDENSEL_W::new(self)
    }
    #[doc = "Bit 3 - Internal Secure non-invasive debug enable"]
    #[inline(always)]
    #[must_use]
    pub fn intspniden(&mut self) -> INTSPNIDEN_W<3> {
        INTSPNIDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Authentication Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dauthctrl](index.html) module"]
pub struct DAUTHCTRL_SPEC;
impl crate::RegisterSpec for DAUTHCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dauthctrl::R](R) reader structure"]
impl crate::Readable for DAUTHCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dauthctrl::W](W) writer structure"]
impl crate::Writable for DAUTHCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAUTHCTRL to value 0"]
impl crate::Resettable for DAUTHCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

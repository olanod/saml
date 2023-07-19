#[doc = "Register `DPRESCALER` reader"]
pub struct R(crate::R<DPRESCALER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPRESCALER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPRESCALER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPRESCALER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPRESCALER` writer"]
pub struct W(crate::W<DPRESCALER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPRESCALER_SPEC>;
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
impl From<crate::W<DPRESCALER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPRESCALER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER0` reader - Debouncer Prescaler"]
pub type PRESCALER0_R = crate::FieldReader;
#[doc = "Field `PRESCALER0` writer - Debouncer Prescaler"]
pub type PRESCALER0_W<'a, const O: u8> = crate::FieldWriter<'a, DPRESCALER_SPEC, 3, O>;
#[doc = "Field `STATES0` reader - Debouncer number of states"]
pub type STATES0_R = crate::BitReader;
#[doc = "Field `STATES0` writer - Debouncer number of states"]
pub type STATES0_W<'a, const O: u8> = crate::BitWriter<'a, DPRESCALER_SPEC, O>;
#[doc = "Field `TICKON` reader - Pin Sampler frequency selection"]
pub type TICKON_R = crate::BitReader;
#[doc = "Field `TICKON` writer - Pin Sampler frequency selection"]
pub type TICKON_W<'a, const O: u8> = crate::BitWriter<'a, DPRESCALER_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    pub fn prescaler0(&self) -> PRESCALER0_R {
        PRESCALER0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    pub fn states0(&self) -> STATES0_R {
        STATES0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    pub fn tickon(&self) -> TICKON_R {
        TICKON_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Debouncer Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler0(&mut self) -> PRESCALER0_W<0> {
        PRESCALER0_W::new(self)
    }
    #[doc = "Bit 3 - Debouncer number of states"]
    #[inline(always)]
    #[must_use]
    pub fn states0(&mut self) -> STATES0_W<3> {
        STATES0_W::new(self)
    }
    #[doc = "Bit 16 - Pin Sampler frequency selection"]
    #[inline(always)]
    #[must_use]
    pub fn tickon(&mut self) -> TICKON_W<16> {
        TICKON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debouncer Prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dprescaler](index.html) module"]
pub struct DPRESCALER_SPEC;
impl crate::RegisterSpec for DPRESCALER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dprescaler::R](R) reader structure"]
impl crate::Readable for DPRESCALER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dprescaler::W](W) writer structure"]
impl crate::Writable for DPRESCALER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPRESCALER to value 0"]
impl crate::Resettable for DPRESCALER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

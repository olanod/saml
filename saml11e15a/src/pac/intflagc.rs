#[doc = "Register `INTFLAGC` reader"]
pub struct R(crate::R<INTFLAGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGC` writer"]
pub struct W(crate::W<INTFLAGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGC_SPEC>;
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
impl From<crate::W<INTFLAGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS"]
pub type EVSYS__R = crate::BitReader;
#[doc = "Field `EVSYS_` writer - EVSYS"]
pub type EVSYS__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `SERCOM0_` reader - SERCOM0"]
pub type SERCOM0__R = crate::BitReader;
#[doc = "Field `SERCOM0_` writer - SERCOM0"]
pub type SERCOM0__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `SERCOM1_` reader - SERCOM1"]
pub type SERCOM1__R = crate::BitReader;
#[doc = "Field `SERCOM1_` writer - SERCOM1"]
pub type SERCOM1__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `SERCOM2_` reader - SERCOM2"]
pub type SERCOM2__R = crate::BitReader;
#[doc = "Field `SERCOM2_` writer - SERCOM2"]
pub type SERCOM2__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `TC0_` reader - TC0"]
pub type TC0__R = crate::BitReader;
#[doc = "Field `TC0_` writer - TC0"]
pub type TC0__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `TC1_` reader - TC1"]
pub type TC1__R = crate::BitReader;
#[doc = "Field `TC1_` writer - TC1"]
pub type TC1__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `TC2_` reader - TC2"]
pub type TC2__R = crate::BitReader;
#[doc = "Field `TC2_` writer - TC2"]
pub type TC2__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `ADC_` reader - ADC"]
pub type ADC__R = crate::BitReader;
#[doc = "Field `ADC_` writer - ADC"]
pub type ADC__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `DAC_` reader - DAC"]
pub type DAC__R = crate::BitReader;
#[doc = "Field `DAC_` writer - DAC"]
pub type DAC__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `PTC_` reader - PTC"]
pub type PTC__R = crate::BitReader;
#[doc = "Field `PTC_` writer - PTC"]
pub type PTC__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `TRNG_` reader - TRNG"]
pub type TRNG__R = crate::BitReader;
#[doc = "Field `TRNG_` writer - TRNG"]
pub type TRNG__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `CCL_` reader - CCL"]
pub type CCL__R = crate::BitReader;
#[doc = "Field `CCL_` writer - CCL"]
pub type CCL__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `OPAMP_` reader - OPAMP"]
pub type OPAMP__R = crate::BitReader;
#[doc = "Field `OPAMP_` writer - OPAMP"]
pub type OPAMP__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
#[doc = "Field `TRAM_` reader - TRAM"]
pub type TRAM__R = crate::BitReader;
#[doc = "Field `TRAM_` writer - TRAM"]
pub type TRAM__W<'a, const O: u8> = crate::BitWriter<'a, INTFLAGC_SPEC, O>;
impl R {
    #[doc = "Bit 0 - EVSYS"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM2"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC0"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC1"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC2"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTC"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CCL"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OPAMP"]
    #[inline(always)]
    pub fn opamp_(&self) -> OPAMP__R {
        OPAMP__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TRAM"]
    #[inline(always)]
    pub fn tram_(&self) -> TRAM__R {
        TRAM__R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EVSYS"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> EVSYS__W<0> {
        EVSYS__W::new(self)
    }
    #[doc = "Bit 1 - SERCOM0"]
    #[inline(always)]
    #[must_use]
    pub fn sercom0_(&mut self) -> SERCOM0__W<1> {
        SERCOM0__W::new(self)
    }
    #[doc = "Bit 2 - SERCOM1"]
    #[inline(always)]
    #[must_use]
    pub fn sercom1_(&mut self) -> SERCOM1__W<2> {
        SERCOM1__W::new(self)
    }
    #[doc = "Bit 3 - SERCOM2"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> SERCOM2__W<3> {
        SERCOM2__W::new(self)
    }
    #[doc = "Bit 4 - TC0"]
    #[inline(always)]
    #[must_use]
    pub fn tc0_(&mut self) -> TC0__W<4> {
        TC0__W::new(self)
    }
    #[doc = "Bit 5 - TC1"]
    #[inline(always)]
    #[must_use]
    pub fn tc1_(&mut self) -> TC1__W<5> {
        TC1__W::new(self)
    }
    #[doc = "Bit 6 - TC2"]
    #[inline(always)]
    #[must_use]
    pub fn tc2_(&mut self) -> TC2__W<6> {
        TC2__W::new(self)
    }
    #[doc = "Bit 7 - ADC"]
    #[inline(always)]
    #[must_use]
    pub fn adc_(&mut self) -> ADC__W<7> {
        ADC__W::new(self)
    }
    #[doc = "Bit 8 - DAC"]
    #[inline(always)]
    #[must_use]
    pub fn dac_(&mut self) -> DAC__W<8> {
        DAC__W::new(self)
    }
    #[doc = "Bit 9 - PTC"]
    #[inline(always)]
    #[must_use]
    pub fn ptc_(&mut self) -> PTC__W<9> {
        PTC__W::new(self)
    }
    #[doc = "Bit 10 - TRNG"]
    #[inline(always)]
    #[must_use]
    pub fn trng_(&mut self) -> TRNG__W<10> {
        TRNG__W::new(self)
    }
    #[doc = "Bit 11 - CCL"]
    #[inline(always)]
    #[must_use]
    pub fn ccl_(&mut self) -> CCL__W<11> {
        CCL__W::new(self)
    }
    #[doc = "Bit 12 - OPAMP"]
    #[inline(always)]
    #[must_use]
    pub fn opamp_(&mut self) -> OPAMP__W<12> {
        OPAMP__W::new(self)
    }
    #[doc = "Bit 13 - TRAM"]
    #[inline(always)]
    #[must_use]
    pub fn tram_(&mut self) -> TRAM__W<13> {
        TRAM__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagc](index.html) module"]
pub struct INTFLAGC_SPEC;
impl crate::RegisterSpec for INTFLAGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagc::R](R) reader structure"]
impl crate::Readable for INTFLAGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagc::W](W) writer structure"]
impl crate::Writable for INTFLAGC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGC to value 0"]
impl crate::Resettable for INTFLAGC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

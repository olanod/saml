#[doc = "Register `SECLOCKC` reader"]
pub struct R(crate::R<SECLOCKC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECLOCKC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECLOCKC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECLOCKC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS Secure Status Locked"]
pub type EVSYS__R = crate::BitReader;
#[doc = "Field `SERCOM0_` reader - SERCOM0 Secure Status Locked"]
pub type SERCOM0__R = crate::BitReader;
#[doc = "Field `SERCOM1_` reader - SERCOM1 Secure Status Locked"]
pub type SERCOM1__R = crate::BitReader;
#[doc = "Field `TC0_` reader - TC0 Secure Status Locked"]
pub type TC0__R = crate::BitReader;
#[doc = "Field `TC1_` reader - TC1 Secure Status Locked"]
pub type TC1__R = crate::BitReader;
#[doc = "Field `TC2_` reader - TC2 Secure Status Locked"]
pub type TC2__R = crate::BitReader;
#[doc = "Field `ADC_` reader - ADC Secure Status Locked"]
pub type ADC__R = crate::BitReader;
#[doc = "Field `DAC_` reader - DAC Secure Status Locked"]
pub type DAC__R = crate::BitReader;
#[doc = "Field `PTC_` reader - PTC Secure Status Locked"]
pub type PTC__R = crate::BitReader;
#[doc = "Field `TRNG_` reader - TRNG Secure Status Locked"]
pub type TRNG__R = crate::BitReader;
#[doc = "Field `CCL_` reader - CCL Secure Status Locked"]
pub type CCL__R = crate::BitReader;
#[doc = "Field `OPAMP_` reader - OPAMP Secure Status Locked"]
pub type OPAMP__R = crate::BitReader;
#[doc = "Field `TRAM_` reader - TRAM Secure Status Locked"]
pub type TRAM__R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EVSYS Secure Status Locked"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 Secure Status Locked"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 Secure Status Locked"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TC0 Secure Status Locked"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC1 Secure Status Locked"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TC2 Secure Status Locked"]
    #[inline(always)]
    pub fn tc2_(&self) -> TC2__R {
        TC2__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Secure Status Locked"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC Secure Status Locked"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PTC Secure Status Locked"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TRNG Secure Status Locked"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CCL Secure Status Locked"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OPAMP Secure Status Locked"]
    #[inline(always)]
    pub fn opamp_(&self) -> OPAMP__R {
        OPAMP__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TRAM Secure Status Locked"]
    #[inline(always)]
    pub fn tram_(&self) -> TRAM__R {
        TRAM__R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Peripheral secure status locked - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seclockc](index.html) module"]
pub struct SECLOCKC_SPEC;
impl crate::RegisterSpec for SECLOCKC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seclockc::R](R) reader structure"]
impl crate::Readable for SECLOCKC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECLOCKC to value 0"]
impl crate::Resettable for SECLOCKC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

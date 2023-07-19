#[doc = "Register `SECLOCKA` reader"]
pub struct R(crate::R<SECLOCKA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECLOCKA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECLOCKA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECLOCKA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PAC_` reader - PAC Secure Status Locked"]
pub type PAC__R = crate::BitReader;
#[doc = "Field `PM_` reader - PM Secure Status Locked"]
pub type PM__R = crate::BitReader;
#[doc = "Field `MCLK_` reader - MCLK Secure Status Locked"]
pub type MCLK__R = crate::BitReader;
#[doc = "Field `RSTC_` reader - RSTC Secure Status Locked"]
pub type RSTC__R = crate::BitReader;
#[doc = "Field `OSCCTRL_` reader - OSCCTRL Secure Status Locked"]
pub type OSCCTRL__R = crate::BitReader;
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL Secure Status Locked"]
pub type OSC32KCTRL__R = crate::BitReader;
#[doc = "Field `SUPC_` reader - SUPC Secure Status Locked"]
pub type SUPC__R = crate::BitReader;
#[doc = "Field `GCLK_` reader - GCLK Secure Status Locked"]
pub type GCLK__R = crate::BitReader;
#[doc = "Field `WDT_` reader - WDT Secure Status Locked"]
pub type WDT__R = crate::BitReader;
#[doc = "Field `RTC_` reader - RTC Secure Status Locked"]
pub type RTC__R = crate::BitReader;
#[doc = "Field `EIC_` reader - EIC Secure Status Locked"]
pub type EIC__R = crate::BitReader;
#[doc = "Field `FREQM_` reader - FREQM Secure Status Locked"]
pub type FREQM__R = crate::BitReader;
#[doc = "Field `PORT_` reader - PORT Secure Status Locked"]
pub type PORT__R = crate::BitReader;
#[doc = "Field `AC_` reader - AC Secure Status Locked"]
pub type AC__R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PAC Secure Status Locked"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PM Secure Status Locked"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCLK Secure Status Locked"]
    #[inline(always)]
    pub fn mclk_(&self) -> MCLK__R {
        MCLK__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSTC Secure Status Locked"]
    #[inline(always)]
    pub fn rstc_(&self) -> RSTC__R {
        RSTC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL Secure Status Locked"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> OSCCTRL__R {
        OSCCTRL__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL Secure Status Locked"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> OSC32KCTRL__R {
        OSC32KCTRL__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUPC Secure Status Locked"]
    #[inline(always)]
    pub fn supc_(&self) -> SUPC__R {
        SUPC__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GCLK Secure Status Locked"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT Secure Status Locked"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC Secure Status Locked"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EIC Secure Status Locked"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FREQM Secure Status Locked"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PORT Secure Status Locked"]
    #[inline(always)]
    pub fn port_(&self) -> PORT__R {
        PORT__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AC Secure Status Locked"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Peripheral secure status locked - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seclocka](index.html) module"]
pub struct SECLOCKA_SPEC;
impl crate::RegisterSpec for SECLOCKA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seclocka::R](R) reader structure"]
impl crate::Readable for SECLOCKA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECLOCKA to value 0"]
impl crate::Resettable for SECLOCKA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `RESCTRL` reader"]
pub struct R(crate::R<RESCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESCTRL` writer"]
pub struct W(crate::W<RESCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESCTRL_SPEC>;
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
impl From<crate::W<RESCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RES2OUT` reader - Resistor ladder To Output"]
pub type RES2OUT_R = crate::BitReader;
#[doc = "Field `RES2OUT` writer - Resistor ladder To Output"]
pub type RES2OUT_W<'a, const O: u8> = crate::BitWriter<'a, RESCTRL_SPEC, O>;
#[doc = "Field `RES1EN` reader - Resistor 1 Enable"]
pub type RES1EN_R = crate::BitReader;
#[doc = "Field `RES1EN` writer - Resistor 1 Enable"]
pub type RES1EN_W<'a, const O: u8> = crate::BitWriter<'a, RESCTRL_SPEC, O>;
#[doc = "Field `RES1MUX` reader - Resistor 1 Mux"]
pub type RES1MUX_R = crate::BitReader;
#[doc = "Field `RES1MUX` writer - Resistor 1 Mux"]
pub type RES1MUX_W<'a, const O: u8> = crate::BitWriter<'a, RESCTRL_SPEC, O>;
#[doc = "Field `POTMUX` reader - Potentiometer Selection"]
pub type POTMUX_R = crate::FieldReader;
#[doc = "Field `POTMUX` writer - Potentiometer Selection"]
pub type POTMUX_W<'a, const O: u8> = crate::FieldWriter<'a, RESCTRL_SPEC, 3, O>;
#[doc = "Field `REFBUFLEVEL` reader - Reference output voltage level select"]
pub type REFBUFLEVEL_R = crate::FieldReader;
#[doc = "Field `REFBUFLEVEL` writer - Reference output voltage level select"]
pub type REFBUFLEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, RESCTRL_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - Resistor ladder To Output"]
    #[inline(always)]
    pub fn res2out(&self) -> RES2OUT_R {
        RES2OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resistor 1 Enable"]
    #[inline(always)]
    pub fn res1en(&self) -> RES1EN_R {
        RES1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Resistor 1 Mux"]
    #[inline(always)]
    pub fn res1mux(&self) -> RES1MUX_R {
        RES1MUX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Potentiometer Selection"]
    #[inline(always)]
    pub fn potmux(&self) -> POTMUX_R {
        POTMUX_R::new((self.bits >> 3) & 7)
    }
    #[doc = "Bits 6:7 - Reference output voltage level select"]
    #[inline(always)]
    pub fn refbuflevel(&self) -> REFBUFLEVEL_R {
        REFBUFLEVEL_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Resistor ladder To Output"]
    #[inline(always)]
    #[must_use]
    pub fn res2out(&mut self) -> RES2OUT_W<0> {
        RES2OUT_W::new(self)
    }
    #[doc = "Bit 1 - Resistor 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn res1en(&mut self) -> RES1EN_W<1> {
        RES1EN_W::new(self)
    }
    #[doc = "Bit 2 - Resistor 1 Mux"]
    #[inline(always)]
    #[must_use]
    pub fn res1mux(&mut self) -> RES1MUX_W<2> {
        RES1MUX_W::new(self)
    }
    #[doc = "Bits 3:5 - Potentiometer Selection"]
    #[inline(always)]
    #[must_use]
    pub fn potmux(&mut self) -> POTMUX_W<3> {
        POTMUX_W::new(self)
    }
    #[doc = "Bits 6:7 - Reference output voltage level select"]
    #[inline(always)]
    #[must_use]
    pub fn refbuflevel(&mut self) -> REFBUFLEVEL_W<6> {
        REFBUFLEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Resister Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resctrl](index.html) module"]
pub struct RESCTRL_SPEC;
impl crate::RegisterSpec for RESCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [resctrl::R](R) reader structure"]
impl crate::Readable for RESCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resctrl::W](W) writer structure"]
impl crate::Writable for RESCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESCTRL to value 0"]
impl crate::Resettable for RESCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

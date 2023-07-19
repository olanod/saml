#[doc = "Register `DFLLULPCTRL` reader"]
pub struct R(crate::R<DFLLULPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLULPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLULPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLULPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLULPCTRL` writer"]
pub struct W(crate::W<DFLLULPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLULPCTRL_SPEC>;
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
impl From<crate::W<DFLLULPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLULPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, DFLLULPCTRL_SPEC, O>;
#[doc = "Field `BINSE` reader - Binary Search Enable"]
pub type BINSE_R = crate::BitReader;
#[doc = "Field `BINSE` writer - Binary Search Enable"]
pub type BINSE_W<'a, const O: u8> = crate::BitWriter<'a, DFLLULPCTRL_SPEC, O>;
#[doc = "Field `SAFE` reader - Tuner Safe Mode"]
pub type SAFE_R = crate::BitReader;
#[doc = "Field `SAFE` writer - Tuner Safe Mode"]
pub type SAFE_W<'a, const O: u8> = crate::BitWriter<'a, DFLLULPCTRL_SPEC, O>;
#[doc = "Field `DITHER` reader - Tuner Dither Mode"]
pub type DITHER_R = crate::BitReader;
#[doc = "Field `DITHER` writer - Tuner Dither Mode"]
pub type DITHER_W<'a, const O: u8> = crate::BitWriter<'a, DFLLULPCTRL_SPEC, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, DFLLULPCTRL_SPEC, O>;
#[doc = "Field `ONDEMAND` reader - On Demand"]
pub type ONDEMAND_R = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, DFLLULPCTRL_SPEC, O>;
#[doc = "Field `DIV` reader - Division Factor"]
pub type DIV_R = crate::FieldReader<DIVSELECT_A>;
#[doc = "Division Factor\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVSELECT_A {
    #[doc = "0: Frequency Divided by 1"]
    DIV1 = 0,
    #[doc = "1: Frequency Divided by 2"]
    DIV2 = 1,
    #[doc = "2: Frequency Divided by 4"]
    DIV4 = 2,
    #[doc = "3: Frequency Divided by 8"]
    DIV8 = 3,
    #[doc = "4: Frequency Divided by 16"]
    DIV16 = 4,
    #[doc = "5: Frequency Divided by 32"]
    DIV32 = 5,
}
impl From<DIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVSELECT_A {
    type Ux = u8;
}
impl DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVSELECT_A> {
        match self.bits {
            0 => Some(DIVSELECT_A::DIV1),
            1 => Some(DIVSELECT_A::DIV2),
            2 => Some(DIVSELECT_A::DIV4),
            3 => Some(DIVSELECT_A::DIV8),
            4 => Some(DIVSELECT_A::DIV16),
            5 => Some(DIVSELECT_A::DIV32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVSELECT_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIVSELECT_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIVSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIVSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DIVSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DIVSELECT_A::DIV32
    }
}
#[doc = "Field `DIV` writer - Division Factor"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, DFLLULPCTRL_SPEC, 3, O, DIVSELECT_A>;
impl<'a, const O: u8> DIV_W<'a, O> {
    #[doc = "Frequency Divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIV1)
    }
    #[doc = "Frequency Divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIV2)
    }
    #[doc = "Frequency Divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIV4)
    }
    #[doc = "Frequency Divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIV8)
    }
    #[doc = "Frequency Divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIV16)
    }
    #[doc = "Frequency Divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DIVSELECT_A::DIV32)
    }
}
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Binary Search Enable"]
    #[inline(always)]
    pub fn binse(&self) -> BINSE_R {
        BINSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tuner Safe Mode"]
    #[inline(always)]
    pub fn safe(&self) -> SAFE_R {
        SAFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tuner Dither Mode"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Binary Search Enable"]
    #[inline(always)]
    #[must_use]
    pub fn binse(&mut self) -> BINSE_W<3> {
        BINSE_W::new(self)
    }
    #[doc = "Bit 4 - Tuner Safe Mode"]
    #[inline(always)]
    #[must_use]
    pub fn safe(&mut self) -> SAFE_W<4> {
        SAFE_W::new(self)
    }
    #[doc = "Bit 5 - Tuner Dither Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<5> {
        DITHER_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bits 8:10 - Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<8> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLLULP Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllulpctrl](index.html) module"]
pub struct DFLLULPCTRL_SPEC;
impl crate::RegisterSpec for DFLLULPCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dfllulpctrl::R](R) reader structure"]
impl crate::Readable for DFLLULPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllulpctrl::W](W) writer structure"]
impl crate::Writable for DFLLULPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLULPCTRL to value 0x0504"]
impl crate::Resettable for DFLLULPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0504;
}

#[doc = "Register `BOD12` reader"]
pub struct R(crate::R<BOD12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD12` writer"]
pub struct W(crate::W<BOD12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD12_SPEC>;
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
impl From<crate::W<BOD12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, BOD12_SPEC, O>;
#[doc = "Field `HYST` reader - Hysteresis Enable"]
pub type HYST_R = crate::BitReader;
#[doc = "Field `HYST` writer - Hysteresis Enable"]
pub type HYST_W<'a, const O: u8> = crate::BitWriter<'a, BOD12_SPEC, O>;
#[doc = "Field `ACTION` reader - Action when Threshold Crossed"]
pub type ACTION_R = crate::FieldReader<ACTIONSELECT_A>;
#[doc = "Action when Threshold Crossed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTIONSELECT_A {
    #[doc = "0: No action"]
    NONE = 0,
    #[doc = "1: The BOD12 generates a reset"]
    RESET = 1,
    #[doc = "2: The BOD12 generates an interrupt"]
    INT = 2,
}
impl From<ACTIONSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTIONSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTIONSELECT_A {
    type Ux = u8;
}
impl ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ACTIONSELECT_A> {
        match self.bits {
            0 => Some(ACTIONSELECT_A::NONE),
            1 => Some(ACTIONSELECT_A::RESET),
            2 => Some(ACTIONSELECT_A::INT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACTIONSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ACTIONSELECT_A::RESET
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == ACTIONSELECT_A::INT
    }
}
#[doc = "Field `ACTION` writer - Action when Threshold Crossed"]
pub type ACTION_W<'a, const O: u8> = crate::FieldWriter<'a, BOD12_SPEC, 2, O, ACTIONSELECT_A>;
impl<'a, const O: u8> ACTION_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACTIONSELECT_A::NONE)
    }
    #[doc = "The BOD12 generates a reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ACTIONSELECT_A::RESET)
    }
    #[doc = "The BOD12 generates an interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(ACTIONSELECT_A::INT)
    }
}
#[doc = "Field `STDBYCFG` reader - Configuration in Standby mode"]
pub type STDBYCFG_R = crate::BitReader;
#[doc = "Field `STDBYCFG` writer - Configuration in Standby mode"]
pub type STDBYCFG_W<'a, const O: u8> = crate::BitWriter<'a, BOD12_SPEC, O>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RUNSTDBY_R = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, BOD12_SPEC, O>;
#[doc = "Field `ACTCFG` reader - Configuration in Active mode"]
pub type ACTCFG_R = crate::BitReader;
#[doc = "Field `ACTCFG` writer - Configuration in Active mode"]
pub type ACTCFG_W<'a, const O: u8> = crate::BitWriter<'a, BOD12_SPEC, O>;
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub type PSEL_R = crate::FieldReader<PSELSELECT_A>;
#[doc = "Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSELSELECT_A {
    #[doc = "0: Divide clock by 2"]
    DIV2 = 0,
    #[doc = "1: Divide clock by 4"]
    DIV4 = 1,
    #[doc = "2: Divide clock by 8"]
    DIV8 = 2,
    #[doc = "3: Divide clock by 16"]
    DIV16 = 3,
    #[doc = "4: Divide clock by 32"]
    DIV32 = 4,
    #[doc = "5: Divide clock by 64"]
    DIV64 = 5,
    #[doc = "6: Divide clock by 128"]
    DIV128 = 6,
    #[doc = "7: Divide clock by 256"]
    DIV256 = 7,
    #[doc = "8: Divide clock by 512"]
    DIV512 = 8,
    #[doc = "9: Divide clock by 1024"]
    DIV1024 = 9,
    #[doc = "10: Divide clock by 2048"]
    DIV2048 = 10,
    #[doc = "11: Divide clock by 4096"]
    DIV4096 = 11,
    #[doc = "12: Divide clock by 8192"]
    DIV8192 = 12,
    #[doc = "13: Divide clock by 16384"]
    DIV16384 = 13,
    #[doc = "14: Divide clock by 32768"]
    DIV32768 = 14,
    #[doc = "15: Divide clock by 65536"]
    DIV65536 = 15,
}
impl From<PSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSELSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSELSELECT_A {
    type Ux = u8;
}
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSELSELECT_A {
        match self.bits {
            0 => PSELSELECT_A::DIV2,
            1 => PSELSELECT_A::DIV4,
            2 => PSELSELECT_A::DIV8,
            3 => PSELSELECT_A::DIV16,
            4 => PSELSELECT_A::DIV32,
            5 => PSELSELECT_A::DIV64,
            6 => PSELSELECT_A::DIV128,
            7 => PSELSELECT_A::DIV256,
            8 => PSELSELECT_A::DIV512,
            9 => PSELSELECT_A::DIV1024,
            10 => PSELSELECT_A::DIV2048,
            11 => PSELSELECT_A::DIV4096,
            12 => PSELSELECT_A::DIV8192,
            13 => PSELSELECT_A::DIV16384,
            14 => PSELSELECT_A::DIV32768,
            15 => PSELSELECT_A::DIV65536,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PSELSELECT_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PSELSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PSELSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PSELSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PSELSELECT_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PSELSELECT_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PSELSELECT_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PSELSELECT_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PSELSELECT_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PSELSELECT_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == PSELSELECT_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == PSELSELECT_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == PSELSELECT_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == PSELSELECT_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == PSELSELECT_A::DIV32768
    }
    #[doc = "Checks if the value of the field is `DIV65536`"]
    #[inline(always)]
    pub fn is_div65536(&self) -> bool {
        *self == PSELSELECT_A::DIV65536
    }
}
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, BOD12_SPEC, 4, O, PSELSELECT_A>;
impl<'a, const O: u8> PSEL_W<'a, O> {
    #[doc = "Divide clock by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV2)
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV4)
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV8)
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV16)
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV32)
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV64)
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV128)
    }
    #[doc = "Divide clock by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV256)
    }
    #[doc = "Divide clock by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV512)
    }
    #[doc = "Divide clock by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV1024)
    }
    #[doc = "Divide clock by 2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV2048)
    }
    #[doc = "Divide clock by 4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV4096)
    }
    #[doc = "Divide clock by 8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV8192)
    }
    #[doc = "Divide clock by 16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV16384)
    }
    #[doc = "Divide clock by 32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV32768)
    }
    #[doc = "Divide clock by 65536"]
    #[inline(always)]
    pub fn div65536(self) -> &'a mut W {
        self.variant(PSELSELECT_A::DIV65536)
    }
}
#[doc = "Field `LEVEL` reader - Threshold Level"]
pub type LEVEL_R = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold Level"]
pub type LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, BOD12_SPEC, 6, O>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Action when Threshold Crossed"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Configuration in Standby mode"]
    #[inline(always)]
    pub fn stdbycfg(&self) -> STDBYCFG_R {
        STDBYCFG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Configuration in Active mode"]
    #[inline(always)]
    pub fn actcfg(&self) -> ACTCFG_R {
        ACTCFG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Hysteresis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<2> {
        HYST_W::new(self)
    }
    #[doc = "Bits 3:4 - Action when Threshold Crossed"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ACTION_W<3> {
        ACTION_W::new(self)
    }
    #[doc = "Bit 5 - Configuration in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn stdbycfg(&mut self) -> STDBYCFG_W<5> {
        STDBYCFG_W::new(self)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 8 - Configuration in Active mode"]
    #[inline(always)]
    #[must_use]
    pub fn actcfg(&mut self) -> ACTCFG_W<8> {
        ACTCFG_W::new(self)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<12> {
        PSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - Threshold Level"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LEVEL_W<16> {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD12 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod12](index.html) module"]
pub struct BOD12_SPEC;
impl crate::RegisterSpec for BOD12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod12::R](R) reader structure"]
impl crate::Readable for BOD12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod12::W](W) writer structure"]
impl crate::Writable for BOD12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD12 to value 0"]
impl crate::Resettable for BOD12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

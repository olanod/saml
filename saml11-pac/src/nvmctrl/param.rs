#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PARAM` writer"]
pub struct W(crate::W<PARAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PARAM_SPEC>;
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
impl From<crate::W<PARAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PARAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASHP` reader - FLASH Pages"]
pub type FLASHP_R = crate::FieldReader<u16>;
#[doc = "Field `FLASHP` writer - FLASH Pages"]
pub type FLASHP_W<'a, const O: u8> = crate::FieldWriter<'a, PARAM_SPEC, 16, O, u16>;
#[doc = "Field `PSZ` reader - Page Size"]
pub type PSZ_R = crate::FieldReader<PSZSELECT_A>;
#[doc = "Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSZSELECT_A {
    #[doc = "0: 8 bytes"]
    _8 = 0,
    #[doc = "1: 16 bytes"]
    _16 = 1,
    #[doc = "2: 32 bytes"]
    _32 = 2,
    #[doc = "3: 64 bytes"]
    _64 = 3,
    #[doc = "4: 128 bytes"]
    _128 = 4,
    #[doc = "5: 256 bytes"]
    _256 = 5,
    #[doc = "6: 512 bytes"]
    _512 = 6,
    #[doc = "7: 1024 bytes"]
    _1024 = 7,
}
impl From<PSZSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSZSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSZSELECT_A {
    type Ux = u8;
}
impl PSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSZSELECT_A {
        match self.bits {
            0 => PSZSELECT_A::_8,
            1 => PSZSELECT_A::_16,
            2 => PSZSELECT_A::_32,
            3 => PSZSELECT_A::_64,
            4 => PSZSELECT_A::_128,
            5 => PSZSELECT_A::_256,
            6 => PSZSELECT_A::_512,
            7 => PSZSELECT_A::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == PSZSELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == PSZSELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == PSZSELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == PSZSELECT_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == PSZSELECT_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == PSZSELECT_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == PSZSELECT_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == PSZSELECT_A::_1024
    }
}
#[doc = "Field `PSZ` writer - Page Size"]
pub type PSZ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, PARAM_SPEC, 3, O, PSZSELECT_A>;
impl<'a, const O: u8> PSZ_W<'a, O> {
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(PSZSELECT_A::_8)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(PSZSELECT_A::_16)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(PSZSELECT_A::_32)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(PSZSELECT_A::_64)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(PSZSELECT_A::_128)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(PSZSELECT_A::_256)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(PSZSELECT_A::_512)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(PSZSELECT_A::_1024)
    }
}
#[doc = "Field `DFLASHP` reader - DATAFLASH Pages"]
pub type DFLASHP_R = crate::FieldReader<u16>;
#[doc = "Field `DFLASHP` writer - DATAFLASH Pages"]
pub type DFLASHP_W<'a, const O: u8> = crate::FieldWriter<'a, PARAM_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:15 - FLASH Pages"]
    #[inline(always)]
    pub fn flashp(&self) -> FLASHP_R {
        FLASHP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:31 - DATAFLASH Pages"]
    #[inline(always)]
    pub fn dflashp(&self) -> DFLASHP_R {
        DFLASHP_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FLASH Pages"]
    #[inline(always)]
    #[must_use]
    pub fn flashp(&mut self) -> FLASHP_W<0> {
        FLASHP_W::new(self)
    }
    #[doc = "Bits 16:18 - Page Size"]
    #[inline(always)]
    #[must_use]
    pub fn psz(&mut self) -> PSZ_W<16> {
        PSZ_W::new(self)
    }
    #[doc = "Bits 20:31 - DATAFLASH Pages"]
    #[inline(always)]
    #[must_use]
    pub fn dflashp(&mut self) -> DFLASHP_W<20> {
        DFLASHP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NVM Parameter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [param::W](W) writer structure"]
impl crate::Writable for PARAM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

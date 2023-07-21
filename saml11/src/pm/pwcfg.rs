#[doc = "Register `PWCFG` reader"]
pub struct R(crate::R<PWCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWCFG` writer"]
pub struct W(crate::W<PWCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWCFG_SPEC>;
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
impl From<crate::W<PWCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMPSWC` reader - RAM Power Switch Configuration"]
pub type RAMPSWC_R = crate::FieldReader<RAMPSWCSELECT_A>;
#[doc = "RAM Power Switch Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMPSWCSELECT_A {
    #[doc = "0: 16KB Available"]
    _16KB = 0,
    #[doc = "1: 12KB Available"]
    _12KB = 1,
    #[doc = "2: 8KB Available"]
    _8KB = 2,
    #[doc = "3: 4KB Available"]
    _4KB = 3,
}
impl From<RAMPSWCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPSWCSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RAMPSWCSELECT_A {
    type Ux = u8;
}
impl RAMPSWC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMPSWCSELECT_A {
        match self.bits {
            0 => RAMPSWCSELECT_A::_16KB,
            1 => RAMPSWCSELECT_A::_12KB,
            2 => RAMPSWCSELECT_A::_8KB,
            3 => RAMPSWCSELECT_A::_4KB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16KB`"]
    #[inline(always)]
    pub fn is_16kb(&self) -> bool {
        *self == RAMPSWCSELECT_A::_16KB
    }
    #[doc = "Checks if the value of the field is `_12KB`"]
    #[inline(always)]
    pub fn is_12kb(&self) -> bool {
        *self == RAMPSWCSELECT_A::_12KB
    }
    #[doc = "Checks if the value of the field is `_8KB`"]
    #[inline(always)]
    pub fn is_8kb(&self) -> bool {
        *self == RAMPSWCSELECT_A::_8KB
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline(always)]
    pub fn is_4kb(&self) -> bool {
        *self == RAMPSWCSELECT_A::_4KB
    }
}
#[doc = "Field `RAMPSWC` writer - RAM Power Switch Configuration"]
pub type RAMPSWC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, PWCFG_SPEC, 2, O, RAMPSWCSELECT_A>;
impl<'a, const O: u8> RAMPSWC_W<'a, O> {
    #[doc = "16KB Available"]
    #[inline(always)]
    pub fn _16kb(self) -> &'a mut W {
        self.variant(RAMPSWCSELECT_A::_16KB)
    }
    #[doc = "12KB Available"]
    #[inline(always)]
    pub fn _12kb(self) -> &'a mut W {
        self.variant(RAMPSWCSELECT_A::_12KB)
    }
    #[doc = "8KB Available"]
    #[inline(always)]
    pub fn _8kb(self) -> &'a mut W {
        self.variant(RAMPSWCSELECT_A::_8KB)
    }
    #[doc = "4KB Available"]
    #[inline(always)]
    pub fn _4kb(self) -> &'a mut W {
        self.variant(RAMPSWCSELECT_A::_4KB)
    }
}
impl R {
    #[doc = "Bits 0:1 - RAM Power Switch Configuration"]
    #[inline(always)]
    pub fn rampswc(&self) -> RAMPSWC_R {
        RAMPSWC_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAM Power Switch Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rampswc(&mut self) -> RAMPSWC_W<0> {
        RAMPSWC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwcfg](index.html) module"]
pub struct PWCFG_SPEC;
impl crate::RegisterSpec for PWCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pwcfg::R](R) reader structure"]
impl crate::Readable for PWCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwcfg::W](W) writer structure"]
impl crate::Writable for PWCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWCFG to value 0"]
impl crate::Resettable for PWCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

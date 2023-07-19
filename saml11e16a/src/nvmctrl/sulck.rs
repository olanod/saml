#[doc = "Register `SULCK` reader"]
pub struct R(crate::R<SULCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SULCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SULCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SULCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SULCK` writer"]
pub struct W(crate::W<SULCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SULCK_SPEC>;
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
impl From<crate::W<SULCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SULCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BS` reader - Secure Boot Region"]
pub type BS_R = crate::BitReader;
#[doc = "Field `BS` writer - Secure Boot Region"]
pub type BS_W<'a, const O: u8> = crate::BitWriter<'a, SULCK_SPEC, O>;
#[doc = "Field `AS` reader - Secure Application Region"]
pub type AS_R = crate::BitReader;
#[doc = "Field `AS` writer - Secure Application Region"]
pub type AS_W<'a, const O: u8> = crate::BitWriter<'a, SULCK_SPEC, O>;
#[doc = "Field `DS` reader - Data Secure Region"]
pub type DS_R = crate::BitReader;
#[doc = "Field `DS` writer - Data Secure Region"]
pub type DS_W<'a, const O: u8> = crate::BitWriter<'a, SULCK_SPEC, O>;
#[doc = "Field `SLKEY` reader - Write Key"]
pub type SLKEY_R = crate::FieldReader<SLKEYSELECT_A>;
#[doc = "Write Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLKEYSELECT_A {
    #[doc = "165: Write Key"]
    KEY = 165,
}
impl From<SLKEYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SLKEYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SLKEYSELECT_A {
    type Ux = u8;
}
impl SLKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLKEYSELECT_A> {
        match self.bits {
            165 => Some(SLKEYSELECT_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == SLKEYSELECT_A::KEY
    }
}
#[doc = "Field `SLKEY` writer - Write Key"]
pub type SLKEY_W<'a, const O: u8> = crate::FieldWriter<'a, SULCK_SPEC, 8, O, SLKEYSELECT_A>;
impl<'a, const O: u8> SLKEY_W<'a, O> {
    #[doc = "Write Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(SLKEYSELECT_A::KEY)
    }
}
impl R {
    #[doc = "Bit 0 - Secure Boot Region"]
    #[inline(always)]
    pub fn bs(&self) -> BS_R {
        BS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure Application Region"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Secure Region"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    pub fn slkey(&self) -> SLKEY_R {
        SLKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Boot Region"]
    #[inline(always)]
    #[must_use]
    pub fn bs(&mut self) -> BS_W<0> {
        BS_W::new(self)
    }
    #[doc = "Bit 1 - Secure Application Region"]
    #[inline(always)]
    #[must_use]
    pub fn as_(&mut self) -> AS_W<1> {
        AS_W::new(self)
    }
    #[doc = "Bit 2 - Data Secure Region"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<2> {
        DS_W::new(self)
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    #[must_use]
    pub fn slkey(&mut self) -> SLKEY_W<8> {
        SLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sulck](index.html) module"]
pub struct SULCK_SPEC;
impl crate::RegisterSpec for SULCK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sulck::R](R) reader structure"]
impl crate::Readable for SULCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sulck::W](W) writer structure"]
impl crate::Writable for SULCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SULCK to value 0"]
impl crate::Resettable for SULCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

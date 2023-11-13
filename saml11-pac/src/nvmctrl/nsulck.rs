#[doc = "Register `NSULCK` reader"]
pub struct R(crate::R<NSULCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSULCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSULCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSULCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSULCK` writer"]
pub struct W(crate::W<NSULCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSULCK_SPEC>;
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
impl From<crate::W<NSULCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSULCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNS` reader - Non-Secure Boot Region"]
pub type BNS_R = crate::BitReader;
#[doc = "Field `BNS` writer - Non-Secure Boot Region"]
pub type BNS_W<'a, const O: u8> = crate::BitWriter<'a, NSULCK_SPEC, O>;
#[doc = "Field `ANS` reader - Non-Secure Application Region"]
pub type ANS_R = crate::BitReader;
#[doc = "Field `ANS` writer - Non-Secure Application Region"]
pub type ANS_W<'a, const O: u8> = crate::BitWriter<'a, NSULCK_SPEC, O>;
#[doc = "Field `DNS` reader - Non-Secure Data Region"]
pub type DNS_R = crate::BitReader;
#[doc = "Field `DNS` writer - Non-Secure Data Region"]
pub type DNS_W<'a, const O: u8> = crate::BitWriter<'a, NSULCK_SPEC, O>;
#[doc = "Field `NSLKEY` reader - Write Key"]
pub type NSLKEY_R = crate::FieldReader<NSLKEYSELECT_A>;
#[doc = "Write Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NSLKEYSELECT_A {
    #[doc = "165: Write Key"]
    KEY = 165,
}
impl From<NSLKEYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NSLKEYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NSLKEYSELECT_A {
    type Ux = u8;
}
impl NSLKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NSLKEYSELECT_A> {
        match self.bits {
            165 => Some(NSLKEYSELECT_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == NSLKEYSELECT_A::KEY
    }
}
#[doc = "Field `NSLKEY` writer - Write Key"]
pub type NSLKEY_W<'a, const O: u8> = crate::FieldWriter<'a, NSULCK_SPEC, 8, O, NSLKEYSELECT_A>;
impl<'a, const O: u8> NSLKEY_W<'a, O> {
    #[doc = "Write Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(NSLKEYSELECT_A::KEY)
    }
}
impl R {
    #[doc = "Bit 0 - Non-Secure Boot Region"]
    #[inline(always)]
    pub fn bns(&self) -> BNS_R {
        BNS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-Secure Application Region"]
    #[inline(always)]
    pub fn ans(&self) -> ANS_R {
        ANS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-Secure Data Region"]
    #[inline(always)]
    pub fn dns(&self) -> DNS_R {
        DNS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    pub fn nslkey(&self) -> NSLKEY_R {
        NSLKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Secure Boot Region"]
    #[inline(always)]
    #[must_use]
    pub fn bns(&mut self) -> BNS_W<0> {
        BNS_W::new(self)
    }
    #[doc = "Bit 1 - Non-Secure Application Region"]
    #[inline(always)]
    #[must_use]
    pub fn ans(&mut self) -> ANS_W<1> {
        ANS_W::new(self)
    }
    #[doc = "Bit 2 - Non-Secure Data Region"]
    #[inline(always)]
    #[must_use]
    pub fn dns(&mut self) -> DNS_W<2> {
        DNS_W::new(self)
    }
    #[doc = "Bits 8:15 - Write Key"]
    #[inline(always)]
    #[must_use]
    pub fn nslkey(&mut self) -> NSLKEY_W<8> {
        NSLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-Secure Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsulck](index.html) module"]
pub struct NSULCK_SPEC;
impl crate::RegisterSpec for NSULCK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [nsulck::R](R) reader structure"]
impl crate::Readable for NSULCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nsulck::W](W) writer structure"]
impl crate::Writable for NSULCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSULCK to value 0"]
impl crate::Resettable for NSULCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

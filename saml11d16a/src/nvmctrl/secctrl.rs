#[doc = "Register `SECCTRL` reader"]
pub struct R(crate::R<SECCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCTRL` writer"]
pub struct W(crate::W<SECCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCTRL_SPEC>;
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
impl From<crate::W<SECCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMPEEN` reader - Tamper Erase Enable"]
pub type TAMPEEN_R = crate::BitReader;
#[doc = "Field `TAMPEEN` writer - Tamper Erase Enable"]
pub type TAMPEEN_W<'a, const O: u8> = crate::BitWriter<'a, SECCTRL_SPEC, O>;
#[doc = "Field `SILACC` reader - Silent Access"]
pub type SILACC_R = crate::BitReader;
#[doc = "Field `SILACC` writer - Silent Access"]
pub type SILACC_W<'a, const O: u8> = crate::BitWriter<'a, SECCTRL_SPEC, O>;
#[doc = "Field `DSCEN` reader - Data Scramble Enable"]
pub type DSCEN_R = crate::BitReader;
#[doc = "Field `DSCEN` writer - Data Scramble Enable"]
pub type DSCEN_W<'a, const O: u8> = crate::BitWriter<'a, SECCTRL_SPEC, O>;
#[doc = "Field `DXN` reader - Data Flash is eXecute Never"]
pub type DXN_R = crate::BitReader;
#[doc = "Field `DXN` writer - Data Flash is eXecute Never"]
pub type DXN_W<'a, const O: u8> = crate::BitWriter<'a, SECCTRL_SPEC, O>;
#[doc = "Field `TEROW` reader - Tamper Rease Row"]
pub type TEROW_R = crate::FieldReader;
#[doc = "Field `TEROW` writer - Tamper Rease Row"]
pub type TEROW_W<'a, const O: u8> = crate::FieldWriter<'a, SECCTRL_SPEC, 3, O>;
#[doc = "Field `KEY` reader - Write Key"]
pub type KEY_R = crate::FieldReader<KEYSELECT_A>;
#[doc = "Write Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSELECT_A {
    #[doc = "165: Write Key"]
    KEY = 165,
}
impl From<KEYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEYSELECT_A {
    type Ux = u8;
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYSELECT_A> {
        match self.bits {
            165 => Some(KEYSELECT_A::KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY`"]
    #[inline(always)]
    pub fn is_key(&self) -> bool {
        *self == KEYSELECT_A::KEY
    }
}
#[doc = "Field `KEY` writer - Write Key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, SECCTRL_SPEC, 8, O, KEYSELECT_A>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Write Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(KEYSELECT_A::KEY)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper Erase Enable"]
    #[inline(always)]
    pub fn tampeen(&self) -> TAMPEEN_R {
        TAMPEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Silent Access"]
    #[inline(always)]
    pub fn silacc(&self) -> SILACC_R {
        SILACC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Scramble Enable"]
    #[inline(always)]
    pub fn dscen(&self) -> DSCEN_R {
        DSCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Data Flash is eXecute Never"]
    #[inline(always)]
    pub fn dxn(&self) -> DXN_R {
        DXN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper Rease Row"]
    #[inline(always)]
    pub fn terow(&self) -> TEROW_R {
        TEROW_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Erase Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampeen(&mut self) -> TAMPEEN_W<0> {
        TAMPEEN_W::new(self)
    }
    #[doc = "Bit 2 - Silent Access"]
    #[inline(always)]
    #[must_use]
    pub fn silacc(&mut self) -> SILACC_W<2> {
        SILACC_W::new(self)
    }
    #[doc = "Bit 3 - Data Scramble Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscen(&mut self) -> DSCEN_W<3> {
        DSCEN_W::new(self)
    }
    #[doc = "Bit 6 - Data Flash is eXecute Never"]
    #[inline(always)]
    #[must_use]
    pub fn dxn(&mut self) -> DXN_W<6> {
        DXN_W::new(self)
    }
    #[doc = "Bits 8:10 - Tamper Rease Row"]
    #[inline(always)]
    #[must_use]
    pub fn terow(&mut self) -> TEROW_W<8> {
        TEROW_W::new(self)
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secctrl](index.html) module"]
pub struct SECCTRL_SPEC;
impl crate::RegisterSpec for SECCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secctrl::R](R) reader structure"]
impl crate::Readable for SECCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secctrl::W](W) writer structure"]
impl crate::Writable for SECCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECCTRL to value 0x30"]
impl crate::Resettable for SECCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}

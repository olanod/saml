#[doc = "Register `WRCTRL` reader"]
pub struct R(crate::R<WRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRCTRL` writer"]
pub struct W(crate::W<WRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRCTRL_SPEC>;
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
impl From<crate::W<WRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERID` reader - Peripheral identifier"]
pub type PERID_R = crate::FieldReader<u16>;
#[doc = "Field `PERID` writer - Peripheral identifier"]
pub type PERID_W<'a, const O: u8> = crate::FieldWriter<'a, WRCTRL_SPEC, 16, O, u16>;
#[doc = "Field `KEY` reader - Peripheral access control key"]
pub type KEY_R = crate::FieldReader<KEYSELECT_A>;
#[doc = "Peripheral access control key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSELECT_A {
    #[doc = "0: No action"]
    OFF = 0,
    #[doc = "1: Clear protection"]
    CLR = 1,
    #[doc = "2: Set protection"]
    SET = 2,
    #[doc = "3: Set and lock protection"]
    SETLCK = 3,
    #[doc = "4: Set IP secure"]
    SETSEC = 4,
    #[doc = "5: Set IP non-secure"]
    SETNONSEC = 5,
    #[doc = "6: Lock IP security value"]
    SECLOCK = 6,
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
            0 => Some(KEYSELECT_A::OFF),
            1 => Some(KEYSELECT_A::CLR),
            2 => Some(KEYSELECT_A::SET),
            3 => Some(KEYSELECT_A::SETLCK),
            4 => Some(KEYSELECT_A::SETSEC),
            5 => Some(KEYSELECT_A::SETNONSEC),
            6 => Some(KEYSELECT_A::SECLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == KEYSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `CLR`"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == KEYSELECT_A::CLR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == KEYSELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `SETLCK`"]
    #[inline(always)]
    pub fn is_setlck(&self) -> bool {
        *self == KEYSELECT_A::SETLCK
    }
    #[doc = "Checks if the value of the field is `SETSEC`"]
    #[inline(always)]
    pub fn is_setsec(&self) -> bool {
        *self == KEYSELECT_A::SETSEC
    }
    #[doc = "Checks if the value of the field is `SETNONSEC`"]
    #[inline(always)]
    pub fn is_setnonsec(&self) -> bool {
        *self == KEYSELECT_A::SETNONSEC
    }
    #[doc = "Checks if the value of the field is `SECLOCK`"]
    #[inline(always)]
    pub fn is_seclock(&self) -> bool {
        *self == KEYSELECT_A::SECLOCK
    }
}
#[doc = "Field `KEY` writer - Peripheral access control key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, WRCTRL_SPEC, 8, O, KEYSELECT_A>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(KEYSELECT_A::OFF)
    }
    #[doc = "Clear protection"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut W {
        self.variant(KEYSELECT_A::CLR)
    }
    #[doc = "Set protection"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(KEYSELECT_A::SET)
    }
    #[doc = "Set and lock protection"]
    #[inline(always)]
    pub fn setlck(self) -> &'a mut W {
        self.variant(KEYSELECT_A::SETLCK)
    }
    #[doc = "Set IP secure"]
    #[inline(always)]
    pub fn setsec(self) -> &'a mut W {
        self.variant(KEYSELECT_A::SETSEC)
    }
    #[doc = "Set IP non-secure"]
    #[inline(always)]
    pub fn setnonsec(self) -> &'a mut W {
        self.variant(KEYSELECT_A::SETNONSEC)
    }
    #[doc = "Lock IP security value"]
    #[inline(always)]
    pub fn seclock(self) -> &'a mut W {
        self.variant(KEYSELECT_A::SECLOCK)
    }
}
impl R {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PERID_R {
        PERID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Peripheral identifier"]
    #[inline(always)]
    #[must_use]
    pub fn perid(&mut self) -> PERID_W<0> {
        PERID_W::new(self)
    }
    #[doc = "Bits 16:23 - Peripheral access control key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<16> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrctrl](index.html) module"]
pub struct WRCTRL_SPEC;
impl crate::RegisterSpec for WRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrctrl::R](R) reader structure"]
impl crate::Readable for WRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrctrl::W](W) writer structure"]
impl crate::Writable for WRCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRCTRL to value 0"]
impl crate::Resettable for WRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `DFLLULPDITHER` reader"]
pub struct R(crate::R<DFLLULPDITHER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLLULPDITHER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLLULPDITHER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLLULPDITHER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLLULPDITHER` writer"]
pub struct W(crate::W<DFLLULPDITHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLLULPDITHER_SPEC>;
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
impl From<crate::W<DFLLULPDITHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLLULPDITHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STEP` reader - Dither Step"]
pub type STEP_R = crate::FieldReader<STEPSELECT_A>;
#[doc = "Dither Step\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STEPSELECT_A {
    #[doc = "0: Dither Step = 1"]
    STEP1 = 0,
    #[doc = "1: Dither Step = 2"]
    STEP2 = 1,
    #[doc = "2: Dither Step = 4"]
    STEP4 = 2,
    #[doc = "3: Dither Step = 8"]
    STEP8 = 3,
}
impl From<STEPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: STEPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STEPSELECT_A {
    type Ux = u8;
}
impl STEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STEPSELECT_A> {
        match self.bits {
            0 => Some(STEPSELECT_A::STEP1),
            1 => Some(STEPSELECT_A::STEP2),
            2 => Some(STEPSELECT_A::STEP4),
            3 => Some(STEPSELECT_A::STEP8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STEP1`"]
    #[inline(always)]
    pub fn is_step1(&self) -> bool {
        *self == STEPSELECT_A::STEP1
    }
    #[doc = "Checks if the value of the field is `STEP2`"]
    #[inline(always)]
    pub fn is_step2(&self) -> bool {
        *self == STEPSELECT_A::STEP2
    }
    #[doc = "Checks if the value of the field is `STEP4`"]
    #[inline(always)]
    pub fn is_step4(&self) -> bool {
        *self == STEPSELECT_A::STEP4
    }
    #[doc = "Checks if the value of the field is `STEP8`"]
    #[inline(always)]
    pub fn is_step8(&self) -> bool {
        *self == STEPSELECT_A::STEP8
    }
}
#[doc = "Field `STEP` writer - Dither Step"]
pub type STEP_W<'a, const O: u8> = crate::FieldWriter<'a, DFLLULPDITHER_SPEC, 3, O, STEPSELECT_A>;
impl<'a, const O: u8> STEP_W<'a, O> {
    #[doc = "Dither Step = 1"]
    #[inline(always)]
    pub fn step1(self) -> &'a mut W {
        self.variant(STEPSELECT_A::STEP1)
    }
    #[doc = "Dither Step = 2"]
    #[inline(always)]
    pub fn step2(self) -> &'a mut W {
        self.variant(STEPSELECT_A::STEP2)
    }
    #[doc = "Dither Step = 4"]
    #[inline(always)]
    pub fn step4(self) -> &'a mut W {
        self.variant(STEPSELECT_A::STEP4)
    }
    #[doc = "Dither Step = 8"]
    #[inline(always)]
    pub fn step8(self) -> &'a mut W {
        self.variant(STEPSELECT_A::STEP8)
    }
}
#[doc = "Field `PER` reader - Dither Period"]
pub type PER_R = crate::FieldReader<PERSELECT_A>;
#[doc = "Dither Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERSELECT_A {
    #[doc = "0: Dither Over 1 Reference Clock Period"]
    PER1 = 0,
    #[doc = "1: Dither Over 2 Reference Clock Period"]
    PER2 = 1,
    #[doc = "2: Dither Over 4 Reference Clock Period"]
    PER4 = 2,
    #[doc = "3: Dither Over 8 Reference Clock Period"]
    PER8 = 3,
    #[doc = "4: Dither Over 16 Reference Clock Period"]
    PER16 = 4,
    #[doc = "5: Dither Over 32 Reference Clock Period"]
    PER32 = 5,
}
impl From<PERSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PERSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PERSELECT_A {
    type Ux = u8;
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERSELECT_A> {
        match self.bits {
            0 => Some(PERSELECT_A::PER1),
            1 => Some(PERSELECT_A::PER2),
            2 => Some(PERSELECT_A::PER4),
            3 => Some(PERSELECT_A::PER8),
            4 => Some(PERSELECT_A::PER16),
            5 => Some(PERSELECT_A::PER32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PER1`"]
    #[inline(always)]
    pub fn is_per1(&self) -> bool {
        *self == PERSELECT_A::PER1
    }
    #[doc = "Checks if the value of the field is `PER2`"]
    #[inline(always)]
    pub fn is_per2(&self) -> bool {
        *self == PERSELECT_A::PER2
    }
    #[doc = "Checks if the value of the field is `PER4`"]
    #[inline(always)]
    pub fn is_per4(&self) -> bool {
        *self == PERSELECT_A::PER4
    }
    #[doc = "Checks if the value of the field is `PER8`"]
    #[inline(always)]
    pub fn is_per8(&self) -> bool {
        *self == PERSELECT_A::PER8
    }
    #[doc = "Checks if the value of the field is `PER16`"]
    #[inline(always)]
    pub fn is_per16(&self) -> bool {
        *self == PERSELECT_A::PER16
    }
    #[doc = "Checks if the value of the field is `PER32`"]
    #[inline(always)]
    pub fn is_per32(&self) -> bool {
        *self == PERSELECT_A::PER32
    }
}
#[doc = "Field `PER` writer - Dither Period"]
pub type PER_W<'a, const O: u8> = crate::FieldWriter<'a, DFLLULPDITHER_SPEC, 3, O, PERSELECT_A>;
impl<'a, const O: u8> PER_W<'a, O> {
    #[doc = "Dither Over 1 Reference Clock Period"]
    #[inline(always)]
    pub fn per1(self) -> &'a mut W {
        self.variant(PERSELECT_A::PER1)
    }
    #[doc = "Dither Over 2 Reference Clock Period"]
    #[inline(always)]
    pub fn per2(self) -> &'a mut W {
        self.variant(PERSELECT_A::PER2)
    }
    #[doc = "Dither Over 4 Reference Clock Period"]
    #[inline(always)]
    pub fn per4(self) -> &'a mut W {
        self.variant(PERSELECT_A::PER4)
    }
    #[doc = "Dither Over 8 Reference Clock Period"]
    #[inline(always)]
    pub fn per8(self) -> &'a mut W {
        self.variant(PERSELECT_A::PER8)
    }
    #[doc = "Dither Over 16 Reference Clock Period"]
    #[inline(always)]
    pub fn per16(self) -> &'a mut W {
        self.variant(PERSELECT_A::PER16)
    }
    #[doc = "Dither Over 32 Reference Clock Period"]
    #[inline(always)]
    pub fn per32(self) -> &'a mut W {
        self.variant(PERSELECT_A::PER32)
    }
}
impl R {
    #[doc = "Bits 0:2 - Dither Step"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(self.bits & 7)
    }
    #[doc = "Bits 4:6 - Dither Period"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Dither Step"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<0> {
        STEP_W::new(self)
    }
    #[doc = "Bits 4:6 - Dither Period"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<4> {
        PER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLLULP Dither Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfllulpdither](index.html) module"]
pub struct DFLLULPDITHER_SPEC;
impl crate::RegisterSpec for DFLLULPDITHER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dfllulpdither::R](R) reader structure"]
impl crate::Readable for DFLLULPDITHER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfllulpdither::W](W) writer structure"]
impl crate::Writable for DFLLULPDITHER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLLULPDITHER to value 0"]
impl crate::Resettable for DFLLULPDITHER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

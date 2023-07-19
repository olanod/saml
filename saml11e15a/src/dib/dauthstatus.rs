#[doc = "Register `DAUTHSTATUS` reader"]
pub struct R(crate::R<DAUTHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAUTHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAUTHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAUTHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NSID` reader - "]
pub type NSID_R = crate::FieldReader<NSIDSELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NSIDSELECT_A {
    #[doc = "2: Non-secure invasive debug prohibited"]
    NO = 2,
    #[doc = "3: Non-secure invasive debug allowed"]
    YES = 3,
}
impl From<NSIDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NSIDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NSIDSELECT_A {
    type Ux = u8;
}
impl NSID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NSIDSELECT_A> {
        match self.bits {
            2 => Some(NSIDSELECT_A::NO),
            3 => Some(NSIDSELECT_A::YES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == NSIDSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == NSIDSELECT_A::YES
    }
}
#[doc = "Field `NSNID` reader - "]
pub type NSNID_R = crate::FieldReader<NSNIDSELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NSNIDSELECT_A {
    #[doc = "2: Non-secure non-invasive debug prohibited"]
    NO = 2,
    #[doc = "3: Non-secure non-invasive debug allowed"]
    YES = 3,
}
impl From<NSNIDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NSNIDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NSNIDSELECT_A {
    type Ux = u8;
}
impl NSNID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NSNIDSELECT_A> {
        match self.bits {
            2 => Some(NSNIDSELECT_A::NO),
            3 => Some(NSNIDSELECT_A::YES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == NSNIDSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == NSNIDSELECT_A::YES
    }
}
#[doc = "Field `SID` reader - "]
pub type SID_R = crate::FieldReader<SIDSELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIDSELECT_A {
    #[doc = "0: Security Extension not implemented"]
    NOSEC = 0,
    #[doc = "2: Secure invasive debug prohibited"]
    NO = 2,
    #[doc = "3: Secure invasive debug allowed"]
    YES = 3,
}
impl From<SIDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SIDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SIDSELECT_A {
    type Ux = u8;
}
impl SID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIDSELECT_A> {
        match self.bits {
            0 => Some(SIDSELECT_A::NOSEC),
            2 => Some(SIDSELECT_A::NO),
            3 => Some(SIDSELECT_A::YES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOSEC`"]
    #[inline(always)]
    pub fn is_nosec(&self) -> bool {
        *self == SIDSELECT_A::NOSEC
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SIDSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SIDSELECT_A::YES
    }
}
#[doc = "Field `SNID` reader - "]
pub type SNID_R = crate::FieldReader<SNIDSELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SNIDSELECT_A {
    #[doc = "0: Security Extension not implemented"]
    NOSEC = 0,
    #[doc = "2: Secure non-invasive debug prohibited"]
    NO = 2,
    #[doc = "3: Secure non-invasive debug allowed"]
    YES = 3,
}
impl From<SNIDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SNIDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SNIDSELECT_A {
    type Ux = u8;
}
impl SNID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SNIDSELECT_A> {
        match self.bits {
            0 => Some(SNIDSELECT_A::NOSEC),
            2 => Some(SNIDSELECT_A::NO),
            3 => Some(SNIDSELECT_A::YES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOSEC`"]
    #[inline(always)]
    pub fn is_nosec(&self) -> bool {
        *self == SNIDSELECT_A::NOSEC
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SNIDSELECT_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == SNIDSELECT_A::YES
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn nsid(&self) -> NSID_R {
        NSID_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn nsnid(&self) -> NSNID_R {
        NSNID_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn snid(&self) -> SNID_R {
        SNID_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[doc = "Debug Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dauthstatus](index.html) module"]
pub struct DAUTHSTATUS_SPEC;
impl crate::RegisterSpec for DAUTHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dauthstatus::R](R) reader structure"]
impl crate::Readable for DAUTHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAUTHSTATUS to value 0"]
impl crate::Resettable for DAUTHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

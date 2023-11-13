#[doc = "Register `DPLLCTRLB` reader"]
pub struct R(crate::R<DPLLCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLLCTRLB` writer"]
pub struct W(crate::W<DPLLCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLLCTRLB_SPEC>;
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
impl From<crate::W<DPLLCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLLCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTER` reader - Proportional Integral Filter Selection"]
pub type FILTER_R = crate::FieldReader<FILTERSELECT_A>;
#[doc = "Proportional Integral Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTERSELECT_A {
    #[doc = "0: Default Filter Mode"]
    DEFAULT = 0,
    #[doc = "1: Low Bandwidth Filter"]
    LBFILT = 1,
    #[doc = "2: High Bandwidth Filter"]
    HBFILT = 2,
    #[doc = "3: High Damping Filter"]
    HDFILT = 3,
}
impl From<FILTERSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTERSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FILTERSELECT_A {
    type Ux = u8;
}
impl FILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTERSELECT_A {
        match self.bits {
            0 => FILTERSELECT_A::DEFAULT,
            1 => FILTERSELECT_A::LBFILT,
            2 => FILTERSELECT_A::HBFILT,
            3 => FILTERSELECT_A::HDFILT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == FILTERSELECT_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LBFILT`"]
    #[inline(always)]
    pub fn is_lbfilt(&self) -> bool {
        *self == FILTERSELECT_A::LBFILT
    }
    #[doc = "Checks if the value of the field is `HBFILT`"]
    #[inline(always)]
    pub fn is_hbfilt(&self) -> bool {
        *self == FILTERSELECT_A::HBFILT
    }
    #[doc = "Checks if the value of the field is `HDFILT`"]
    #[inline(always)]
    pub fn is_hdfilt(&self) -> bool {
        *self == FILTERSELECT_A::HDFILT
    }
}
#[doc = "Field `FILTER` writer - Proportional Integral Filter Selection"]
pub type FILTER_W<'a, const O: u8> = crate::FieldWriterSafe<'a, DPLLCTRLB_SPEC, 2, O, FILTERSELECT_A>;
impl<'a, const O: u8> FILTER_W<'a, O> {
    #[doc = "Default Filter Mode"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(FILTERSELECT_A::DEFAULT)
    }
    #[doc = "Low Bandwidth Filter"]
    #[inline(always)]
    pub fn lbfilt(self) -> &'a mut W {
        self.variant(FILTERSELECT_A::LBFILT)
    }
    #[doc = "High Bandwidth Filter"]
    #[inline(always)]
    pub fn hbfilt(self) -> &'a mut W {
        self.variant(FILTERSELECT_A::HBFILT)
    }
    #[doc = "High Damping Filter"]
    #[inline(always)]
    pub fn hdfilt(self) -> &'a mut W {
        self.variant(FILTERSELECT_A::HDFILT)
    }
}
#[doc = "Field `LPEN` reader - Low-Power Enable"]
pub type LPEN_R = crate::BitReader;
#[doc = "Field `LPEN` writer - Low-Power Enable"]
pub type LPEN_W<'a, const O: u8> = crate::BitWriter<'a, DPLLCTRLB_SPEC, O>;
#[doc = "Field `WUF` reader - Wake Up Fast"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `WUF` writer - Wake Up Fast"]
pub type WUF_W<'a, const O: u8> = crate::BitWriter<'a, DPLLCTRLB_SPEC, O>;
#[doc = "Field `REFCLK` reader - Reference Clock Selection"]
pub type REFCLK_R = crate::FieldReader<REFCLKSELECT_A>;
#[doc = "Reference Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFCLKSELECT_A {
    #[doc = "0: XOSC32K Clock Reference"]
    XOSC32K = 0,
    #[doc = "1: XOSC Clock Reference"]
    XOSC = 1,
    #[doc = "2: GCLK Clock Reference"]
    GCLK = 2,
}
impl From<REFCLKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: REFCLKSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REFCLKSELECT_A {
    type Ux = u8;
}
impl REFCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFCLKSELECT_A> {
        match self.bits {
            0 => Some(REFCLKSELECT_A::XOSC32K),
            1 => Some(REFCLKSELECT_A::XOSC),
            2 => Some(REFCLKSELECT_A::GCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XOSC32K`"]
    #[inline(always)]
    pub fn is_xosc32k(&self) -> bool {
        *self == REFCLKSELECT_A::XOSC32K
    }
    #[doc = "Checks if the value of the field is `XOSC`"]
    #[inline(always)]
    pub fn is_xosc(&self) -> bool {
        *self == REFCLKSELECT_A::XOSC
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == REFCLKSELECT_A::GCLK
    }
}
#[doc = "Field `REFCLK` writer - Reference Clock Selection"]
pub type REFCLK_W<'a, const O: u8> = crate::FieldWriter<'a, DPLLCTRLB_SPEC, 2, O, REFCLKSELECT_A>;
impl<'a, const O: u8> REFCLK_W<'a, O> {
    #[doc = "XOSC32K Clock Reference"]
    #[inline(always)]
    pub fn xosc32k(self) -> &'a mut W {
        self.variant(REFCLKSELECT_A::XOSC32K)
    }
    #[doc = "XOSC Clock Reference"]
    #[inline(always)]
    pub fn xosc(self) -> &'a mut W {
        self.variant(REFCLKSELECT_A::XOSC)
    }
    #[doc = "GCLK Clock Reference"]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut W {
        self.variant(REFCLKSELECT_A::GCLK)
    }
}
#[doc = "Field `LTIME` reader - Lock Time"]
pub type LTIME_R = crate::FieldReader<LTIMESELECT_A>;
#[doc = "Lock Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LTIMESELECT_A {
    #[doc = "0: No time-out. Automatic lock"]
    DEFAULT = 0,
    #[doc = "4: Time-out if no lock within 8 ms"]
    _8MS = 4,
    #[doc = "5: Time-out if no lock within 9 ms"]
    _9MS = 5,
    #[doc = "6: Time-out if no lock within 10 ms"]
    _10MS = 6,
    #[doc = "7: Time-out if no lock within 11 ms"]
    _11MS = 7,
}
impl From<LTIMESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LTIMESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LTIMESELECT_A {
    type Ux = u8;
}
impl LTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LTIMESELECT_A> {
        match self.bits {
            0 => Some(LTIMESELECT_A::DEFAULT),
            4 => Some(LTIMESELECT_A::_8MS),
            5 => Some(LTIMESELECT_A::_9MS),
            6 => Some(LTIMESELECT_A::_10MS),
            7 => Some(LTIMESELECT_A::_11MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LTIMESELECT_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_8MS`"]
    #[inline(always)]
    pub fn is_8ms(&self) -> bool {
        *self == LTIMESELECT_A::_8MS
    }
    #[doc = "Checks if the value of the field is `_9MS`"]
    #[inline(always)]
    pub fn is_9ms(&self) -> bool {
        *self == LTIMESELECT_A::_9MS
    }
    #[doc = "Checks if the value of the field is `_10MS`"]
    #[inline(always)]
    pub fn is_10ms(&self) -> bool {
        *self == LTIMESELECT_A::_10MS
    }
    #[doc = "Checks if the value of the field is `_11MS`"]
    #[inline(always)]
    pub fn is_11ms(&self) -> bool {
        *self == LTIMESELECT_A::_11MS
    }
}
#[doc = "Field `LTIME` writer - Lock Time"]
pub type LTIME_W<'a, const O: u8> = crate::FieldWriter<'a, DPLLCTRLB_SPEC, 3, O, LTIMESELECT_A>;
impl<'a, const O: u8> LTIME_W<'a, O> {
    #[doc = "No time-out. Automatic lock"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LTIMESELECT_A::DEFAULT)
    }
    #[doc = "Time-out if no lock within 8 ms"]
    #[inline(always)]
    pub fn _8ms(self) -> &'a mut W {
        self.variant(LTIMESELECT_A::_8MS)
    }
    #[doc = "Time-out if no lock within 9 ms"]
    #[inline(always)]
    pub fn _9ms(self) -> &'a mut W {
        self.variant(LTIMESELECT_A::_9MS)
    }
    #[doc = "Time-out if no lock within 10 ms"]
    #[inline(always)]
    pub fn _10ms(self) -> &'a mut W {
        self.variant(LTIMESELECT_A::_10MS)
    }
    #[doc = "Time-out if no lock within 11 ms"]
    #[inline(always)]
    pub fn _11ms(self) -> &'a mut W {
        self.variant(LTIMESELECT_A::_11MS)
    }
}
#[doc = "Field `LBYPASS` reader - Lock Bypass"]
pub type LBYPASS_R = crate::BitReader;
#[doc = "Field `LBYPASS` writer - Lock Bypass"]
pub type LBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, DPLLCTRLB_SPEC, O>;
#[doc = "Field `DIV` reader - Clock Divider"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Clock Divider"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, DPLLCTRLB_SPEC, 11, O, u16>;
impl R {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refclk(&self) -> REFCLK_R {
        REFCLK_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    pub fn ltime(&self) -> LTIME_R {
        LTIME_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    pub fn lbypass(&self) -> LBYPASS_R {
        LBYPASS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Proportional Integral Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<0> {
        FILTER_W::new(self)
    }
    #[doc = "Bit 2 - Low-Power Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpen(&mut self) -> LPEN_W<2> {
        LPEN_W::new(self)
    }
    #[doc = "Bit 3 - Wake Up Fast"]
    #[inline(always)]
    #[must_use]
    pub fn wuf(&mut self) -> WUF_W<3> {
        WUF_W::new(self)
    }
    #[doc = "Bits 4:5 - Reference Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refclk(&mut self) -> REFCLK_W<4> {
        REFCLK_W::new(self)
    }
    #[doc = "Bits 8:10 - Lock Time"]
    #[inline(always)]
    #[must_use]
    pub fn ltime(&mut self) -> LTIME_W<8> {
        LTIME_W::new(self)
    }
    #[doc = "Bit 12 - Lock Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lbypass(&mut self) -> LBYPASS_W<12> {
        LBYPASS_W::new(self)
    }
    #[doc = "Bits 16:26 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<16> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DPLL Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrlb](index.html) module"]
pub struct DPLLCTRLB_SPEC;
impl crate::RegisterSpec for DPLLCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpllctrlb::R](R) reader structure"]
impl crate::Readable for DPLLCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpllctrlb::W](W) writer structure"]
impl crate::Writable for DPLLCTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPLLCTRLB to value 0"]
impl crate::Resettable for DPLLCTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

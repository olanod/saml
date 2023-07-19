#[doc = "Register `NONSECCHAN` reader"]
pub struct R(crate::R<NONSECCHAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NONSECCHAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NONSECCHAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NONSECCHAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NONSECCHAN` writer"]
pub struct W(crate::W<NONSECCHAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NONSECCHAN_SPEC>;
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
impl From<crate::W<NONSECCHAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NONSECCHAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNEL0` reader - Non-Secure for Channel 0"]
pub type CHANNEL0_R = crate::BitReader;
#[doc = "Field `CHANNEL0` writer - Non-Secure for Channel 0"]
pub type CHANNEL0_W<'a, const O: u8> = crate::BitWriter<'a, NONSECCHAN_SPEC, O>;
#[doc = "Field `CHANNEL1` reader - Non-Secure for Channel 1"]
pub type CHANNEL1_R = crate::BitReader;
#[doc = "Field `CHANNEL1` writer - Non-Secure for Channel 1"]
pub type CHANNEL1_W<'a, const O: u8> = crate::BitWriter<'a, NONSECCHAN_SPEC, O>;
#[doc = "Field `CHANNEL2` reader - Non-Secure for Channel 2"]
pub type CHANNEL2_R = crate::BitReader;
#[doc = "Field `CHANNEL2` writer - Non-Secure for Channel 2"]
pub type CHANNEL2_W<'a, const O: u8> = crate::BitWriter<'a, NONSECCHAN_SPEC, O>;
#[doc = "Field `CHANNEL3` reader - Non-Secure for Channel 3"]
pub type CHANNEL3_R = crate::BitReader;
#[doc = "Field `CHANNEL3` writer - Non-Secure for Channel 3"]
pub type CHANNEL3_W<'a, const O: u8> = crate::BitWriter<'a, NONSECCHAN_SPEC, O>;
#[doc = "Field `CHANNEL4` reader - Non-Secure for Channel 4"]
pub type CHANNEL4_R = crate::BitReader;
#[doc = "Field `CHANNEL4` writer - Non-Secure for Channel 4"]
pub type CHANNEL4_W<'a, const O: u8> = crate::BitWriter<'a, NONSECCHAN_SPEC, O>;
#[doc = "Field `CHANNEL5` reader - Non-Secure for Channel 5"]
pub type CHANNEL5_R = crate::BitReader;
#[doc = "Field `CHANNEL5` writer - Non-Secure for Channel 5"]
pub type CHANNEL5_W<'a, const O: u8> = crate::BitWriter<'a, NONSECCHAN_SPEC, O>;
#[doc = "Field `CHANNEL6` reader - Non-Secure for Channel 6"]
pub type CHANNEL6_R = crate::BitReader;
#[doc = "Field `CHANNEL6` writer - Non-Secure for Channel 6"]
pub type CHANNEL6_W<'a, const O: u8> = crate::BitWriter<'a, NONSECCHAN_SPEC, O>;
#[doc = "Field `CHANNEL7` reader - Non-Secure for Channel 7"]
pub type CHANNEL7_R = crate::BitReader;
#[doc = "Field `CHANNEL7` writer - Non-Secure for Channel 7"]
pub type CHANNEL7_W<'a, const O: u8> = crate::BitWriter<'a, NONSECCHAN_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Non-Secure for Channel 0"]
    #[inline(always)]
    pub fn channel0(&self) -> CHANNEL0_R {
        CHANNEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-Secure for Channel 1"]
    #[inline(always)]
    pub fn channel1(&self) -> CHANNEL1_R {
        CHANNEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-Secure for Channel 2"]
    #[inline(always)]
    pub fn channel2(&self) -> CHANNEL2_R {
        CHANNEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-Secure for Channel 3"]
    #[inline(always)]
    pub fn channel3(&self) -> CHANNEL3_R {
        CHANNEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Non-Secure for Channel 4"]
    #[inline(always)]
    pub fn channel4(&self) -> CHANNEL4_R {
        CHANNEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-Secure for Channel 5"]
    #[inline(always)]
    pub fn channel5(&self) -> CHANNEL5_R {
        CHANNEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non-Secure for Channel 6"]
    #[inline(always)]
    pub fn channel6(&self) -> CHANNEL6_R {
        CHANNEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Secure for Channel 7"]
    #[inline(always)]
    pub fn channel7(&self) -> CHANNEL7_R {
        CHANNEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Secure for Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn channel0(&mut self) -> CHANNEL0_W<0> {
        CHANNEL0_W::new(self)
    }
    #[doc = "Bit 1 - Non-Secure for Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn channel1(&mut self) -> CHANNEL1_W<1> {
        CHANNEL1_W::new(self)
    }
    #[doc = "Bit 2 - Non-Secure for Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn channel2(&mut self) -> CHANNEL2_W<2> {
        CHANNEL2_W::new(self)
    }
    #[doc = "Bit 3 - Non-Secure for Channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn channel3(&mut self) -> CHANNEL3_W<3> {
        CHANNEL3_W::new(self)
    }
    #[doc = "Bit 4 - Non-Secure for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn channel4(&mut self) -> CHANNEL4_W<4> {
        CHANNEL4_W::new(self)
    }
    #[doc = "Bit 5 - Non-Secure for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn channel5(&mut self) -> CHANNEL5_W<5> {
        CHANNEL5_W::new(self)
    }
    #[doc = "Bit 6 - Non-Secure for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn channel6(&mut self) -> CHANNEL6_W<6> {
        CHANNEL6_W::new(self)
    }
    #[doc = "Bit 7 - Non-Secure for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn channel7(&mut self) -> CHANNEL7_W<7> {
        CHANNEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channels Security Attribution\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nonsecchan](index.html) module"]
pub struct NONSECCHAN_SPEC;
impl crate::RegisterSpec for NONSECCHAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nonsecchan::R](R) reader structure"]
impl crate::Readable for NONSECCHAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nonsecchan::W](W) writer structure"]
impl crate::Writable for NONSECCHAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NONSECCHAN to value 0"]
impl crate::Resettable for NONSECCHAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

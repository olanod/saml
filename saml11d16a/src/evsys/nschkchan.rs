#[doc = "Register `NSCHKCHAN` reader"]
pub struct R(crate::R<NSCHKCHAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSCHKCHAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSCHKCHAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSCHKCHAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSCHKCHAN` writer"]
pub struct W(crate::W<NSCHKCHAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSCHKCHAN_SPEC>;
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
impl From<crate::W<NSCHKCHAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSCHKCHAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNEL0` reader - Channel 0 to be checked as non-secured"]
pub type CHANNEL0_R = crate::BitReader;
#[doc = "Field `CHANNEL0` writer - Channel 0 to be checked as non-secured"]
pub type CHANNEL0_W<'a, const O: u8> = crate::BitWriter<'a, NSCHKCHAN_SPEC, O>;
#[doc = "Field `CHANNEL1` reader - Channel 1 to be checked as non-secured"]
pub type CHANNEL1_R = crate::BitReader;
#[doc = "Field `CHANNEL1` writer - Channel 1 to be checked as non-secured"]
pub type CHANNEL1_W<'a, const O: u8> = crate::BitWriter<'a, NSCHKCHAN_SPEC, O>;
#[doc = "Field `CHANNEL2` reader - Channel 2 to be checked as non-secured"]
pub type CHANNEL2_R = crate::BitReader;
#[doc = "Field `CHANNEL2` writer - Channel 2 to be checked as non-secured"]
pub type CHANNEL2_W<'a, const O: u8> = crate::BitWriter<'a, NSCHKCHAN_SPEC, O>;
#[doc = "Field `CHANNEL3` reader - Channel 3 to be checked as non-secured"]
pub type CHANNEL3_R = crate::BitReader;
#[doc = "Field `CHANNEL3` writer - Channel 3 to be checked as non-secured"]
pub type CHANNEL3_W<'a, const O: u8> = crate::BitWriter<'a, NSCHKCHAN_SPEC, O>;
#[doc = "Field `CHANNEL4` reader - Channel 4 to be checked as non-secured"]
pub type CHANNEL4_R = crate::BitReader;
#[doc = "Field `CHANNEL4` writer - Channel 4 to be checked as non-secured"]
pub type CHANNEL4_W<'a, const O: u8> = crate::BitWriter<'a, NSCHKCHAN_SPEC, O>;
#[doc = "Field `CHANNEL5` reader - Channel 5 to be checked as non-secured"]
pub type CHANNEL5_R = crate::BitReader;
#[doc = "Field `CHANNEL5` writer - Channel 5 to be checked as non-secured"]
pub type CHANNEL5_W<'a, const O: u8> = crate::BitWriter<'a, NSCHKCHAN_SPEC, O>;
#[doc = "Field `CHANNEL6` reader - Channel 6 to be checked as non-secured"]
pub type CHANNEL6_R = crate::BitReader;
#[doc = "Field `CHANNEL6` writer - Channel 6 to be checked as non-secured"]
pub type CHANNEL6_W<'a, const O: u8> = crate::BitWriter<'a, NSCHKCHAN_SPEC, O>;
#[doc = "Field `CHANNEL7` reader - Channel 7 to be checked as non-secured"]
pub type CHANNEL7_R = crate::BitReader;
#[doc = "Field `CHANNEL7` writer - Channel 7 to be checked as non-secured"]
pub type CHANNEL7_W<'a, const O: u8> = crate::BitWriter<'a, NSCHKCHAN_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 to be checked as non-secured"]
    #[inline(always)]
    pub fn channel0(&self) -> CHANNEL0_R {
        CHANNEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 to be checked as non-secured"]
    #[inline(always)]
    pub fn channel1(&self) -> CHANNEL1_R {
        CHANNEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 to be checked as non-secured"]
    #[inline(always)]
    pub fn channel2(&self) -> CHANNEL2_R {
        CHANNEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 to be checked as non-secured"]
    #[inline(always)]
    pub fn channel3(&self) -> CHANNEL3_R {
        CHANNEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 to be checked as non-secured"]
    #[inline(always)]
    pub fn channel4(&self) -> CHANNEL4_R {
        CHANNEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 to be checked as non-secured"]
    #[inline(always)]
    pub fn channel5(&self) -> CHANNEL5_R {
        CHANNEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 to be checked as non-secured"]
    #[inline(always)]
    pub fn channel6(&self) -> CHANNEL6_R {
        CHANNEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 to be checked as non-secured"]
    #[inline(always)]
    pub fn channel7(&self) -> CHANNEL7_R {
        CHANNEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 to be checked as non-secured"]
    #[inline(always)]
    #[must_use]
    pub fn channel0(&mut self) -> CHANNEL0_W<0> {
        CHANNEL0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 to be checked as non-secured"]
    #[inline(always)]
    #[must_use]
    pub fn channel1(&mut self) -> CHANNEL1_W<1> {
        CHANNEL1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 to be checked as non-secured"]
    #[inline(always)]
    #[must_use]
    pub fn channel2(&mut self) -> CHANNEL2_W<2> {
        CHANNEL2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 to be checked as non-secured"]
    #[inline(always)]
    #[must_use]
    pub fn channel3(&mut self) -> CHANNEL3_W<3> {
        CHANNEL3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 to be checked as non-secured"]
    #[inline(always)]
    #[must_use]
    pub fn channel4(&mut self) -> CHANNEL4_W<4> {
        CHANNEL4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 to be checked as non-secured"]
    #[inline(always)]
    #[must_use]
    pub fn channel5(&mut self) -> CHANNEL5_W<5> {
        CHANNEL5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 to be checked as non-secured"]
    #[inline(always)]
    #[must_use]
    pub fn channel6(&mut self) -> CHANNEL6_W<6> {
        CHANNEL6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 to be checked as non-secured"]
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
#[doc = "Non-Secure Channels Check\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nschkchan](index.html) module"]
pub struct NSCHKCHAN_SPEC;
impl crate::RegisterSpec for NSCHKCHAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nschkchan::R](R) reader structure"]
impl crate::Readable for NSCHKCHAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nschkchan::W](W) writer structure"]
impl crate::Writable for NSCHKCHAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSCHKCHAN to value 0"]
impl crate::Resettable for NSCHKCHAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

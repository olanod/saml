#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER0` reader - Periodic Interval 0 Interrupt Enable"]
pub type PER0_R = crate::BitReader;
#[doc = "Field `PER0` writer - Periodic Interval 0 Interrupt Enable"]
pub type PER0_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `PER1` reader - Periodic Interval 1 Interrupt Enable"]
pub type PER1_R = crate::BitReader;
#[doc = "Field `PER1` writer - Periodic Interval 1 Interrupt Enable"]
pub type PER1_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `PER2` reader - Periodic Interval 2 Interrupt Enable"]
pub type PER2_R = crate::BitReader;
#[doc = "Field `PER2` writer - Periodic Interval 2 Interrupt Enable"]
pub type PER2_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `PER3` reader - Periodic Interval 3 Interrupt Enable"]
pub type PER3_R = crate::BitReader;
#[doc = "Field `PER3` writer - Periodic Interval 3 Interrupt Enable"]
pub type PER3_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `PER4` reader - Periodic Interval 4 Interrupt Enable"]
pub type PER4_R = crate::BitReader;
#[doc = "Field `PER4` writer - Periodic Interval 4 Interrupt Enable"]
pub type PER4_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `PER5` reader - Periodic Interval 5 Interrupt Enable"]
pub type PER5_R = crate::BitReader;
#[doc = "Field `PER5` writer - Periodic Interval 5 Interrupt Enable"]
pub type PER5_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `PER6` reader - Periodic Interval 6 Interrupt Enable"]
pub type PER6_R = crate::BitReader;
#[doc = "Field `PER6` writer - Periodic Interval 6 Interrupt Enable"]
pub type PER6_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `PER7` reader - Periodic Interval 7 Interrupt Enable"]
pub type PER7_R = crate::BitReader;
#[doc = "Field `PER7` writer - Periodic Interval 7 Interrupt Enable"]
pub type PER7_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `CMP0` reader - Compare 0 Interrupt Enable"]
pub type CMP0_R = crate::BitReader;
#[doc = "Field `CMP0` writer - Compare 0 Interrupt Enable"]
pub type CMP0_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `TAMPER` reader - Tamper Enable"]
pub type TAMPER_R = crate::BitReader;
#[doc = "Field `TAMPER` writer - Tamper Enable"]
pub type TAMPER_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, INTENSET_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Periodic Interval 0 Interrupt Enable"]
    #[inline(always)]
    pub fn per0(&self) -> PER0_R {
        PER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Interrupt Enable"]
    #[inline(always)]
    pub fn per1(&self) -> PER1_R {
        PER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Interrupt Enable"]
    #[inline(always)]
    pub fn per2(&self) -> PER2_R {
        PER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Interrupt Enable"]
    #[inline(always)]
    pub fn per3(&self) -> PER3_R {
        PER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Interrupt Enable"]
    #[inline(always)]
    pub fn per4(&self) -> PER4_R {
        PER4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Interrupt Enable"]
    #[inline(always)]
    pub fn per5(&self) -> PER5_R {
        PER5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Interrupt Enable"]
    #[inline(always)]
    pub fn per6(&self) -> PER6_R {
        PER6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Interrupt Enable"]
    #[inline(always)]
    pub fn per7(&self) -> PER7_R {
        PER7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp0(&self) -> CMP0_R {
        CMP0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - Tamper Enable"]
    #[inline(always)]
    pub fn tamper(&self) -> TAMPER_R {
        TAMPER_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interval 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per0(&mut self) -> PER0_W<0> {
        PER0_W::new(self)
    }
    #[doc = "Bit 1 - Periodic Interval 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per1(&mut self) -> PER1_W<1> {
        PER1_W::new(self)
    }
    #[doc = "Bit 2 - Periodic Interval 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per2(&mut self) -> PER2_W<2> {
        PER2_W::new(self)
    }
    #[doc = "Bit 3 - Periodic Interval 3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per3(&mut self) -> PER3_W<3> {
        PER3_W::new(self)
    }
    #[doc = "Bit 4 - Periodic Interval 4 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per4(&mut self) -> PER4_W<4> {
        PER4_W::new(self)
    }
    #[doc = "Bit 5 - Periodic Interval 5 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per5(&mut self) -> PER5_W<5> {
        PER5_W::new(self)
    }
    #[doc = "Bit 6 - Periodic Interval 6 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per6(&mut self) -> PER6_W<6> {
        PER6_W::new(self)
    }
    #[doc = "Bit 7 - Periodic Interval 7 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn per7(&mut self) -> PER7_W<7> {
        PER7_W::new(self)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> CMP0_W<8> {
        CMP0_W::new(self)
    }
    #[doc = "Bit 14 - Tamper Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamper(&mut self) -> TAMPER_W<14> {
        TAMPER_W::new(self)
    }
    #[doc = "Bit 15 - Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<15> {
        OVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE0 Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

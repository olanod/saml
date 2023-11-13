#[doc = "Register `VREGSUSP` reader"]
pub struct R(crate::R<VREGSUSP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGSUSP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREGSUSP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREGSUSP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGSUSP` writer"]
pub struct W(crate::W<VREGSUSP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGSUSP_SPEC>;
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
impl From<crate::W<VREGSUSP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREGSUSP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREGSEN` reader - Enable Voltage Regulator Suspend"]
pub type VREGSEN_R = crate::BitReader;
#[doc = "Field `VREGSEN` writer - Enable Voltage Regulator Suspend"]
pub type VREGSEN_W<'a, const O: u8> = crate::BitWriter<'a, VREGSUSP_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Enable Voltage Regulator Suspend"]
    #[inline(always)]
    pub fn vregsen(&self) -> VREGSEN_R {
        VREGSEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Voltage Regulator Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn vregsen(&mut self) -> VREGSEN_W<0> {
        VREGSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREG Suspend Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vregsusp](index.html) module"]
pub struct VREGSUSP_SPEC;
impl crate::RegisterSpec for VREGSUSP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vregsusp::R](R) reader structure"]
impl crate::Readable for VREGSUSP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vregsusp::W](W) writer structure"]
impl crate::Writable for VREGSUSP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREGSUSP to value 0"]
impl crate::Resettable for VREGSUSP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

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
#[doc = "Field `RXN` reader - CPU RAM is eXecute Never"]
pub type RXN_R = crate::BitReader;
#[doc = "Field `RXN` writer - CPU RAM is eXecute Never"]
pub type RXN_W<'a, const O: u8> = crate::BitWriter<'a, SECCTRL_SPEC, O>;
impl R {
    #[doc = "Bit 2 - CPU RAM is eXecute Never"]
    #[inline(always)]
    pub fn rxn(&self) -> RXN_R {
        RXN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - CPU RAM is eXecute Never"]
    #[inline(always)]
    #[must_use]
    pub fn rxn(&mut self) -> RXN_W<2> {
        RXN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SECCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secctrl](index.html) module"]
pub struct SECCTRL_SPEC;
impl crate::RegisterSpec for SECCTRL_SPEC {
    type Ux = u8;
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
#[doc = "`reset()` method sets SECCTRL to value 0x03"]
impl crate::Resettable for SECCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}

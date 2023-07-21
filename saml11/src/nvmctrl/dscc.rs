#[doc = "Register `DSCC` writer"]
pub struct W(crate::W<DSCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCC_SPEC>;
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
impl From<crate::W<DSCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSCKEY` writer - Data Scramble Key"]
pub type DSCKEY_W<'a, const O: u8> = crate::FieldWriter<'a, DSCC_SPEC, 30, O, u32>;
impl W {
    #[doc = "Bits 0:29 - Data Scramble Key"]
    #[inline(always)]
    #[must_use]
    pub fn dsckey(&mut self) -> DSCKEY_W<0> {
        DSCKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Scramble Configuration\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscc](index.html) module"]
pub struct DSCC_SPEC;
impl crate::RegisterSpec for DSCC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dscc::W](W) writer structure"]
impl crate::Writable for DSCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSCC to value 0"]
impl crate::Resettable for DSCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

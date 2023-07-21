#[doc = "Register `PERMW` writer"]
pub struct W(crate::W<PERMW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERMW_SPEC>;
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
impl From<crate::W<PERMW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERMW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Permutation Scrambler Data Input"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, PERMW_SPEC, 3, O>;
impl W {
    #[doc = "Bits 0:2 - Permutation Scrambler Data Input"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Permutation Write\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [permw](index.html) module"]
pub struct PERMW_SPEC;
impl crate::RegisterSpec for PERMW_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [permw::W](W) writer structure"]
impl crate::Writable for PERMW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERMW to value 0"]
impl crate::Resettable for PERMW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

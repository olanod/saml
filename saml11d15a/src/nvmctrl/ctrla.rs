#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDSELECT_AW {
    #[doc = "2: Erase Row - Erases the row addressed by the ADDR register."]
    ER = 2,
    #[doc = "4: Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    WP = 4,
    #[doc = "66: Sets the power reduction mode."]
    SPRM = 66,
    #[doc = "67: Clears the power reduction mode."]
    CPRM = 67,
    #[doc = "68: Page Buffer Clear - Clears the page buffer."]
    PBC = 68,
    #[doc = "70: Invalidate all cache lines."]
    INVALL = 70,
    #[doc = "75: Set DAL=0"]
    SDAL0 = 75,
    #[doc = "76: Set DAL=1"]
    SDAL1 = 76,
}
impl From<CMDSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMDSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `CMD` writer - Command"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, CTRLA_SPEC, 7, O, CMDSELECT_AW>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "Erase Row - Erases the row addressed by the ADDR register."]
    #[inline(always)]
    pub fn er(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::ER)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register."]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::WP)
    }
    #[doc = "Sets the power reduction mode."]
    #[inline(always)]
    pub fn sprm(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::SPRM)
    }
    #[doc = "Clears the power reduction mode."]
    #[inline(always)]
    pub fn cprm(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::CPRM)
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline(always)]
    pub fn pbc(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::PBC)
    }
    #[doc = "Invalidate all cache lines."]
    #[inline(always)]
    pub fn invall(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::INVALL)
    }
    #[doc = "Set DAL=0"]
    #[inline(always)]
    pub fn sdal0(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::SDAL0)
    }
    #[doc = "Set DAL=1"]
    #[inline(always)]
    pub fn sdal1(self) -> &'a mut W {
        self.variant(CMDSELECT_AW::SDAL1)
    }
}
#[doc = "Command Execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDEXSELECT_AW {
    #[doc = "165: Execution Key"]
    KEY = 165,
}
impl From<CMDEXSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMDEXSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMDEXSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `CMDEX` writer - Command Execution"]
pub type CMDEX_W<'a, const O: u8> = crate::FieldWriter<'a, CTRLA_SPEC, 8, O, CMDEXSELECT_AW>;
impl<'a, const O: u8> CMDEX_W<'a, O> {
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CMDEXSELECT_AW::KEY)
    }
}
impl W {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    #[must_use]
    pub fn cmdex(&mut self) -> CMDEX_W<8> {
        CMDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

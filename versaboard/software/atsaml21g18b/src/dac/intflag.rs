#[doc = "Register `INTFLAG` reader"]
pub struct R(crate::R<INTFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAG` writer"]
pub struct W(crate::W<INTFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAG_SPEC>;
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
impl From<crate::W<INTFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNDERRUN0` reader - DAC 0 Underrun"]
pub type UNDERRUN0_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN0` writer - DAC 0 Underrun"]
pub type UNDERRUN0_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
#[doc = "Field `UNDERRUN1` reader - DAC 1 Underrun"]
pub type UNDERRUN1_R = crate::BitReader<bool>;
#[doc = "Field `UNDERRUN1` writer - DAC 1 Underrun"]
pub type UNDERRUN1_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EMPTY0` reader - Data Buffer 0 Empty"]
pub type EMPTY0_R = crate::BitReader<bool>;
#[doc = "Field `EMPTY0` writer - Data Buffer 0 Empty"]
pub type EMPTY0_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
#[doc = "Field `EMPTY1` reader - Data Buffer 1 Empty"]
pub type EMPTY1_R = crate::BitReader<bool>;
#[doc = "Field `EMPTY1` writer - Data Buffer 1 Empty"]
pub type EMPTY1_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DAC 0 Underrun"]
    #[inline(always)]
    pub fn underrun0(&self) -> UNDERRUN0_R {
        UNDERRUN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC 1 Underrun"]
    #[inline(always)]
    pub fn underrun1(&self) -> UNDERRUN1_R {
        UNDERRUN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty"]
    #[inline(always)]
    pub fn empty0(&self) -> EMPTY0_R {
        EMPTY0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty"]
    #[inline(always)]
    pub fn empty1(&self) -> EMPTY1_R {
        EMPTY1_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC 0 Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn underrun0(&mut self) -> UNDERRUN0_W<0> {
        UNDERRUN0_W::new(self)
    }
    #[doc = "Bit 1 - DAC 1 Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn underrun1(&mut self) -> UNDERRUN1_W<1> {
        UNDERRUN1_W::new(self)
    }
    #[doc = "Bit 2 - Data Buffer 0 Empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty0(&mut self) -> EMPTY0_W<2> {
        EMPTY0_W::new(self)
    }
    #[doc = "Bit 3 - Data Buffer 1 Empty"]
    #[inline(always)]
    #[must_use]
    pub fn empty1(&mut self) -> EMPTY1_W<3> {
        EMPTY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflag](index.html) module"]
pub struct INTFLAG_SPEC;
impl crate::RegisterSpec for INTFLAG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intflag::R](R) reader structure"]
impl crate::Readable for INTFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflag::W](W) writer structure"]
impl crate::Writable for INTFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for INTFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

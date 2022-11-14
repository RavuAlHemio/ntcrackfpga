#[doc = "Register `DACCTRL%s` reader"]
pub struct R(crate::R<DACCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACCTRL%s` writer"]
pub struct W(crate::W<DACCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACCTRL_SPEC>;
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
impl From<crate::W<DACCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEFTADJ` reader - Left Adjusted Data"]
pub type LEFTADJ_R = crate::BitReader<bool>;
#[doc = "Field `LEFTADJ` writer - Left Adjusted Data"]
pub type LEFTADJ_W<'a, const O: u8> = crate::BitWriter<'a, u16, DACCTRL_SPEC, bool, O>;
#[doc = "Field `ENABLE` reader - Enable DAC0"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable DAC0"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DACCTRL_SPEC, bool, O>;
#[doc = "Field `CCTRL` reader - Current Control"]
pub type CCTRL_R = crate::FieldReader<u8, CCTRLSELECT_A>;
#[doc = "Current Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCTRLSELECT_A {
    #[doc = "0: GCLK_DAC <= 1.2MHz (100kSPS)"]
    CC100K = 0,
    #[doc = "1: 1.2MHz < GCLK_DAC <= 6MHz (500kSPS)"]
    CC1M = 1,
    #[doc = "2: 6MHz < GCLK_DAC <=12MHz (1MSPS)"]
    CC12M = 2,
}
impl From<CCTRLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CCTRLSELECT_A) -> Self {
        variant as _
    }
}
impl CCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCTRLSELECT_A> {
        match self.bits {
            0 => Some(CCTRLSELECT_A::CC100K),
            1 => Some(CCTRLSELECT_A::CC1M),
            2 => Some(CCTRLSELECT_A::CC12M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CC100K`"]
    #[inline(always)]
    pub fn is_cc100k(&self) -> bool {
        *self == CCTRLSELECT_A::CC100K
    }
    #[doc = "Checks if the value of the field is `CC1M`"]
    #[inline(always)]
    pub fn is_cc1m(&self) -> bool {
        *self == CCTRLSELECT_A::CC1M
    }
    #[doc = "Checks if the value of the field is `CC12M`"]
    #[inline(always)]
    pub fn is_cc12m(&self) -> bool {
        *self == CCTRLSELECT_A::CC12M
    }
}
#[doc = "Field `CCTRL` writer - Current Control"]
pub type CCTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, DACCTRL_SPEC, u8, CCTRLSELECT_A, 2, O>;
impl<'a, const O: u8> CCTRL_W<'a, O> {
    #[doc = "GCLK_DAC <= 1.2MHz (100kSPS)"]
    #[inline(always)]
    pub fn cc100k(self) -> &'a mut W {
        self.variant(CCTRLSELECT_A::CC100K)
    }
    #[doc = "1.2MHz < GCLK_DAC <= 6MHz (500kSPS)"]
    #[inline(always)]
    pub fn cc1m(self) -> &'a mut W {
        self.variant(CCTRLSELECT_A::CC1M)
    }
    #[doc = "6MHz < GCLK_DAC <=12MHz (1MSPS)"]
    #[inline(always)]
    pub fn cc12m(self) -> &'a mut W {
        self.variant(CCTRLSELECT_A::CC12M)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u16, DACCTRL_SPEC, bool, O>;
#[doc = "Field `DITHER` reader - Dithering Mode"]
pub type DITHER_R = crate::BitReader<bool>;
#[doc = "Field `DITHER` writer - Dithering Mode"]
pub type DITHER_W<'a, const O: u8> = crate::BitWriter<'a, u16, DACCTRL_SPEC, bool, O>;
#[doc = "Field `REFRESH` reader - Refresh period"]
pub type REFRESH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFRESH` writer - Refresh period"]
pub type REFRESH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DACCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline(always)]
    pub fn leftadj(&self) -> LEFTADJ_R {
        LEFTADJ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline(always)]
    pub fn cctrl(&self) -> CCTRL_R {
        CCTRL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Left Adjusted Data"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LEFTADJ_W<0> {
        LEFTADJ_W::new(self)
    }
    #[doc = "Bit 1 - Enable DAC0"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Current Control"]
    #[inline(always)]
    #[must_use]
    pub fn cctrl(&mut self) -> CCTRL_W<2> {
        CCTRL_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - Dithering Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dither(&mut self) -> DITHER_W<7> {
        DITHER_W::new(self)
    }
    #[doc = "Bits 8:11 - Refresh period"]
    #[inline(always)]
    #[must_use]
    pub fn refresh(&mut self) -> REFRESH_W<8> {
        REFRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC n Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacctrl](index.html) module"]
pub struct DACCTRL_SPEC;
impl crate::RegisterSpec for DACCTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dacctrl::R](R) reader structure"]
impl crate::Readable for DACCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacctrl::W](W) writer structure"]
impl crate::Writable for DACCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACCTRL%s to value 0"]
impl crate::Resettable for DACCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

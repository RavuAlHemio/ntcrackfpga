#[doc = "Register `PCHCTRL%s` reader"]
pub struct R(crate::R<PCHCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCHCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCHCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCHCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCHCTRL%s` writer"]
pub struct W(crate::W<PCHCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCHCTRL_SPEC>;
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
impl From<crate::W<PCHCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCHCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN` reader - Generic Clock Generator"]
pub type GEN_R = crate::FieldReader<u8, GENSELECT_A>;
#[doc = "Generic Clock Generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GENSELECT_A {
    #[doc = "0: Generic clock generator 0"]
    GCLK0 = 0,
    #[doc = "1: Generic clock generator 1"]
    GCLK1 = 1,
    #[doc = "2: Generic clock generator 2"]
    GCLK2 = 2,
    #[doc = "3: Generic clock generator 3"]
    GCLK3 = 3,
    #[doc = "4: Generic clock generator 4"]
    GCLK4 = 4,
    #[doc = "5: Generic clock generator 5"]
    GCLK5 = 5,
    #[doc = "6: Generic clock generator 6"]
    GCLK6 = 6,
    #[doc = "7: Generic clock generator 7"]
    GCLK7 = 7,
    #[doc = "8: Generic clock generator 8"]
    GCLK8 = 8,
}
impl From<GENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GENSELECT_A) -> Self {
        variant as _
    }
}
impl GEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GENSELECT_A> {
        match self.bits {
            0 => Some(GENSELECT_A::GCLK0),
            1 => Some(GENSELECT_A::GCLK1),
            2 => Some(GENSELECT_A::GCLK2),
            3 => Some(GENSELECT_A::GCLK3),
            4 => Some(GENSELECT_A::GCLK4),
            5 => Some(GENSELECT_A::GCLK5),
            6 => Some(GENSELECT_A::GCLK6),
            7 => Some(GENSELECT_A::GCLK7),
            8 => Some(GENSELECT_A::GCLK8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GCLK0`"]
    #[inline(always)]
    pub fn is_gclk0(&self) -> bool {
        *self == GENSELECT_A::GCLK0
    }
    #[doc = "Checks if the value of the field is `GCLK1`"]
    #[inline(always)]
    pub fn is_gclk1(&self) -> bool {
        *self == GENSELECT_A::GCLK1
    }
    #[doc = "Checks if the value of the field is `GCLK2`"]
    #[inline(always)]
    pub fn is_gclk2(&self) -> bool {
        *self == GENSELECT_A::GCLK2
    }
    #[doc = "Checks if the value of the field is `GCLK3`"]
    #[inline(always)]
    pub fn is_gclk3(&self) -> bool {
        *self == GENSELECT_A::GCLK3
    }
    #[doc = "Checks if the value of the field is `GCLK4`"]
    #[inline(always)]
    pub fn is_gclk4(&self) -> bool {
        *self == GENSELECT_A::GCLK4
    }
    #[doc = "Checks if the value of the field is `GCLK5`"]
    #[inline(always)]
    pub fn is_gclk5(&self) -> bool {
        *self == GENSELECT_A::GCLK5
    }
    #[doc = "Checks if the value of the field is `GCLK6`"]
    #[inline(always)]
    pub fn is_gclk6(&self) -> bool {
        *self == GENSELECT_A::GCLK6
    }
    #[doc = "Checks if the value of the field is `GCLK7`"]
    #[inline(always)]
    pub fn is_gclk7(&self) -> bool {
        *self == GENSELECT_A::GCLK7
    }
    #[doc = "Checks if the value of the field is `GCLK8`"]
    #[inline(always)]
    pub fn is_gclk8(&self) -> bool {
        *self == GENSELECT_A::GCLK8
    }
}
#[doc = "Field `GEN` writer - Generic Clock Generator"]
pub type GEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCHCTRL_SPEC, u8, GENSELECT_A, 4, O>;
impl<'a, const O: u8> GEN_W<'a, O> {
    #[doc = "Generic clock generator 0"]
    #[inline(always)]
    pub fn gclk0(self) -> &'a mut W {
        self.variant(GENSELECT_A::GCLK0)
    }
    #[doc = "Generic clock generator 1"]
    #[inline(always)]
    pub fn gclk1(self) -> &'a mut W {
        self.variant(GENSELECT_A::GCLK1)
    }
    #[doc = "Generic clock generator 2"]
    #[inline(always)]
    pub fn gclk2(self) -> &'a mut W {
        self.variant(GENSELECT_A::GCLK2)
    }
    #[doc = "Generic clock generator 3"]
    #[inline(always)]
    pub fn gclk3(self) -> &'a mut W {
        self.variant(GENSELECT_A::GCLK3)
    }
    #[doc = "Generic clock generator 4"]
    #[inline(always)]
    pub fn gclk4(self) -> &'a mut W {
        self.variant(GENSELECT_A::GCLK4)
    }
    #[doc = "Generic clock generator 5"]
    #[inline(always)]
    pub fn gclk5(self) -> &'a mut W {
        self.variant(GENSELECT_A::GCLK5)
    }
    #[doc = "Generic clock generator 6"]
    #[inline(always)]
    pub fn gclk6(self) -> &'a mut W {
        self.variant(GENSELECT_A::GCLK6)
    }
    #[doc = "Generic clock generator 7"]
    #[inline(always)]
    pub fn gclk7(self) -> &'a mut W {
        self.variant(GENSELECT_A::GCLK7)
    }
    #[doc = "Generic clock generator 8"]
    #[inline(always)]
    pub fn gclk8(self) -> &'a mut W {
        self.variant(GENSELECT_A::GCLK8)
    }
}
#[doc = "Field `CHEN` reader - Channel Enable"]
pub type CHEN_R = crate::BitReader<bool>;
#[doc = "Field `CHEN` writer - Channel Enable"]
pub type CHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCHCTRL_SPEC, bool, O>;
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WRTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WRTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCHCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Generic Clock Generator"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<0> {
        GEN_W::new(self)
    }
    #[doc = "Bit 6 - Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<6> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 7 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WRTLOCK_W<7> {
        WRTLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pchctrl](index.html) module"]
pub struct PCHCTRL_SPEC;
impl crate::RegisterSpec for PCHCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pchctrl::R](R) reader structure"]
impl crate::Readable for PCHCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pchctrl::W](W) writer structure"]
impl crate::Writable for PCHCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCHCTRL%s to value 0"]
impl crate::Resettable for PCHCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

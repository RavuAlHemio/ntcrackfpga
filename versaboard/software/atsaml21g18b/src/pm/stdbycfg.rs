#[doc = "Register `STDBYCFG` reader"]
pub struct R(crate::R<STDBYCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STDBYCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STDBYCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STDBYCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STDBYCFG` writer"]
pub struct W(crate::W<STDBYCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STDBYCFG_SPEC>;
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
impl From<crate::W<STDBYCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STDBYCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDCFG` reader - Power Domain Configuration"]
pub type PDCFG_R = crate::FieldReader<u8, PDCFGSELECT_A>;
#[doc = "Power Domain Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PDCFGSELECT_A {
    #[doc = "0: All power domains switching is handled by hardware."]
    DEFAULT = 0,
    #[doc = "1: PD0 is forced ACTIVE. PD1 and PD2 power domains switching is handled by hardware."]
    PD0 = 1,
    #[doc = "2: PD0 and PD1 are forced ACTIVE. PD2 power domain switching is handled by hardware."]
    PD01 = 2,
    #[doc = "3: All power domains are forced ACTIVE."]
    PD012 = 3,
}
impl From<PDCFGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PDCFGSELECT_A) -> Self {
        variant as _
    }
}
impl PDCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDCFGSELECT_A {
        match self.bits {
            0 => PDCFGSELECT_A::DEFAULT,
            1 => PDCFGSELECT_A::PD0,
            2 => PDCFGSELECT_A::PD01,
            3 => PDCFGSELECT_A::PD012,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == PDCFGSELECT_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `PD0`"]
    #[inline(always)]
    pub fn is_pd0(&self) -> bool {
        *self == PDCFGSELECT_A::PD0
    }
    #[doc = "Checks if the value of the field is `PD01`"]
    #[inline(always)]
    pub fn is_pd01(&self) -> bool {
        *self == PDCFGSELECT_A::PD01
    }
    #[doc = "Checks if the value of the field is `PD012`"]
    #[inline(always)]
    pub fn is_pd012(&self) -> bool {
        *self == PDCFGSELECT_A::PD012
    }
}
#[doc = "Field `PDCFG` writer - Power Domain Configuration"]
pub type PDCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, STDBYCFG_SPEC, u8, PDCFGSELECT_A, 2, O>;
impl<'a, const O: u8> PDCFG_W<'a, O> {
    #[doc = "All power domains switching is handled by hardware."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(PDCFGSELECT_A::DEFAULT)
    }
    #[doc = "PD0 is forced ACTIVE. PD1 and PD2 power domains switching is handled by hardware."]
    #[inline(always)]
    pub fn pd0(self) -> &'a mut W {
        self.variant(PDCFGSELECT_A::PD0)
    }
    #[doc = "PD0 and PD1 are forced ACTIVE. PD2 power domain switching is handled by hardware."]
    #[inline(always)]
    pub fn pd01(self) -> &'a mut W {
        self.variant(PDCFGSELECT_A::PD01)
    }
    #[doc = "All power domains are forced ACTIVE."]
    #[inline(always)]
    pub fn pd012(self) -> &'a mut W {
        self.variant(PDCFGSELECT_A::PD012)
    }
}
#[doc = "Field `DPGPD0` reader - Dynamic Power Gating for PD0"]
pub type DPGPD0_R = crate::BitReader<bool>;
#[doc = "Field `DPGPD0` writer - Dynamic Power Gating for PD0"]
pub type DPGPD0_W<'a, const O: u8> = crate::BitWriter<'a, u16, STDBYCFG_SPEC, bool, O>;
#[doc = "Field `DPGPD1` reader - Dynamic Power Gating for PD1"]
pub type DPGPD1_R = crate::BitReader<bool>;
#[doc = "Field `DPGPD1` writer - Dynamic Power Gating for PD1"]
pub type DPGPD1_W<'a, const O: u8> = crate::BitWriter<'a, u16, STDBYCFG_SPEC, bool, O>;
#[doc = "Field `VREGSMOD` reader - Voltage Regulator Standby mode"]
pub type VREGSMOD_R = crate::FieldReader<u8, VREGSMODSELECT_A>;
#[doc = "Voltage Regulator Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VREGSMODSELECT_A {
    #[doc = "0: Automatic mode"]
    AUTO = 0,
    #[doc = "1: Performance oriented"]
    PERFORMANCE = 1,
    #[doc = "2: Low Power oriented"]
    LP = 2,
}
impl From<VREGSMODSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: VREGSMODSELECT_A) -> Self {
        variant as _
    }
}
impl VREGSMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VREGSMODSELECT_A> {
        match self.bits {
            0 => Some(VREGSMODSELECT_A::AUTO),
            1 => Some(VREGSMODSELECT_A::PERFORMANCE),
            2 => Some(VREGSMODSELECT_A::LP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == VREGSMODSELECT_A::AUTO
    }
    #[doc = "Checks if the value of the field is `PERFORMANCE`"]
    #[inline(always)]
    pub fn is_performance(&self) -> bool {
        *self == VREGSMODSELECT_A::PERFORMANCE
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == VREGSMODSELECT_A::LP
    }
}
#[doc = "Field `VREGSMOD` writer - Voltage Regulator Standby mode"]
pub type VREGSMOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, STDBYCFG_SPEC, u8, VREGSMODSELECT_A, 2, O>;
impl<'a, const O: u8> VREGSMOD_W<'a, O> {
    #[doc = "Automatic mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(VREGSMODSELECT_A::AUTO)
    }
    #[doc = "Performance oriented"]
    #[inline(always)]
    pub fn performance(self) -> &'a mut W {
        self.variant(VREGSMODSELECT_A::PERFORMANCE)
    }
    #[doc = "Low Power oriented"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(VREGSMODSELECT_A::LP)
    }
}
#[doc = "Field `LINKPD` reader - Linked Power Domain"]
pub type LINKPD_R = crate::FieldReader<u8, LINKPDSELECT_A>;
#[doc = "Linked Power Domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LINKPDSELECT_A {
    #[doc = "0: Power domains are not linked"]
    DEFAULT = 0,
    #[doc = "1: PD0 and PD1 power domains are linked"]
    PD01 = 1,
    #[doc = "2: PD1 and PD2 power domains are linked"]
    PD12 = 2,
    #[doc = "3: All power domains are linked"]
    PD012 = 3,
}
impl From<LINKPDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LINKPDSELECT_A) -> Self {
        variant as _
    }
}
impl LINKPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINKPDSELECT_A {
        match self.bits {
            0 => LINKPDSELECT_A::DEFAULT,
            1 => LINKPDSELECT_A::PD01,
            2 => LINKPDSELECT_A::PD12,
            3 => LINKPDSELECT_A::PD012,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LINKPDSELECT_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `PD01`"]
    #[inline(always)]
    pub fn is_pd01(&self) -> bool {
        *self == LINKPDSELECT_A::PD01
    }
    #[doc = "Checks if the value of the field is `PD12`"]
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == LINKPDSELECT_A::PD12
    }
    #[doc = "Checks if the value of the field is `PD012`"]
    #[inline(always)]
    pub fn is_pd012(&self) -> bool {
        *self == LINKPDSELECT_A::PD012
    }
}
#[doc = "Field `LINKPD` writer - Linked Power Domain"]
pub type LINKPD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, STDBYCFG_SPEC, u8, LINKPDSELECT_A, 2, O>;
impl<'a, const O: u8> LINKPD_W<'a, O> {
    #[doc = "Power domains are not linked"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LINKPDSELECT_A::DEFAULT)
    }
    #[doc = "PD0 and PD1 power domains are linked"]
    #[inline(always)]
    pub fn pd01(self) -> &'a mut W {
        self.variant(LINKPDSELECT_A::PD01)
    }
    #[doc = "PD1 and PD2 power domains are linked"]
    #[inline(always)]
    pub fn pd12(self) -> &'a mut W {
        self.variant(LINKPDSELECT_A::PD12)
    }
    #[doc = "All power domains are linked"]
    #[inline(always)]
    pub fn pd012(self) -> &'a mut W {
        self.variant(LINKPDSELECT_A::PD012)
    }
}
#[doc = "Field `BBIASHS` reader - Back Bias for HMCRAMCHS"]
pub type BBIASHS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BBIASHS` writer - Back Bias for HMCRAMCHS"]
pub type BBIASHS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, STDBYCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `BBIASLP` reader - Back Bias for HMCRAMCLP"]
pub type BBIASLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BBIASLP` writer - Back Bias for HMCRAMCLP"]
pub type BBIASLP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, STDBYCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `BBIASPP` reader - Back Bias for PicoPram"]
pub type BBIASPP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BBIASPP` writer - Back Bias for PicoPram"]
pub type BBIASPP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, STDBYCFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Power Domain Configuration"]
    #[inline(always)]
    pub fn pdcfg(&self) -> PDCFG_R {
        PDCFG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Dynamic Power Gating for PD0"]
    #[inline(always)]
    pub fn dpgpd0(&self) -> DPGPD0_R {
        DPGPD0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dynamic Power Gating for PD1"]
    #[inline(always)]
    pub fn dpgpd1(&self) -> DPGPD1_R {
        DPGPD1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    pub fn vregsmod(&self) -> VREGSMOD_R {
        VREGSMOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Linked Power Domain"]
    #[inline(always)]
    pub fn linkpd(&self) -> LINKPD_R {
        LINKPD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Back Bias for HMCRAMCHS"]
    #[inline(always)]
    pub fn bbiashs(&self) -> BBIASHS_R {
        BBIASHS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Back Bias for HMCRAMCLP"]
    #[inline(always)]
    pub fn bbiaslp(&self) -> BBIASLP_R {
        BBIASLP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Back Bias for PicoPram"]
    #[inline(always)]
    pub fn bbiaspp(&self) -> BBIASPP_R {
        BBIASPP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power Domain Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pdcfg(&mut self) -> PDCFG_W<0> {
        PDCFG_W::new(self)
    }
    #[doc = "Bit 4 - Dynamic Power Gating for PD0"]
    #[inline(always)]
    #[must_use]
    pub fn dpgpd0(&mut self) -> DPGPD0_W<4> {
        DPGPD0_W::new(self)
    }
    #[doc = "Bit 5 - Dynamic Power Gating for PD1"]
    #[inline(always)]
    #[must_use]
    pub fn dpgpd1(&mut self) -> DPGPD1_W<5> {
        DPGPD1_W::new(self)
    }
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn vregsmod(&mut self) -> VREGSMOD_W<6> {
        VREGSMOD_W::new(self)
    }
    #[doc = "Bits 8:9 - Linked Power Domain"]
    #[inline(always)]
    #[must_use]
    pub fn linkpd(&mut self) -> LINKPD_W<8> {
        LINKPD_W::new(self)
    }
    #[doc = "Bits 10:11 - Back Bias for HMCRAMCHS"]
    #[inline(always)]
    #[must_use]
    pub fn bbiashs(&mut self) -> BBIASHS_W<10> {
        BBIASHS_W::new(self)
    }
    #[doc = "Bits 12:13 - Back Bias for HMCRAMCLP"]
    #[inline(always)]
    #[must_use]
    pub fn bbiaslp(&mut self) -> BBIASLP_W<12> {
        BBIASLP_W::new(self)
    }
    #[doc = "Bits 14:15 - Back Bias for PicoPram"]
    #[inline(always)]
    #[must_use]
    pub fn bbiaspp(&mut self) -> BBIASPP_W<14> {
        BBIASPP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standby Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stdbycfg](index.html) module"]
pub struct STDBYCFG_SPEC;
impl crate::RegisterSpec for STDBYCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [stdbycfg::R](R) reader structure"]
impl crate::Readable for STDBYCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stdbycfg::W](W) writer structure"]
impl crate::Writable for STDBYCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STDBYCFG to value 0"]
impl crate::Resettable for STDBYCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

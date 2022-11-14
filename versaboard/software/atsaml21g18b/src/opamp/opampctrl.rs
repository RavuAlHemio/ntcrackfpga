#[doc = "Register `OPAMPCTRL%s` reader"]
pub struct R(crate::R<OPAMPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMPCTRL%s` writer"]
pub struct W(crate::W<OPAMPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMPCTRL_SPEC>;
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
impl From<crate::W<OPAMPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Operational Amplifier Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Operational Amplifier Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMPCTRL_SPEC, bool, O>;
#[doc = "Field `ANAOUT` reader - Analog Output"]
pub type ANAOUT_R = crate::BitReader<bool>;
#[doc = "Field `ANAOUT` writer - Analog Output"]
pub type ANAOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMPCTRL_SPEC, bool, O>;
#[doc = "Field `BIAS` reader - Bias Selection"]
pub type BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIAS` writer - Bias Selection"]
pub type BIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMPCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMPCTRL_SPEC, bool, O>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type ONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type ONDEMAND_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMPCTRL_SPEC, bool, O>;
#[doc = "Field `RES2OUT` reader - Resistor ladder To Output"]
pub type RES2OUT_R = crate::BitReader<bool>;
#[doc = "Field `RES2OUT` writer - Resistor ladder To Output"]
pub type RES2OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMPCTRL_SPEC, bool, O>;
#[doc = "Field `RES2VCC` reader - Resistor ladder To VCC"]
pub type RES2VCC_R = crate::BitReader<bool>;
#[doc = "Field `RES2VCC` writer - Resistor ladder To VCC"]
pub type RES2VCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMPCTRL_SPEC, bool, O>;
#[doc = "Field `RES1EN` reader - Resistor 1 Enable"]
pub type RES1EN_R = crate::BitReader<bool>;
#[doc = "Field `RES1EN` writer - Resistor 1 Enable"]
pub type RES1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMPCTRL_SPEC, bool, O>;
#[doc = "Field `RES1MUX` reader - Resistor 1 Mux"]
pub type RES1MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RES1MUX` writer - Resistor 1 Mux"]
pub type RES1MUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMPCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `POTMUX` reader - Potentiometer Selection"]
pub type POTMUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POTMUX` writer - Potentiometer Selection"]
pub type POTMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMPCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `MUXPOS` reader - Positive Input Mux Selection"]
pub type MUXPOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUXPOS` writer - Positive Input Mux Selection"]
pub type MUXPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMPCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `MUXNEG` reader - Negative Input Mux Selection"]
pub type MUXNEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MUXNEG` writer - Negative Input Mux Selection"]
pub type MUXNEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMPCTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 1 - Operational Amplifier Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Output"]
    #[inline(always)]
    pub fn anaout(&self) -> ANAOUT_R {
        ANAOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Bias Selection"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Resistor ladder To Output"]
    #[inline(always)]
    pub fn res2out(&self) -> RES2OUT_R {
        RES2OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Resistor ladder To VCC"]
    #[inline(always)]
    pub fn res2vcc(&self) -> RES2VCC_R {
        RES2VCC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Resistor 1 Enable"]
    #[inline(always)]
    pub fn res1en(&self) -> RES1EN_R {
        RES1EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Resistor 1 Mux"]
    #[inline(always)]
    pub fn res1mux(&self) -> RES1MUX_R {
        RES1MUX_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - Potentiometer Selection"]
    #[inline(always)]
    pub fn potmux(&self) -> POTMUX_R {
        POTMUX_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Positive Input Mux Selection"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Negative Input Mux Selection"]
    #[inline(always)]
    pub fn muxneg(&self) -> MUXNEG_R {
        MUXNEG_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Operational Amplifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<1> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Analog Output"]
    #[inline(always)]
    #[must_use]
    pub fn anaout(&mut self) -> ANAOUT_W<2> {
        ANAOUT_W::new(self)
    }
    #[doc = "Bits 3:4 - Bias Selection"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BIAS_W<3> {
        BIAS_W::new(self)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<6> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> ONDEMAND_W<7> {
        ONDEMAND_W::new(self)
    }
    #[doc = "Bit 8 - Resistor ladder To Output"]
    #[inline(always)]
    #[must_use]
    pub fn res2out(&mut self) -> RES2OUT_W<8> {
        RES2OUT_W::new(self)
    }
    #[doc = "Bit 9 - Resistor ladder To VCC"]
    #[inline(always)]
    #[must_use]
    pub fn res2vcc(&mut self) -> RES2VCC_W<9> {
        RES2VCC_W::new(self)
    }
    #[doc = "Bit 10 - Resistor 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn res1en(&mut self) -> RES1EN_W<10> {
        RES1EN_W::new(self)
    }
    #[doc = "Bits 11:12 - Resistor 1 Mux"]
    #[inline(always)]
    #[must_use]
    pub fn res1mux(&mut self) -> RES1MUX_W<11> {
        RES1MUX_W::new(self)
    }
    #[doc = "Bits 13:15 - Potentiometer Selection"]
    #[inline(always)]
    #[must_use]
    pub fn potmux(&mut self) -> POTMUX_W<13> {
        POTMUX_W::new(self)
    }
    #[doc = "Bits 16:18 - Positive Input Mux Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxpos(&mut self) -> MUXPOS_W<16> {
        MUXPOS_W::new(self)
    }
    #[doc = "Bits 20:22 - Negative Input Mux Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxneg(&mut self) -> MUXNEG_W<20> {
        MUXNEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP n Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opampctrl](index.html) module"]
pub struct OPAMPCTRL_SPEC;
impl crate::RegisterSpec for OPAMPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opampctrl::R](R) reader structure"]
impl crate::Readable for OPAMPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opampctrl::W](W) writer structure"]
impl crate::Writable for OPAMPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPAMPCTRL%s to value 0"]
impl crate::Resettable for OPAMPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

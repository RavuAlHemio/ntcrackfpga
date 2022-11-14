#[doc = "Register `APBDMASK` reader"]
pub struct R(crate::R<APBDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBDMASK` writer"]
pub struct W(crate::W<APBDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBDMASK_SPEC>;
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
impl From<crate::W<APBDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub type EVSYS__R = crate::BitReader<bool>;
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub type EVSYS__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Clock Enable"]
pub type SERCOM5__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM5_` writer - SERCOM5 APB Clock Enable"]
pub type SERCOM5__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `TC4_` reader - TC4 APB Clock Enable"]
pub type TC4__R = crate::BitReader<bool>;
#[doc = "Field `TC4_` writer - TC4 APB Clock Enable"]
pub type TC4__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `ADC_` reader - ADC APB Clock Enable"]
pub type ADC__R = crate::BitReader<bool>;
#[doc = "Field `ADC_` writer - ADC APB Clock Enable"]
pub type ADC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type AC__R = crate::BitReader<bool>;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type AC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `PTC_` reader - PTC APB Clock Enable"]
pub type PTC__R = crate::BitReader<bool>;
#[doc = "Field `PTC_` writer - PTC APB Clock Enable"]
pub type PTC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `OPAMP_` reader - OPAMP APB Clock Enable"]
pub type OPAMP__R = crate::BitReader<bool>;
#[doc = "Field `OPAMP_` writer - OPAMP APB Clock Enable"]
pub type OPAMP__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
#[doc = "Field `CCL_` reader - CCL APB Clock Enable"]
pub type CCL__R = crate::BitReader<bool>;
#[doc = "Field `CCL_` writer - CCL APB Clock Enable"]
pub type CCL__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBDMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TC4 APB Clock Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC APB Clock Enable"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OPAMP APB Clock Enable"]
    #[inline(always)]
    pub fn opamp_(&self) -> OPAMP__R {
        OPAMP__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> EVSYS__W<0> {
        EVSYS__W::new(self)
    }
    #[doc = "Bit 1 - SERCOM5 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom5_(&mut self) -> SERCOM5__W<1> {
        SERCOM5__W::new(self)
    }
    #[doc = "Bit 2 - TC4 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc4_(&mut self) -> TC4__W<2> {
        TC4__W::new(self)
    }
    #[doc = "Bit 3 - ADC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_(&mut self) -> ADC__W<3> {
        ADC__W::new(self)
    }
    #[doc = "Bit 4 - AC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac_(&mut self) -> AC__W<4> {
        AC__W::new(self)
    }
    #[doc = "Bit 5 - PTC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ptc_(&mut self) -> PTC__W<5> {
        PTC__W::new(self)
    }
    #[doc = "Bit 6 - OPAMP APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opamp_(&mut self) -> OPAMP__W<6> {
        OPAMP__W::new(self)
    }
    #[doc = "Bit 7 - CCL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccl_(&mut self) -> CCL__W<7> {
        CCL__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBD Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbdmask](index.html) module"]
pub struct APBDMASK_SPEC;
impl crate::RegisterSpec for APBDMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbdmask::R](R) reader structure"]
impl crate::Readable for APBDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbdmask::W](W) writer structure"]
impl crate::Writable for APBDMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBDMASK to value 0xff"]
impl crate::Resettable for APBDMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}

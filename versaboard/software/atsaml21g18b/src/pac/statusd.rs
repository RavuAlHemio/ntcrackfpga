#[doc = "Register `STATUSD` reader"]
pub struct R(crate::R<STATUSD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVSYS_` reader - EVSYS APB Protect Enable"]
pub type EVSYS__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Protect Enable"]
pub type SERCOM5__R = crate::BitReader<bool>;
#[doc = "Field `TC4_` reader - TC4 APB Protect Enable"]
pub type TC4__R = crate::BitReader<bool>;
#[doc = "Field `ADC_` reader - ADC APB Protect Enable"]
pub type ADC__R = crate::BitReader<bool>;
#[doc = "Field `AC_` reader - AC APB Protect Enable"]
pub type AC__R = crate::BitReader<bool>;
#[doc = "Field `PTC_` reader - PTC APB Protect Enable"]
pub type PTC__R = crate::BitReader<bool>;
#[doc = "Field `OPAMP_` reader - OPAMP APB Protect Enable"]
pub type OPAMP__R = crate::BitReader<bool>;
#[doc = "Field `CCL_` reader - CCL APB Protect Enable"]
pub type CCL__R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> EVSYS__R {
        EVSYS__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM5 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> SERCOM5__R {
        SERCOM5__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC APB Protect Enable"]
    #[inline(always)]
    pub fn adc_(&self) -> ADC__R {
        ADC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AC APB Protect Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PTC APB Protect Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> PTC__R {
        PTC__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OPAMP APB Protect Enable"]
    #[inline(always)]
    pub fn opamp_(&self) -> OPAMP__R {
        OPAMP__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CCL APB Protect Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge D\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusd](index.html) module"]
pub struct STATUSD_SPEC;
impl crate::RegisterSpec for STATUSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusd::R](R) reader structure"]
impl crate::Readable for STATUSD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSD to value 0"]
impl crate::Resettable for STATUSD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

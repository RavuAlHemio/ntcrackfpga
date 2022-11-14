#[doc = "Register `STATUSC` reader"]
pub struct R(crate::R<STATUSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Protect Enable"]
pub type SERCOM0__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Protect Enable"]
pub type SERCOM1__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Protect Enable"]
pub type SERCOM2__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Protect Enable"]
pub type SERCOM3__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Protect Enable"]
pub type SERCOM4__R = crate::BitReader<bool>;
#[doc = "Field `TCC0_` reader - TCC0 APB Protect Enable"]
pub type TCC0__R = crate::BitReader<bool>;
#[doc = "Field `TCC1_` reader - TCC1 APB Protect Enable"]
pub type TCC1__R = crate::BitReader<bool>;
#[doc = "Field `TCC2_` reader - TCC2 APB Protect Enable"]
pub type TCC2__R = crate::BitReader<bool>;
#[doc = "Field `TC0_` reader - TC0 APB Protect Enable"]
pub type TC0__R = crate::BitReader<bool>;
#[doc = "Field `TC1_` reader - TC1 APB Protect Enable"]
pub type TC1__R = crate::BitReader<bool>;
#[doc = "Field `DAC_` reader - DAC APB Protect Enable"]
pub type DAC__R = crate::BitReader<bool>;
#[doc = "Field `AES_` reader - AES APB Protect Enable"]
pub type AES__R = crate::BitReader<bool>;
#[doc = "Field `TRNG_` reader - TRNG APB Protect Enable"]
pub type TRNG__R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SERCOM0 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM1 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM2 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM3 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SERCOM4 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TCC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AES APB Protect Enable"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TRNG APB Protect Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statusc](index.html) module"]
pub struct STATUSC_SPEC;
impl crate::RegisterSpec for STATUSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statusc::R](R) reader structure"]
impl crate::Readable for STATUSC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for STATUSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

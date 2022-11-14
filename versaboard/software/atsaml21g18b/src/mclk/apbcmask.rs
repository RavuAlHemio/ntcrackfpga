#[doc = "Register `APBCMASK` reader"]
pub struct R(crate::R<APBCMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBCMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBCMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBCMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBCMASK` writer"]
pub struct W(crate::W<APBCMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBCMASK_SPEC>;
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
impl From<crate::W<APBCMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBCMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Clock Enable"]
pub type SERCOM0__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM0_` writer - SERCOM0 APB Clock Enable"]
pub type SERCOM0__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Clock Enable"]
pub type SERCOM1__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM1_` writer - SERCOM1 APB Clock Enable"]
pub type SERCOM1__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub type SERCOM2__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub type SERCOM2__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Clock Enable"]
pub type SERCOM3__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM3_` writer - SERCOM3 APB Clock Enable"]
pub type SERCOM3__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Clock Enable"]
pub type SERCOM4__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM4_` writer - SERCOM4 APB Clock Enable"]
pub type SERCOM4__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TCC0_` reader - TCC0 APB Clock Enable"]
pub type TCC0__R = crate::BitReader<bool>;
#[doc = "Field `TCC0_` writer - TCC0 APB Clock Enable"]
pub type TCC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TCC1_` reader - TCC1 APB Clock Enable"]
pub type TCC1__R = crate::BitReader<bool>;
#[doc = "Field `TCC1_` writer - TCC1 APB Clock Enable"]
pub type TCC1__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TCC2_` reader - TCC2 APB Clock Enable"]
pub type TCC2__R = crate::BitReader<bool>;
#[doc = "Field `TCC2_` writer - TCC2 APB Clock Enable"]
pub type TCC2__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TC0_` reader - TC0 APB Clock Enable"]
pub type TC0__R = crate::BitReader<bool>;
#[doc = "Field `TC0_` writer - TC0 APB Clock Enable"]
pub type TC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub type TC1__R = crate::BitReader<bool>;
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub type TC1__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `DAC_` reader - DAC APB Clock Enable"]
pub type DAC__R = crate::BitReader<bool>;
#[doc = "Field `DAC_` writer - DAC APB Clock Enable"]
pub type DAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `AES_` reader - AES APB Clock Enable"]
pub type AES__R = crate::BitReader<bool>;
#[doc = "Field `AES_` writer - AES APB Clock Enable"]
pub type AES__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
#[doc = "Field `TRNG_` reader - TRNG APB Clock Enable"]
pub type TRNG__R = crate::BitReader<bool>;
#[doc = "Field `TRNG_` writer - TRNG APB Clock Enable"]
pub type TRNG__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBCMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AES APB Clock Enable"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom0_(&mut self) -> SERCOM0__W<0> {
        SERCOM0__W::new(self)
    }
    #[doc = "Bit 1 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom1_(&mut self) -> SERCOM1__W<1> {
        SERCOM1__W::new(self)
    }
    #[doc = "Bit 2 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> SERCOM2__W<2> {
        SERCOM2__W::new(self)
    }
    #[doc = "Bit 3 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom3_(&mut self) -> SERCOM3__W<3> {
        SERCOM3__W::new(self)
    }
    #[doc = "Bit 4 - SERCOM4 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom4_(&mut self) -> SERCOM4__W<4> {
        SERCOM4__W::new(self)
    }
    #[doc = "Bit 5 - TCC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc0_(&mut self) -> TCC0__W<5> {
        TCC0__W::new(self)
    }
    #[doc = "Bit 6 - TCC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc1_(&mut self) -> TCC1__W<6> {
        TCC1__W::new(self)
    }
    #[doc = "Bit 7 - TCC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc2_(&mut self) -> TCC2__W<7> {
        TCC2__W::new(self)
    }
    #[doc = "Bit 8 - TC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc0_(&mut self) -> TC0__W<8> {
        TC0__W::new(self)
    }
    #[doc = "Bit 9 - TC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc1_(&mut self) -> TC1__W<9> {
        TC1__W::new(self)
    }
    #[doc = "Bit 12 - DAC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_(&mut self) -> DAC__W<12> {
        DAC__W::new(self)
    }
    #[doc = "Bit 13 - AES APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aes_(&mut self) -> AES__W<13> {
        AES__W::new(self)
    }
    #[doc = "Bit 14 - TRNG APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trng_(&mut self) -> TRNG__W<14> {
        TRNG__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBC Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcmask](index.html) module"]
pub struct APBCMASK_SPEC;
impl crate::RegisterSpec for APBCMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbcmask::R](R) reader structure"]
impl crate::Readable for APBCMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbcmask::W](W) writer structure"]
impl crate::Writable for APBCMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBCMASK to value 0x7fff"]
impl crate::Resettable for APBCMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff;
}

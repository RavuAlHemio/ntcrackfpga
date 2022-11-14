#[doc = "Register `INTFLAGC` reader"]
pub struct R(crate::R<INTFLAGC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGC` writer"]
pub struct W(crate::W<INTFLAGC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGC_SPEC>;
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
impl From<crate::W<INTFLAGC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERCOM0_` reader - SERCOM0"]
pub type SERCOM0__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM0_` writer - SERCOM0"]
pub type SERCOM0__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `SERCOM1_` reader - SERCOM1"]
pub type SERCOM1__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM1_` writer - SERCOM1"]
pub type SERCOM1__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `SERCOM2_` reader - SERCOM2"]
pub type SERCOM2__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM2_` writer - SERCOM2"]
pub type SERCOM2__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `SERCOM3_` reader - SERCOM3"]
pub type SERCOM3__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM3_` writer - SERCOM3"]
pub type SERCOM3__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `SERCOM4_` reader - SERCOM4"]
pub type SERCOM4__R = crate::BitReader<bool>;
#[doc = "Field `SERCOM4_` writer - SERCOM4"]
pub type SERCOM4__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `TCC0_` reader - TCC0"]
pub type TCC0__R = crate::BitReader<bool>;
#[doc = "Field `TCC0_` writer - TCC0"]
pub type TCC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `TCC1_` reader - TCC1"]
pub type TCC1__R = crate::BitReader<bool>;
#[doc = "Field `TCC1_` writer - TCC1"]
pub type TCC1__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `TCC2_` reader - TCC2"]
pub type TCC2__R = crate::BitReader<bool>;
#[doc = "Field `TCC2_` writer - TCC2"]
pub type TCC2__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `TC0_` reader - TC0"]
pub type TC0__R = crate::BitReader<bool>;
#[doc = "Field `TC0_` writer - TC0"]
pub type TC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `TC1_` reader - TC1"]
pub type TC1__R = crate::BitReader<bool>;
#[doc = "Field `TC1_` writer - TC1"]
pub type TC1__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `DAC_` reader - DAC"]
pub type DAC__R = crate::BitReader<bool>;
#[doc = "Field `DAC_` writer - DAC"]
pub type DAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `AES_` reader - AES"]
pub type AES__R = crate::BitReader<bool>;
#[doc = "Field `AES_` writer - AES"]
pub type AES__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
#[doc = "Field `TRNG_` reader - TRNG"]
pub type TRNG__R = crate::BitReader<bool>;
#[doc = "Field `TRNG_` writer - TRNG"]
pub type TRNG__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SERCOM0"]
    #[inline(always)]
    pub fn sercom0_(&self) -> SERCOM0__R {
        SERCOM0__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM1"]
    #[inline(always)]
    pub fn sercom1_(&self) -> SERCOM1__R {
        SERCOM1__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM2"]
    #[inline(always)]
    pub fn sercom2_(&self) -> SERCOM2__R {
        SERCOM2__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM3"]
    #[inline(always)]
    pub fn sercom3_(&self) -> SERCOM3__R {
        SERCOM3__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SERCOM4"]
    #[inline(always)]
    pub fn sercom4_(&self) -> SERCOM4__R {
        SERCOM4__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TCC0"]
    #[inline(always)]
    pub fn tcc0_(&self) -> TCC0__R {
        TCC0__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCC1"]
    #[inline(always)]
    pub fn tcc1_(&self) -> TCC1__R {
        TCC1__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCC2"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TC0"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TC1"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - DAC"]
    #[inline(always)]
    pub fn dac_(&self) -> DAC__R {
        DAC__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AES"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TRNG"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM0"]
    #[inline(always)]
    #[must_use]
    pub fn sercom0_(&mut self) -> SERCOM0__W<0> {
        SERCOM0__W::new(self)
    }
    #[doc = "Bit 1 - SERCOM1"]
    #[inline(always)]
    #[must_use]
    pub fn sercom1_(&mut self) -> SERCOM1__W<1> {
        SERCOM1__W::new(self)
    }
    #[doc = "Bit 2 - SERCOM2"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> SERCOM2__W<2> {
        SERCOM2__W::new(self)
    }
    #[doc = "Bit 3 - SERCOM3"]
    #[inline(always)]
    #[must_use]
    pub fn sercom3_(&mut self) -> SERCOM3__W<3> {
        SERCOM3__W::new(self)
    }
    #[doc = "Bit 4 - SERCOM4"]
    #[inline(always)]
    #[must_use]
    pub fn sercom4_(&mut self) -> SERCOM4__W<4> {
        SERCOM4__W::new(self)
    }
    #[doc = "Bit 5 - TCC0"]
    #[inline(always)]
    #[must_use]
    pub fn tcc0_(&mut self) -> TCC0__W<5> {
        TCC0__W::new(self)
    }
    #[doc = "Bit 6 - TCC1"]
    #[inline(always)]
    #[must_use]
    pub fn tcc1_(&mut self) -> TCC1__W<6> {
        TCC1__W::new(self)
    }
    #[doc = "Bit 7 - TCC2"]
    #[inline(always)]
    #[must_use]
    pub fn tcc2_(&mut self) -> TCC2__W<7> {
        TCC2__W::new(self)
    }
    #[doc = "Bit 8 - TC0"]
    #[inline(always)]
    #[must_use]
    pub fn tc0_(&mut self) -> TC0__W<8> {
        TC0__W::new(self)
    }
    #[doc = "Bit 9 - TC1"]
    #[inline(always)]
    #[must_use]
    pub fn tc1_(&mut self) -> TC1__W<9> {
        TC1__W::new(self)
    }
    #[doc = "Bit 12 - DAC"]
    #[inline(always)]
    #[must_use]
    pub fn dac_(&mut self) -> DAC__W<12> {
        DAC__W::new(self)
    }
    #[doc = "Bit 13 - AES"]
    #[inline(always)]
    #[must_use]
    pub fn aes_(&mut self) -> AES__W<13> {
        AES__W::new(self)
    }
    #[doc = "Bit 14 - TRNG"]
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
#[doc = "Peripheral interrupt flag status - Bridge C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagc](index.html) module"]
pub struct INTFLAGC_SPEC;
impl crate::RegisterSpec for INTFLAGC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagc::R](R) reader structure"]
impl crate::Readable for INTFLAGC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagc::W](W) writer structure"]
impl crate::Writable for INTFLAGC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGC to value 0"]
impl crate::Resettable for INTFLAGC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

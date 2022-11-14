#[doc = "Register `INTFLAGAHB` reader"]
pub struct R(crate::R<INTFLAGAHB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGAHB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGAHB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGAHB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGAHB` writer"]
pub struct W(crate::W<INTFLAGAHB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGAHB_SPEC>;
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
impl From<crate::W<INTFLAGAHB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGAHB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_` reader - FLASH"]
pub type FLASH__R = crate::BitReader<bool>;
#[doc = "Field `FLASH_` writer - FLASH"]
pub type FLASH__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HSRAMCM0P_` reader - HSRAMCM0P"]
pub type HSRAMCM0P__R = crate::BitReader<bool>;
#[doc = "Field `HSRAMCM0P_` writer - HSRAMCM0P"]
pub type HSRAMCM0P__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HSRAMDSU_` reader - HSRAMDSU"]
pub type HSRAMDSU__R = crate::BitReader<bool>;
#[doc = "Field `HSRAMDSU_` writer - HSRAMDSU"]
pub type HSRAMDSU__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HPB1_` reader - HPB1"]
pub type HPB1__R = crate::BitReader<bool>;
#[doc = "Field `HPB1_` writer - HPB1"]
pub type HPB1__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `H2LBRIDGES_` reader - H2LBRIDGES"]
pub type H2LBRIDGES__R = crate::BitReader<bool>;
#[doc = "Field `H2LBRIDGES_` writer - H2LBRIDGES"]
pub type H2LBRIDGES__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HPB0_` reader - HPB0"]
pub type HPB0__R = crate::BitReader<bool>;
#[doc = "Field `HPB0_` writer - HPB0"]
pub type HPB0__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HPB2_` reader - HPB2"]
pub type HPB2__R = crate::BitReader<bool>;
#[doc = "Field `HPB2_` writer - HPB2"]
pub type HPB2__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HPB3_` reader - HPB3"]
pub type HPB3__R = crate::BitReader<bool>;
#[doc = "Field `HPB3_` writer - HPB3"]
pub type HPB3__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HPB4_` reader - HPB4"]
pub type HPB4__R = crate::BitReader<bool>;
#[doc = "Field `HPB4_` writer - HPB4"]
pub type HPB4__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `LPRAMHS_` reader - LPRAMHS"]
pub type LPRAMHS__R = crate::BitReader<bool>;
#[doc = "Field `LPRAMHS_` writer - LPRAMHS"]
pub type LPRAMHS__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `LPRAMPICOP_` reader - LPRAMPICOP"]
pub type LPRAMPICOP__R = crate::BitReader<bool>;
#[doc = "Field `LPRAMPICOP_` writer - LPRAMPICOP"]
pub type LPRAMPICOP__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `LPRAMDMAC_` reader - LPRAMDMAC"]
pub type LPRAMDMAC__R = crate::BitReader<bool>;
#[doc = "Field `LPRAMDMAC_` writer - LPRAMDMAC"]
pub type LPRAMDMAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `L2HBRIDGES_` reader - L2HBRIDGES"]
pub type L2HBRIDGES__R = crate::BitReader<bool>;
#[doc = "Field `L2HBRIDGES_` writer - L2HBRIDGES"]
pub type L2HBRIDGES__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
#[doc = "Field `HSRAMLP_` reader - HSRAMLP"]
pub type HSRAMLP__R = crate::BitReader<bool>;
#[doc = "Field `HSRAMLP_` writer - HSRAMLP"]
pub type HSRAMLP__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGAHB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> FLASH__R {
        FLASH__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    pub fn hsramcm0p_(&self) -> HSRAMCM0P__R {
        HSRAMCM0P__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&self) -> HSRAMDSU__R {
        HSRAMDSU__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - H2LBRIDGES"]
    #[inline(always)]
    pub fn h2lbridges_(&self) -> H2LBRIDGES__R {
        H2LBRIDGES__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HPB3"]
    #[inline(always)]
    pub fn hpb3_(&self) -> HPB3__R {
        HPB3__R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HPB4"]
    #[inline(always)]
    pub fn hpb4_(&self) -> HPB4__R {
        HPB4__R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - LPRAMHS"]
    #[inline(always)]
    pub fn lpramhs_(&self) -> LPRAMHS__R {
        LPRAMHS__R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LPRAMPICOP"]
    #[inline(always)]
    pub fn lprampicop_(&self) -> LPRAMPICOP__R {
        LPRAMPICOP__R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LPRAMDMAC"]
    #[inline(always)]
    pub fn lpramdmac_(&self) -> LPRAMDMAC__R {
        LPRAMDMAC__R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - L2HBRIDGES"]
    #[inline(always)]
    pub fn l2hbridges_(&self) -> L2HBRIDGES__R {
        L2HBRIDGES__R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HSRAMLP"]
    #[inline(always)]
    pub fn hsramlp_(&self) -> HSRAMLP__R {
        HSRAMLP__R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    #[must_use]
    pub fn flash_(&mut self) -> FLASH__W<0> {
        FLASH__W::new(self)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    #[must_use]
    pub fn hsramcm0p_(&mut self) -> HSRAMCM0P__W<1> {
        HSRAMCM0P__W::new(self)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    #[must_use]
    pub fn hsramdsu_(&mut self) -> HSRAMDSU__W<2> {
        HSRAMDSU__W::new(self)
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> HPB1__W<3> {
        HPB1__W::new(self)
    }
    #[doc = "Bit 4 - H2LBRIDGES"]
    #[inline(always)]
    #[must_use]
    pub fn h2lbridges_(&mut self) -> H2LBRIDGES__W<4> {
        H2LBRIDGES__W::new(self)
    }
    #[doc = "Bit 16 - HPB0"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> HPB0__W<16> {
        HPB0__W::new(self)
    }
    #[doc = "Bit 17 - HPB2"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> HPB2__W<17> {
        HPB2__W::new(self)
    }
    #[doc = "Bit 18 - HPB3"]
    #[inline(always)]
    #[must_use]
    pub fn hpb3_(&mut self) -> HPB3__W<18> {
        HPB3__W::new(self)
    }
    #[doc = "Bit 19 - HPB4"]
    #[inline(always)]
    #[must_use]
    pub fn hpb4_(&mut self) -> HPB4__W<19> {
        HPB4__W::new(self)
    }
    #[doc = "Bit 21 - LPRAMHS"]
    #[inline(always)]
    #[must_use]
    pub fn lpramhs_(&mut self) -> LPRAMHS__W<21> {
        LPRAMHS__W::new(self)
    }
    #[doc = "Bit 22 - LPRAMPICOP"]
    #[inline(always)]
    #[must_use]
    pub fn lprampicop_(&mut self) -> LPRAMPICOP__W<22> {
        LPRAMPICOP__W::new(self)
    }
    #[doc = "Bit 23 - LPRAMDMAC"]
    #[inline(always)]
    #[must_use]
    pub fn lpramdmac_(&mut self) -> LPRAMDMAC__W<23> {
        LPRAMDMAC__W::new(self)
    }
    #[doc = "Bit 24 - L2HBRIDGES"]
    #[inline(always)]
    #[must_use]
    pub fn l2hbridges_(&mut self) -> L2HBRIDGES__W<24> {
        L2HBRIDGES__W::new(self)
    }
    #[doc = "Bit 25 - HSRAMLP"]
    #[inline(always)]
    #[must_use]
    pub fn hsramlp_(&mut self) -> HSRAMLP__W<25> {
        HSRAMLP__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bridge interrupt flag status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflagahb](index.html) module"]
pub struct INTFLAGAHB_SPEC;
impl crate::RegisterSpec for INTFLAGAHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflagahb::R](R) reader structure"]
impl crate::Readable for INTFLAGAHB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflagahb::W](W) writer structure"]
impl crate::Writable for INTFLAGAHB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for INTFLAGAHB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `AHBMASK` reader"]
pub struct R(crate::R<AHBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBMASK` writer"]
pub struct W(crate::W<AHBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBMASK_SPEC>;
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
impl From<crate::W<AHBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPB0_` reader - HPB0 AHB Clock Mask"]
pub type HPB0__R = crate::BitReader<bool>;
#[doc = "Field `HPB0_` writer - HPB0 AHB Clock Mask"]
pub type HPB0__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `HPB1_` reader - HPB1 AHB Clock Mask"]
pub type HPB1__R = crate::BitReader<bool>;
#[doc = "Field `HPB1_` writer - HPB1 AHB Clock Mask"]
pub type HPB1__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `HPB2_` reader - HPB2 AHB Clock Mask"]
pub type HPB2__R = crate::BitReader<bool>;
#[doc = "Field `HPB2_` writer - HPB2 AHB Clock Mask"]
pub type HPB2__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `HPB3_` reader - HPB3 AHB Clock Mask"]
pub type HPB3__R = crate::BitReader<bool>;
#[doc = "Field `HPB3_` writer - HPB3 AHB Clock Mask"]
pub type HPB3__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `HPB4_` reader - HPB4 AHB Clock Mask"]
pub type HPB4__R = crate::BitReader<bool>;
#[doc = "Field `HPB4_` writer - HPB4 AHB Clock Mask"]
pub type HPB4__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `DSU_` reader - DSU AHB Clock Mask"]
pub type DSU__R = crate::BitReader<bool>;
#[doc = "Field `DSU_` writer - DSU AHB Clock Mask"]
pub type DSU__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__R = crate::BitReader<bool>;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL AHB Clock Mask"]
pub type NVMCTRL__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `HSRAM_` reader - HSRAM AHB Clock Mask"]
pub type HSRAM__R = crate::BitReader<bool>;
#[doc = "Field `HSRAM_` writer - HSRAM AHB Clock Mask"]
pub type HSRAM__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `LPRAM_` reader - LPRAM AHB Clock Mask"]
pub type LPRAM__R = crate::BitReader<bool>;
#[doc = "Field `LPRAM_` writer - LPRAM AHB Clock Mask"]
pub type LPRAM__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `DMAC_` reader - DMAC AHB Clock Mask"]
pub type DMAC__R = crate::BitReader<bool>;
#[doc = "Field `DMAC_` writer - DMAC AHB Clock Mask"]
pub type DMAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `USB_` reader - USB AHB Clock Mask"]
pub type USB__R = crate::BitReader<bool>;
#[doc = "Field `USB_` writer - USB AHB Clock Mask"]
pub type USB__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `PAC_` reader - PAC AHB Clock Mask"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `PAC_` writer - PAC AHB Clock Mask"]
pub type PAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `NVMCTRL_PICACHU_` reader - NVMCTRL_PICACHU AHB Clock Mask"]
pub type NVMCTRL_PICACHU__R = crate::BitReader<bool>;
#[doc = "Field `NVMCTRL_PICACHU_` writer - NVMCTRL_PICACHU AHB Clock Mask"]
pub type NVMCTRL_PICACHU__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `L2HBRIDGES_H_` reader - L2HBRIDGES_H AHB Clock Mask"]
pub type L2HBRIDGES_H__R = crate::BitReader<bool>;
#[doc = "Field `L2HBRIDGES_H_` writer - L2HBRIDGES_H AHB Clock Mask"]
pub type L2HBRIDGES_H__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `H2LBRIDGES_H_` reader - H2LBRIDGES_H AHB Clock Mask"]
pub type H2LBRIDGES_H__R = crate::BitReader<bool>;
#[doc = "Field `H2LBRIDGES_H_` writer - H2LBRIDGES_H AHB Clock Mask"]
pub type H2LBRIDGES_H__W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `HSRAM_AHBSETUPKEEPER_` reader - HSRAM_AHBSETUPKEEPER AHB Clock Mask"]
pub type HSRAM_AHBSETUPKEEPER__R = crate::BitReader<bool>;
#[doc = "Field `HSRAM_AHBSETUPKEEPER_` writer - HSRAM_AHBSETUPKEEPER AHB Clock Mask"]
pub type HSRAM_AHBSETUPKEEPER__W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
#[doc = "Field `HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE_` reader - HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE AHB Clock Mask"]
pub type HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE__R = crate::BitReader<bool>;
#[doc = "Field `HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE_` writer - HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE AHB Clock Mask"]
pub type HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE__W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb0_(&self) -> HPB0__R {
        HPB0__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb1_(&self) -> HPB1__R {
        HPB1__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb2_(&self) -> HPB2__R {
        HPB2__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb3_(&self) -> HPB3__R {
        HPB3__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HPB4 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb4_(&self) -> HPB4__R {
        HPB4__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&self) -> DSU__R {
        DSU__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> NVMCTRL__R {
        NVMCTRL__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&self) -> HSRAM__R {
        HSRAM__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn lpram_(&self) -> LPRAM__R {
        LPRAM__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB AHB Clock Mask"]
    #[inline(always)]
    pub fn usb_(&self) -> USB__R {
        USB__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NVMCTRL_PICACHU AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_picachu_(&self) -> NVMCTRL_PICACHU__R {
        NVMCTRL_PICACHU__R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L2HBRIDGES_H AHB Clock Mask"]
    #[inline(always)]
    pub fn l2hbridges_h_(&self) -> L2HBRIDGES_H__R {
        L2HBRIDGES_H__R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - H2LBRIDGES_H AHB Clock Mask"]
    #[inline(always)]
    pub fn h2lbridges_h_(&self) -> H2LBRIDGES_H__R {
        H2LBRIDGES_H__R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSRAM_AHBSETUPKEEPER AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_ahbsetupkeeper_(&self) -> HSRAM_AHBSETUPKEEPER__R {
        HSRAM_AHBSETUPKEEPER__R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_hmatrixlp2hmcramchsbridge_(&self) -> HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE__R {
        HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE__R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> HPB0__W<0> {
        HPB0__W::new(self)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> HPB1__W<1> {
        HPB1__W::new(self)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> HPB2__W<2> {
        HPB2__W::new(self)
    }
    #[doc = "Bit 3 - HPB3 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb3_(&mut self) -> HPB3__W<3> {
        HPB3__W::new(self)
    }
    #[doc = "Bit 4 - HPB4 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb4_(&mut self) -> HPB4__W<4> {
        HPB4__W::new(self)
    }
    #[doc = "Bit 5 - DSU AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> DSU__W<5> {
        DSU__W::new(self)
    }
    #[doc = "Bit 8 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> NVMCTRL__W<8> {
        NVMCTRL__W::new(self)
    }
    #[doc = "Bit 9 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hsram_(&mut self) -> HSRAM__W<9> {
        HSRAM__W::new(self)
    }
    #[doc = "Bit 10 - LPRAM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpram_(&mut self) -> LPRAM__W<10> {
        LPRAM__W::new(self)
    }
    #[doc = "Bit 11 - DMAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> DMAC__W<11> {
        DMAC__W::new(self)
    }
    #[doc = "Bit 12 - USB AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usb_(&mut self) -> USB__W<12> {
        USB__W::new(self)
    }
    #[doc = "Bit 14 - PAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pac_(&mut self) -> PAC__W<14> {
        PAC__W::new(self)
    }
    #[doc = "Bit 15 - NVMCTRL_PICACHU AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_picachu_(&mut self) -> NVMCTRL_PICACHU__W<15> {
        NVMCTRL_PICACHU__W::new(self)
    }
    #[doc = "Bit 16 - L2HBRIDGES_H AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn l2hbridges_h_(&mut self) -> L2HBRIDGES_H__W<16> {
        L2HBRIDGES_H__W::new(self)
    }
    #[doc = "Bit 17 - H2LBRIDGES_H AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn h2lbridges_h_(&mut self) -> H2LBRIDGES_H__W<17> {
        H2LBRIDGES_H__W::new(self)
    }
    #[doc = "Bit 18 - HSRAM_AHBSETUPKEEPER AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hsram_ahbsetupkeeper_(&mut self) -> HSRAM_AHBSETUPKEEPER__W<18> {
        HSRAM_AHBSETUPKEEPER__W::new(self)
    }
    #[doc = "Bit 19 - HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hsram_hmatrixlp2hmcramchsbridge_(&mut self) -> HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE__W<19> {
        HSRAM_HMATRIXLP2HMCRAMCHSBRIDGE__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbmask](index.html) module"]
pub struct AHBMASK_SPEC;
impl crate::RegisterSpec for AHBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbmask::R](R) reader structure"]
impl crate::Readable for AHBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbmask::W](W) writer structure"]
impl crate::Writable for AHBMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBMASK to value 0x000f_ffff"]
impl crate::Resettable for AHBMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_ffff;
}

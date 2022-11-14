#[doc = "Register `INTFLAGE` reader"]
pub struct R(crate::R<INTFLAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGE` writer"]
pub struct W(crate::W<INTFLAGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGE_SPEC>;
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
impl From<crate::W<INTFLAGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC_` reader - PAC"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `PAC_` writer - PAC"]
pub type PAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGE_SPEC, bool, O>;
#[doc = "Field `DMAC_` reader - DMAC"]
pub type DMAC__R = crate::BitReader<bool>;
#[doc = "Field `DMAC_` writer - DMAC"]
pub type DMAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, INTFLAGE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PAC"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC"]
    #[inline(always)]
    #[must_use]
    pub fn pac_(&mut self) -> PAC__W<0> {
        PAC__W::new(self)
    }
    #[doc = "Bit 1 - DMAC"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> DMAC__W<1> {
        DMAC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflage](index.html) module"]
pub struct INTFLAGE_SPEC;
impl crate::RegisterSpec for INTFLAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflage::R](R) reader structure"]
impl crate::Readable for INTFLAGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflage::W](W) writer structure"]
impl crate::Writable for INTFLAGE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTFLAGE to value 0"]
impl crate::Resettable for INTFLAGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `APBEMASK` reader"]
pub struct R(crate::R<APBEMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBEMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBEMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBEMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBEMASK` writer"]
pub struct W(crate::W<APBEMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBEMASK_SPEC>;
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
impl From<crate::W<APBEMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBEMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC_` reader - PAC APB Clock Enable"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `PAC_` writer - PAC APB Clock Enable"]
pub type PAC__W<'a, const O: u8> = crate::BitWriter<'a, u32, APBEMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PAC APB Clock Enable"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pac_(&mut self) -> PAC__W<0> {
        PAC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBE Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbemask](index.html) module"]
pub struct APBEMASK_SPEC;
impl crate::RegisterSpec for APBEMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbemask::R](R) reader structure"]
impl crate::Readable for APBEMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbemask::W](W) writer structure"]
impl crate::Writable for APBEMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APBEMASK to value 0x0d"]
impl crate::Resettable for APBEMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}

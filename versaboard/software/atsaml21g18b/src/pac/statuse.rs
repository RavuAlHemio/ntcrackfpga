#[doc = "Register `STATUSE` reader"]
pub struct R(crate::R<STATUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PAC_` reader - PAC APB Protect Enable"]
pub type PAC__R = crate::BitReader<bool>;
#[doc = "Field `DMAC_` reader - DMAC APB Protect Enable"]
pub type DMAC__R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - PAC APB Protect Enable"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC APB Protect Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> DMAC__R {
        DMAC__R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge E\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statuse](index.html) module"]
pub struct STATUSE_SPEC;
impl crate::RegisterSpec for STATUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statuse::R](R) reader structure"]
impl crate::Readable for STATUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUSE to value 0"]
impl crate::Resettable for STATUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

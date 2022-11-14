#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READY0` reader - OPAMP 0 Ready"]
pub type READY0_R = crate::BitReader<bool>;
#[doc = "Field `READY1` reader - OPAMP 1 Ready"]
pub type READY1_R = crate::BitReader<bool>;
#[doc = "Field `READY2` reader - OPAMP 2 Ready"]
pub type READY2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - OPAMP 0 Ready"]
    #[inline(always)]
    pub fn ready0(&self) -> READY0_R {
        READY0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPAMP 1 Ready"]
    #[inline(always)]
    pub fn ready1(&self) -> READY1_R {
        READY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPAMP 2 Ready"]
    #[inline(always)]
    pub fn ready2(&self) -> READY2_R {
        READY2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

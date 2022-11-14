#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWRST` reader - Software Reset Synchroniation Busy bit"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL0` reader - Generic Clock Generator Control 0 Synchronization Busy bits"]
pub type GENCTRL0_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL1` reader - Generic Clock Generator Control 1 Synchronization Busy bits"]
pub type GENCTRL1_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL2` reader - Generic Clock Generator Control 2 Synchronization Busy bits"]
pub type GENCTRL2_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL3` reader - Generic Clock Generator Control 3 Synchronization Busy bits"]
pub type GENCTRL3_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL4` reader - Generic Clock Generator Control 4 Synchronization Busy bits"]
pub type GENCTRL4_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL5` reader - Generic Clock Generator Control 5 Synchronization Busy bits"]
pub type GENCTRL5_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL6` reader - Generic Clock Generator Control 6 Synchronization Busy bits"]
pub type GENCTRL6_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL7` reader - Generic Clock Generator Control 7 Synchronization Busy bits"]
pub type GENCTRL7_R = crate::BitReader<bool>;
#[doc = "Field `GENCTRL8` reader - Generic Clock Generator Control 8 Synchronization Busy bits"]
pub type GENCTRL8_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Generic Clock Generator Control 0 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl0(&self) -> GENCTRL0_R {
        GENCTRL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic Clock Generator Control 1 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl1(&self) -> GENCTRL1_R {
        GENCTRL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic Clock Generator Control 2 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl2(&self) -> GENCTRL2_R {
        GENCTRL2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic Clock Generator Control 3 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl3(&self) -> GENCTRL3_R {
        GENCTRL3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic Clock Generator Control 4 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl4(&self) -> GENCTRL4_R {
        GENCTRL4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generic Clock Generator Control 5 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl5(&self) -> GENCTRL5_R {
        GENCTRL5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic Clock Generator Control 6 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl6(&self) -> GENCTRL6_R {
        GENCTRL6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic Clock Generator Control 7 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl7(&self) -> GENCTRL7_R {
        GENCTRL7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic Clock Generator Control 8 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl8(&self) -> GENCTRL8_R {
        GENCTRL8_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

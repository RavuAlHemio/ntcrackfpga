#[doc = "Register `WKCAUSE` reader"]
pub struct R(crate::R<WKCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKCAUSE` writer"]
pub struct W(crate::W<WKCAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKCAUSE_SPEC>;
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
impl From<crate::W<WKCAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKCAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKCAUSE` reader - Wakeup Cause"]
pub type WKCAUSE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Wakeup Cause"]
    #[inline(always)]
    pub fn wkcause(&self) -> WKCAUSE_R {
        WKCAUSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Cause\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkcause](index.html) module"]
pub struct WKCAUSE_SPEC;
impl crate::RegisterSpec for WKCAUSE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wkcause::R](R) reader structure"]
impl crate::Readable for WKCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkcause::W](W) writer structure"]
impl crate::Writable for WKCAUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKCAUSE to value 0"]
impl crate::Resettable for WKCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

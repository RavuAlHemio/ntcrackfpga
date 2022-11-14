#[doc = "Register `WKPOL` reader"]
pub struct R(crate::R<WKPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKPOL` writer"]
pub struct W(crate::W<WKPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKPOL_SPEC>;
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
impl From<crate::W<WKPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKPOL` reader - Wakeup Polarity"]
pub type WKPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKPOL` writer - Wakeup Polarity"]
pub type WKPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WKPOL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Wakeup Polarity"]
    #[inline(always)]
    pub fn wkpol(&self) -> WKPOL_R {
        WKPOL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wakeup Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wkpol(&mut self) -> WKPOL_W<0> {
        WKPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkpol](index.html) module"]
pub struct WKPOL_SPEC;
impl crate::RegisterSpec for WKPOL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wkpol::R](R) reader structure"]
impl crate::Readable for WKPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkpol::W](W) writer structure"]
impl crate::Writable for WKPOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKPOL to value 0"]
impl crate::Resettable for WKPOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

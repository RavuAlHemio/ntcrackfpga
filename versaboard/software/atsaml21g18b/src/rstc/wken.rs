#[doc = "Register `WKEN` reader"]
pub struct R(crate::R<WKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKEN` writer"]
pub struct W(crate::W<WKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKEN_SPEC>;
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
impl From<crate::W<WKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKEN` reader - Wakeup Enable"]
pub type WKEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKEN` writer - Wakeup Enable"]
pub type WKEN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WKEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Wakeup Enable"]
    #[inline(always)]
    pub fn wken(&self) -> WKEN_R {
        WKEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wken(&mut self) -> WKEN_W<0> {
        WKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wken](index.html) module"]
pub struct WKEN_SPEC;
impl crate::RegisterSpec for WKEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wken::R](R) reader structure"]
impl crate::Readable for WKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wken::W](W) writer structure"]
impl crate::Writable for WKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKEN to value 0"]
impl crate::Resettable for WKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

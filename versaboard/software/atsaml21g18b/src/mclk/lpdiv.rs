#[doc = "Register `LPDIV` reader"]
pub struct R(crate::R<LPDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPDIV` writer"]
pub struct W(crate::W<LPDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDIV_SPEC>;
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
impl From<crate::W<LPDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPDIV` reader - Low-Power Clock Division Factor"]
pub type LPDIV_R = crate::FieldReader<u8, LPDIVSELECT_A>;
#[doc = "Low-Power Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPDIVSELECT_A {
    #[doc = "1: Divide by 1"]
    DIV1 = 1,
    #[doc = "2: Divide by 2"]
    DIV2 = 2,
    #[doc = "4: Divide by 4"]
    DIV4 = 4,
    #[doc = "8: Divide by 8"]
    DIV8 = 8,
    #[doc = "16: Divide by 16"]
    DIV16 = 16,
    #[doc = "32: Divide by 32"]
    DIV32 = 32,
    #[doc = "64: Divide by 64"]
    DIV64 = 64,
    #[doc = "128: Divide by 128"]
    DIV128 = 128,
}
impl From<LPDIVSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LPDIVSELECT_A) -> Self {
        variant as _
    }
}
impl LPDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPDIVSELECT_A> {
        match self.bits {
            1 => Some(LPDIVSELECT_A::DIV1),
            2 => Some(LPDIVSELECT_A::DIV2),
            4 => Some(LPDIVSELECT_A::DIV4),
            8 => Some(LPDIVSELECT_A::DIV8),
            16 => Some(LPDIVSELECT_A::DIV16),
            32 => Some(LPDIVSELECT_A::DIV32),
            64 => Some(LPDIVSELECT_A::DIV64),
            128 => Some(LPDIVSELECT_A::DIV128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LPDIVSELECT_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LPDIVSELECT_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LPDIVSELECT_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LPDIVSELECT_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LPDIVSELECT_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LPDIVSELECT_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LPDIVSELECT_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LPDIVSELECT_A::DIV128
    }
}
#[doc = "Field `LPDIV` writer - Low-Power Clock Division Factor"]
pub type LPDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LPDIV_SPEC, u8, LPDIVSELECT_A, 8, O>;
impl<'a, const O: u8> LPDIV_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LPDIVSELECT_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LPDIVSELECT_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LPDIVSELECT_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LPDIVSELECT_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(LPDIVSELECT_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(LPDIVSELECT_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(LPDIVSELECT_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LPDIVSELECT_A::DIV128)
    }
}
impl R {
    #[doc = "Bits 0:7 - Low-Power Clock Division Factor"]
    #[inline(always)]
    pub fn lpdiv(&self) -> LPDIV_R {
        LPDIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low-Power Clock Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn lpdiv(&mut self) -> LPDIV_W<0> {
        LPDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low-Power Clock Division\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpdiv](index.html) module"]
pub struct LPDIV_SPEC;
impl crate::RegisterSpec for LPDIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lpdiv::R](R) reader structure"]
impl crate::Readable for LPDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpdiv::W](W) writer structure"]
impl crate::Writable for LPDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPDIV to value 0x01"]
impl crate::Resettable for LPDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

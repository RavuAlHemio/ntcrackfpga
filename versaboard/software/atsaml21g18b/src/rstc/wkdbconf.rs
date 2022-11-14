#[doc = "Register `WKDBCONF` reader"]
pub struct R(crate::R<WKDBCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKDBCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKDBCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKDBCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKDBCONF` writer"]
pub struct W(crate::W<WKDBCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKDBCONF_SPEC>;
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
impl From<crate::W<WKDBCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKDBCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKDBCNT` reader - Wakeup Debounce Counter"]
pub type WKDBCNT_R = crate::FieldReader<u8, WKDBCNTSELECT_A>;
#[doc = "Wakeup Debounce Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WKDBCNTSELECT_A {
    #[doc = "0: No debouncing.Input pin is low or high level sensitive depending on its WKPOLx bit."]
    OFF = 0,
    #[doc = "1: Input pin shall be active for at least two 32kHz clock period."]
    _2CK32 = 1,
    #[doc = "2: Input pin shall be active for at least three 32kHz clock period."]
    _3CK32 = 2,
    #[doc = "3: Input pin shall be active for at least 32 32kHz clock period."]
    _32CK32 = 3,
    #[doc = "4: Input pin shall be active for at least 512 32kHz clock period."]
    _512CK32 = 4,
    #[doc = "5: Input pin shall be active for at least 4096 32kHz clock period."]
    _4096CK32 = 5,
    #[doc = "6: Input pin shall be active for at least 32768 32kHz clock period."]
    _32768CK32 = 6,
}
impl From<WKDBCNTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WKDBCNTSELECT_A) -> Self {
        variant as _
    }
}
impl WKDBCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WKDBCNTSELECT_A> {
        match self.bits {
            0 => Some(WKDBCNTSELECT_A::OFF),
            1 => Some(WKDBCNTSELECT_A::_2CK32),
            2 => Some(WKDBCNTSELECT_A::_3CK32),
            3 => Some(WKDBCNTSELECT_A::_32CK32),
            4 => Some(WKDBCNTSELECT_A::_512CK32),
            5 => Some(WKDBCNTSELECT_A::_4096CK32),
            6 => Some(WKDBCNTSELECT_A::_32768CK32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == WKDBCNTSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `_2CK32`"]
    #[inline(always)]
    pub fn is_2ck32(&self) -> bool {
        *self == WKDBCNTSELECT_A::_2CK32
    }
    #[doc = "Checks if the value of the field is `_3CK32`"]
    #[inline(always)]
    pub fn is_3ck32(&self) -> bool {
        *self == WKDBCNTSELECT_A::_3CK32
    }
    #[doc = "Checks if the value of the field is `_32CK32`"]
    #[inline(always)]
    pub fn is_32ck32(&self) -> bool {
        *self == WKDBCNTSELECT_A::_32CK32
    }
    #[doc = "Checks if the value of the field is `_512CK32`"]
    #[inline(always)]
    pub fn is_512ck32(&self) -> bool {
        *self == WKDBCNTSELECT_A::_512CK32
    }
    #[doc = "Checks if the value of the field is `_4096CK32`"]
    #[inline(always)]
    pub fn is_4096ck32(&self) -> bool {
        *self == WKDBCNTSELECT_A::_4096CK32
    }
    #[doc = "Checks if the value of the field is `_32768CK32`"]
    #[inline(always)]
    pub fn is_32768ck32(&self) -> bool {
        *self == WKDBCNTSELECT_A::_32768CK32
    }
}
#[doc = "Field `WKDBCNT` writer - Wakeup Debounce Counter"]
pub type WKDBCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, WKDBCONF_SPEC, u8, WKDBCNTSELECT_A, 5, O>;
impl<'a, const O: u8> WKDBCNT_W<'a, O> {
    #[doc = "No debouncing.Input pin is low or high level sensitive depending on its WKPOLx bit."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(WKDBCNTSELECT_A::OFF)
    }
    #[doc = "Input pin shall be active for at least two 32kHz clock period."]
    #[inline(always)]
    pub fn _2ck32(self) -> &'a mut W {
        self.variant(WKDBCNTSELECT_A::_2CK32)
    }
    #[doc = "Input pin shall be active for at least three 32kHz clock period."]
    #[inline(always)]
    pub fn _3ck32(self) -> &'a mut W {
        self.variant(WKDBCNTSELECT_A::_3CK32)
    }
    #[doc = "Input pin shall be active for at least 32 32kHz clock period."]
    #[inline(always)]
    pub fn _32ck32(self) -> &'a mut W {
        self.variant(WKDBCNTSELECT_A::_32CK32)
    }
    #[doc = "Input pin shall be active for at least 512 32kHz clock period."]
    #[inline(always)]
    pub fn _512ck32(self) -> &'a mut W {
        self.variant(WKDBCNTSELECT_A::_512CK32)
    }
    #[doc = "Input pin shall be active for at least 4096 32kHz clock period."]
    #[inline(always)]
    pub fn _4096ck32(self) -> &'a mut W {
        self.variant(WKDBCNTSELECT_A::_4096CK32)
    }
    #[doc = "Input pin shall be active for at least 32768 32kHz clock period."]
    #[inline(always)]
    pub fn _32768ck32(self) -> &'a mut W {
        self.variant(WKDBCNTSELECT_A::_32768CK32)
    }
}
impl R {
    #[doc = "Bits 0:4 - Wakeup Debounce Counter"]
    #[inline(always)]
    pub fn wkdbcnt(&self) -> WKDBCNT_R {
        WKDBCNT_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Wakeup Debounce Counter"]
    #[inline(always)]
    #[must_use]
    pub fn wkdbcnt(&mut self) -> WKDBCNT_W<0> {
        WKDBCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Debounce Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkdbconf](index.html) module"]
pub struct WKDBCONF_SPEC;
impl crate::RegisterSpec for WKDBCONF_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wkdbconf::R](R) reader structure"]
impl crate::Readable for WKDBCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkdbconf::W](W) writer structure"]
impl crate::Writable for WKDBCONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKDBCONF to value 0"]
impl crate::Resettable for WKDBCONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

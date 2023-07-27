#[doc = "Register `has_stscntrl` reader"]
pub struct R(crate::R<HAS_STSCNTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HAS_STSCNTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HAS_STSCNTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HAS_STSCNTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `has_stscntrl` writer"]
pub struct W(crate::W<HAS_STSCNTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HAS_STSCNTRL_SPEC>;
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
impl From<crate::W<HAS_STSCNTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HAS_STSCNTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `has` reader - "]
pub type HAS_R = crate::BitReader<HAS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAS_A {
    #[doc = "0: `0`"]
    FALSE = 0,
    #[doc = "1: `1`"]
    TRUE = 1,
}
impl From<HAS_A> for bool {
    #[inline(always)]
    fn from(variant: HAS_A) -> Self {
        variant as u8 != 0
    }
}
impl HAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAS_A {
        match self.bits {
            false => HAS_A::FALSE,
            true => HAS_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == HAS_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == HAS_A::TRUE
    }
}
#[doc = "Field `has` writer - "]
pub type HAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HAS_STSCNTRL_SPEC, HAS_A, O>;
impl<'a, const O: u8> HAS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(HAS_A::FALSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(HAS_A::TRUE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn has(&self) -> HAS_R {
        HAS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn has(&mut self) -> HAS_W<0> {
        HAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Whether has stscntrl strm\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [has_stscntrl](index.html) module"]
pub struct HAS_STSCNTRL_SPEC;
impl crate::RegisterSpec for HAS_STSCNTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [has_stscntrl::R](R) reader structure"]
impl crate::Readable for HAS_STSCNTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [has_stscntrl::W](W) writer structure"]
impl crate::Writable for HAS_STSCNTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets has_stscntrl to value 0"]
impl crate::Resettable for HAS_STSCNTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

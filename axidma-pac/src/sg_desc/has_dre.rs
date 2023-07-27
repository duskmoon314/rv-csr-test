#[doc = "Register `has_dre` reader"]
pub struct R(crate::R<HAS_DRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HAS_DRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HAS_DRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HAS_DRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `has_dre` writer"]
pub struct W(crate::W<HAS_DRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HAS_DRE_SPEC>;
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
impl From<crate::W<HAS_DRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HAS_DRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wordlen` reader - "]
pub type WORDLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wordlen` writer - "]
pub type WORDLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HAS_DRE_SPEC, u16, u16, 16, O>;
#[doc = "Field `has_dre` reader - "]
pub type HAS_DRE_R = crate::BitReader<HAS_DRE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HAS_DRE_A {
    #[doc = "0: `0`"]
    FALSE = 0,
    #[doc = "1: `1`"]
    TRUE = 1,
}
impl From<HAS_DRE_A> for bool {
    #[inline(always)]
    fn from(variant: HAS_DRE_A) -> Self {
        variant as u8 != 0
    }
}
impl HAS_DRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAS_DRE_A {
        match self.bits {
            false => HAS_DRE_A::FALSE,
            true => HAS_DRE_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == HAS_DRE_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == HAS_DRE_A::TRUE
    }
}
#[doc = "Field `has_dre` writer - "]
pub type HAS_DRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HAS_DRE_SPEC, HAS_DRE_A, O>;
impl<'a, const O: u8> HAS_DRE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(HAS_DRE_A::FALSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(HAS_DRE_A::TRUE)
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wordlen(&self) -> WORDLEN_R {
        WORDLEN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn has_dre(&self) -> HAS_DRE_R {
        HAS_DRE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn wordlen(&mut self) -> WORDLEN_W<0> {
        WORDLEN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn has_dre(&mut self) -> HAS_DRE_W<16> {
        HAS_DRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Whether has DRE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [has_dre](index.html) module"]
pub struct HAS_DRE_SPEC;
impl crate::RegisterSpec for HAS_DRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [has_dre::R](R) reader structure"]
impl crate::Readable for HAS_DRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [has_dre::W](W) writer structure"]
impl crate::Writable for HAS_DRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets has_dre to value 0"]
impl crate::Resettable for HAS_DRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

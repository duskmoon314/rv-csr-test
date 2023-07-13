#[doc = "Register `ltcr` reader"]
pub struct R(crate::R<LTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ltcr` writer"]
pub struct W(crate::W<LTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTCR_SPEC>;
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
impl From<crate::W<LTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_lt_training_enable` reader - "]
pub type CTL_LT_TRAINING_ENABLE_R = crate::BitReader<CTL_LT_TRAINING_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_LT_TRAINING_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_LT_TRAINING_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_LT_TRAINING_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_LT_TRAINING_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_LT_TRAINING_ENABLE_A {
        match self.bits {
            false => CTL_LT_TRAINING_ENABLE_A::DISABLE,
            true => CTL_LT_TRAINING_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_LT_TRAINING_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_LT_TRAINING_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_lt_training_enable` writer - "]
pub type CTL_LT_TRAINING_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, LTCR_SPEC, O, CTL_LT_TRAINING_ENABLE_A>;
impl<'a, const O: u8> CTL_LT_TRAINING_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_LT_TRAINING_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_LT_TRAINING_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_lt_restart_training` reader - "]
pub type CTL_LT_RESTART_TRAINING_R = crate::BitReader<CTL_LT_RESTART_TRAINING_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_LT_RESTART_TRAINING_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_LT_RESTART_TRAINING_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_LT_RESTART_TRAINING_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_LT_RESTART_TRAINING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_LT_RESTART_TRAINING_A {
        match self.bits {
            false => CTL_LT_RESTART_TRAINING_A::DISABLE,
            true => CTL_LT_RESTART_TRAINING_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_LT_RESTART_TRAINING_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_LT_RESTART_TRAINING_A::ENABLE
    }
}
#[doc = "Field `ctl_lt_restart_training` writer - "]
pub type CTL_LT_RESTART_TRAINING_W<'a, const O: u8> =
    crate::BitWriter<'a, LTCR_SPEC, O, CTL_LT_RESTART_TRAINING_A>;
impl<'a, const O: u8> CTL_LT_RESTART_TRAINING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_LT_RESTART_TRAINING_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_LT_RESTART_TRAINING_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_lt_training_enable(&self) -> CTL_LT_TRAINING_ENABLE_R {
        CTL_LT_TRAINING_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctl_lt_restart_training(&self) -> CTL_LT_RESTART_TRAINING_R {
        CTL_LT_RESTART_TRAINING_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_training_enable(&mut self) -> CTL_LT_TRAINING_ENABLE_W<0> {
        CTL_LT_TRAINING_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_restart_training(&mut self) -> CTL_LT_RESTART_TRAINING_W<1> {
        CTL_LT_RESTART_TRAINING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LT_CONTROL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltcr](index.html) module"]
pub struct LTCR_SPEC;
impl crate::RegisterSpec for LTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltcr::R](R) reader structure"]
impl crate::Readable for LTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltcr::W](W) writer structure"]
impl crate::Writable for LTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ltcr to value 0"]
impl crate::Resettable for LTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

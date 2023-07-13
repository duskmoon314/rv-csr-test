#[doc = "Register `lttr` reader"]
pub struct R(crate::R<LTTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lttr` writer"]
pub struct W(crate::W<LTTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTTR_SPEC>;
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
impl From<crate::W<LTTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_lt_rx_trained` reader - "]
pub type CTL_LT_RX_TRAINED_R = crate::BitReader<CTL_LT_RX_TRAINED_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_LT_RX_TRAINED_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_LT_RX_TRAINED_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_LT_RX_TRAINED_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_LT_RX_TRAINED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_LT_RX_TRAINED_A {
        match self.bits {
            false => CTL_LT_RX_TRAINED_A::DISABLE,
            true => CTL_LT_RX_TRAINED_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_LT_RX_TRAINED_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_LT_RX_TRAINED_A::ENABLE
    }
}
#[doc = "Field `ctl_lt_rx_trained` writer - "]
pub type CTL_LT_RX_TRAINED_W<'a, const O: u8> =
    crate::BitWriter<'a, LTTR_SPEC, O, CTL_LT_RX_TRAINED_A>;
impl<'a, const O: u8> CTL_LT_RX_TRAINED_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_LT_RX_TRAINED_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_LT_RX_TRAINED_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_lt_rx_trained(&self) -> CTL_LT_RX_TRAINED_R {
        CTL_LT_RX_TRAINED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_rx_trained(&mut self) -> CTL_LT_RX_TRAINED_W<0> {
        CTL_LT_RX_TRAINED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LT_TRAINED Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lttr](index.html) module"]
pub struct LTTR_SPEC;
impl crate::RegisterSpec for LTTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lttr::R](R) reader structure"]
impl crate::Readable for LTTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lttr::W](W) writer structure"]
impl crate::Writable for LTTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lttr to value 0"]
impl crate::Resettable for LTTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

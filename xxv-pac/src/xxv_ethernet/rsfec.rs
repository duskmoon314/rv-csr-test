#[doc = "Register `rsfec` reader"]
pub struct R(crate::R<RSFEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSFEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSFEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSFEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rsfec` writer"]
pub struct W(crate::W<RSFEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSFEC_SPEC>;
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
impl From<crate::W<RSFEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSFEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_rsfec_enable` reader - "]
pub type CTL_RSFEC_ENABLE_R = crate::BitReader<CTL_RSFEC_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RSFEC_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RSFEC_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RSFEC_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RSFEC_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RSFEC_ENABLE_A {
        match self.bits {
            false => CTL_RSFEC_ENABLE_A::DISABLE,
            true => CTL_RSFEC_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RSFEC_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RSFEC_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_rsfec_enable` writer - "]
pub type CTL_RSFEC_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, RSFEC_SPEC, O, CTL_RSFEC_ENABLE_A>;
impl<'a, const O: u8> CTL_RSFEC_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RSFEC_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RSFEC_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_rsfec_consortium_25g` reader - "]
pub type CTL_RSFEC_CONSORTIUM_25G_R = crate::BitReader<CTL_RSFEC_CONSORTIUM_25G_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RSFEC_CONSORTIUM_25G_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RSFEC_CONSORTIUM_25G_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RSFEC_CONSORTIUM_25G_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RSFEC_CONSORTIUM_25G_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RSFEC_CONSORTIUM_25G_A {
        match self.bits {
            false => CTL_RSFEC_CONSORTIUM_25G_A::DISABLE,
            true => CTL_RSFEC_CONSORTIUM_25G_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RSFEC_CONSORTIUM_25G_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RSFEC_CONSORTIUM_25G_A::ENABLE
    }
}
#[doc = "Field `ctl_rsfec_consortium_25g` writer - "]
pub type CTL_RSFEC_CONSORTIUM_25G_W<'a, const O: u8> =
    crate::BitWriter<'a, RSFEC_SPEC, O, CTL_RSFEC_CONSORTIUM_25G_A>;
impl<'a, const O: u8> CTL_RSFEC_CONSORTIUM_25G_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RSFEC_CONSORTIUM_25G_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RSFEC_CONSORTIUM_25G_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_rsfec_enable_indication` reader - "]
pub type CTL_RX_RSFEC_ENABLE_INDICATION_R = crate::BitReader<CTL_RX_RSFEC_ENABLE_INDICATION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_RSFEC_ENABLE_INDICATION_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_RSFEC_ENABLE_INDICATION_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_RSFEC_ENABLE_INDICATION_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_RSFEC_ENABLE_INDICATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_RSFEC_ENABLE_INDICATION_A {
        match self.bits {
            false => CTL_RX_RSFEC_ENABLE_INDICATION_A::DISABLE,
            true => CTL_RX_RSFEC_ENABLE_INDICATION_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_RSFEC_ENABLE_INDICATION_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_RSFEC_ENABLE_INDICATION_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_rsfec_enable_indication` writer - "]
pub type CTL_RX_RSFEC_ENABLE_INDICATION_W<'a, const O: u8> =
    crate::BitWriter<'a, RSFEC_SPEC, O, CTL_RX_RSFEC_ENABLE_INDICATION_A>;
impl<'a, const O: u8> CTL_RX_RSFEC_ENABLE_INDICATION_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_RSFEC_ENABLE_INDICATION_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_RSFEC_ENABLE_INDICATION_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_rsfec_enable_correction` reader - "]
pub type CTL_RX_RSFEC_ENABLE_CORRECTION_R = crate::BitReader<CTL_RX_RSFEC_ENABLE_CORRECTION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_RSFEC_ENABLE_CORRECTION_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_RSFEC_ENABLE_CORRECTION_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_RSFEC_ENABLE_CORRECTION_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_RSFEC_ENABLE_CORRECTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_RSFEC_ENABLE_CORRECTION_A {
        match self.bits {
            false => CTL_RX_RSFEC_ENABLE_CORRECTION_A::DISABLE,
            true => CTL_RX_RSFEC_ENABLE_CORRECTION_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_RSFEC_ENABLE_CORRECTION_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_RSFEC_ENABLE_CORRECTION_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_rsfec_enable_correction` writer - "]
pub type CTL_RX_RSFEC_ENABLE_CORRECTION_W<'a, const O: u8> =
    crate::BitWriter<'a, RSFEC_SPEC, O, CTL_RX_RSFEC_ENABLE_CORRECTION_A>;
impl<'a, const O: u8> CTL_RX_RSFEC_ENABLE_CORRECTION_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_RSFEC_ENABLE_CORRECTION_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_RSFEC_ENABLE_CORRECTION_A::ENABLE)
    }
}
#[doc = "Field `ctl_rsfec_ieee_error_indication_mode` reader - "]
pub type CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_R =
    crate::BitReader<CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A {
        match self.bits {
            false => CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A::DISABLE,
            true => CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A::ENABLE
    }
}
#[doc = "Field `ctl_rsfec_ieee_error_indication_mode` writer - "]
pub type CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, RSFEC_SPEC, O, CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A>;
impl<'a, const O: u8> CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_rsfec_enable(&self) -> CTL_RSFEC_ENABLE_R {
        CTL_RSFEC_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctl_rsfec_consortium_25g(&self) -> CTL_RSFEC_CONSORTIUM_25G_R {
        CTL_RSFEC_CONSORTIUM_25G_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ctl_rx_rsfec_enable_indication(&self) -> CTL_RX_RSFEC_ENABLE_INDICATION_R {
        CTL_RX_RSFEC_ENABLE_INDICATION_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ctl_rx_rsfec_enable_correction(&self) -> CTL_RX_RSFEC_ENABLE_CORRECTION_R {
        CTL_RX_RSFEC_ENABLE_CORRECTION_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ctl_rsfec_ieee_error_indication_mode(&self) -> CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_R {
        CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rsfec_enable(&mut self) -> CTL_RSFEC_ENABLE_W<0> {
        CTL_RSFEC_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rsfec_consortium_25g(&mut self) -> CTL_RSFEC_CONSORTIUM_25G_W<1> {
        CTL_RSFEC_CONSORTIUM_25G_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_rsfec_enable_indication(&mut self) -> CTL_RX_RSFEC_ENABLE_INDICATION_W<2> {
        CTL_RX_RSFEC_ENABLE_INDICATION_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_rsfec_enable_correction(&mut self) -> CTL_RX_RSFEC_ENABLE_CORRECTION_W<3> {
        CTL_RX_RSFEC_ENABLE_CORRECTION_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rsfec_ieee_error_indication_mode(
        &mut self,
    ) -> CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_W<5> {
        CTL_RSFEC_IEEE_ERROR_INDICATION_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Refec Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsfec](index.html) module"]
pub struct RSFEC_SPEC;
impl crate::RegisterSpec for RSFEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsfec::R](R) reader structure"]
impl crate::Readable for RSFEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsfec::W](W) writer structure"]
impl crate::Writable for RSFEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rsfec to value 0"]
impl crate::Resettable for RSFEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

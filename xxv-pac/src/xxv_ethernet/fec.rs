#[doc = "Register `fec` reader"]
pub struct R(crate::R<FEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fec` writer"]
pub struct W(crate::W<FEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEC_SPEC>;
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
impl From<crate::W<FEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_fec_rx_enable` reader - "]
pub type CTL_FEC_RX_ENABLE_R = crate::BitReader<CTL_FEC_RX_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_FEC_RX_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_FEC_RX_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_FEC_RX_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_FEC_RX_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_FEC_RX_ENABLE_A {
        match self.bits {
            false => CTL_FEC_RX_ENABLE_A::DISABLE,
            true => CTL_FEC_RX_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_FEC_RX_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_FEC_RX_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_fec_rx_enable` writer - "]
pub type CTL_FEC_RX_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, FEC_SPEC, O, CTL_FEC_RX_ENABLE_A>;
impl<'a, const O: u8> CTL_FEC_RX_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_FEC_RX_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_FEC_RX_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_fec_tx_enable` reader - "]
pub type CTL_FEC_TX_ENABLE_R = crate::BitReader<CTL_FEC_TX_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_FEC_TX_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_FEC_TX_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_FEC_TX_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_FEC_TX_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_FEC_TX_ENABLE_A {
        match self.bits {
            false => CTL_FEC_TX_ENABLE_A::DISABLE,
            true => CTL_FEC_TX_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_FEC_TX_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_FEC_TX_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_fec_tx_enable` writer - "]
pub type CTL_FEC_TX_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, FEC_SPEC, O, CTL_FEC_TX_ENABLE_A>;
impl<'a, const O: u8> CTL_FEC_TX_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_FEC_TX_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_FEC_TX_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_fec_enable_error_to_pcs` reader - "]
pub type CTL_FEC_ENABLE_ERROR_TO_PCS_R = crate::BitReader<CTL_FEC_ENABLE_ERROR_TO_PCS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_FEC_ENABLE_ERROR_TO_PCS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_FEC_ENABLE_ERROR_TO_PCS_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_FEC_ENABLE_ERROR_TO_PCS_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_FEC_ENABLE_ERROR_TO_PCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_FEC_ENABLE_ERROR_TO_PCS_A {
        match self.bits {
            false => CTL_FEC_ENABLE_ERROR_TO_PCS_A::DISABLE,
            true => CTL_FEC_ENABLE_ERROR_TO_PCS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_FEC_ENABLE_ERROR_TO_PCS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_FEC_ENABLE_ERROR_TO_PCS_A::ENABLE
    }
}
#[doc = "Field `ctl_fec_enable_error_to_pcs` writer - "]
pub type CTL_FEC_ENABLE_ERROR_TO_PCS_W<'a, const O: u8> =
    crate::BitWriter<'a, FEC_SPEC, O, CTL_FEC_ENABLE_ERROR_TO_PCS_A>;
impl<'a, const O: u8> CTL_FEC_ENABLE_ERROR_TO_PCS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_FEC_ENABLE_ERROR_TO_PCS_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_FEC_ENABLE_ERROR_TO_PCS_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_fec_rx_enable(&self) -> CTL_FEC_RX_ENABLE_R {
        CTL_FEC_RX_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctl_fec_tx_enable(&self) -> CTL_FEC_TX_ENABLE_R {
        CTL_FEC_TX_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ctl_fec_enable_error_to_pcs(&self) -> CTL_FEC_ENABLE_ERROR_TO_PCS_R {
        CTL_FEC_ENABLE_ERROR_TO_PCS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_fec_rx_enable(&mut self) -> CTL_FEC_RX_ENABLE_W<0> {
        CTL_FEC_RX_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_fec_tx_enable(&mut self) -> CTL_FEC_TX_ENABLE_W<1> {
        CTL_FEC_TX_ENABLE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_fec_enable_error_to_pcs(&mut self) -> CTL_FEC_ENABLE_ERROR_TO_PCS_W<2> {
        CTL_FEC_ENABLE_ERROR_TO_PCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fec Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fec](index.html) module"]
pub struct FEC_SPEC;
impl crate::RegisterSpec for FEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fec::R](R) reader structure"]
impl crate::Readable for FEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fec::W](W) writer structure"]
impl crate::Writable for FEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fec to value 0"]
impl crate::Resettable for FEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

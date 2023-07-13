#[doc = "Register `rst` reader"]
pub struct R(crate::R<RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rst` writer"]
pub struct W(crate::W<RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_SPEC>;
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
impl From<crate::W<RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_serdes_reset` reader - "]
pub type RX_SERDES_RESET_R = crate::BitReader<RX_SERDES_RESET_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_SERDES_RESET_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_SERDES_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RX_SERDES_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_SERDES_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_SERDES_RESET_A {
        match self.bits {
            false => RX_SERDES_RESET_A::DISABLE,
            true => RX_SERDES_RESET_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_SERDES_RESET_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_SERDES_RESET_A::ENABLE
    }
}
#[doc = "Field `rx_serdes_reset` writer - "]
pub type RX_SERDES_RESET_W<'a, const O: u8> = crate::BitWriter<'a, RST_SPEC, O, RX_SERDES_RESET_A>;
impl<'a, const O: u8> RX_SERDES_RESET_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_SERDES_RESET_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_SERDES_RESET_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_reset` reader - "]
pub type CTL_AN_RESET_R = crate::BitReader<CTL_AN_RESET_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_RESET_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_RESET_A {
        match self.bits {
            false => CTL_AN_RESET_A::DISABLE,
            true => CTL_AN_RESET_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_RESET_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_RESET_A::ENABLE
    }
}
#[doc = "Field `ctl_an_reset` writer - "]
pub type CTL_AN_RESET_W<'a, const O: u8> = crate::BitWriter<'a, RST_SPEC, O, CTL_AN_RESET_A>;
impl<'a, const O: u8> CTL_AN_RESET_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_RESET_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_RESET_A::ENABLE)
    }
}
#[doc = "Field `tx_serdes_reset` reader - "]
pub type TX_SERDES_RESET_R = crate::BitReader<TX_SERDES_RESET_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_SERDES_RESET_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_SERDES_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: TX_SERDES_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_SERDES_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_SERDES_RESET_A {
        match self.bits {
            false => TX_SERDES_RESET_A::DISABLE,
            true => TX_SERDES_RESET_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_SERDES_RESET_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_SERDES_RESET_A::ENABLE
    }
}
#[doc = "Field `tx_serdes_reset` writer - "]
pub type TX_SERDES_RESET_W<'a, const O: u8> = crate::BitWriter<'a, RST_SPEC, O, TX_SERDES_RESET_A>;
impl<'a, const O: u8> TX_SERDES_RESET_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_SERDES_RESET_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_SERDES_RESET_A::ENABLE)
    }
}
#[doc = "Field `rx_reset` reader - "]
pub type RX_RESET_R = crate::BitReader<RX_RESET_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_RESET_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RX_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_RESET_A {
        match self.bits {
            false => RX_RESET_A::DISABLE,
            true => RX_RESET_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_RESET_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_RESET_A::ENABLE
    }
}
#[doc = "Field `rx_reset` writer - "]
pub type RX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, RST_SPEC, O, RX_RESET_A>;
impl<'a, const O: u8> RX_RESET_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_RESET_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_RESET_A::ENABLE)
    }
}
#[doc = "Field `tx_reset` reader - "]
pub type TX_RESET_R = crate::BitReader<TX_RESET_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_RESET_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: TX_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_RESET_A {
        match self.bits {
            false => TX_RESET_A::DISABLE,
            true => TX_RESET_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TX_RESET_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TX_RESET_A::ENABLE
    }
}
#[doc = "Field `tx_reset` writer - "]
pub type TX_RESET_W<'a, const O: u8> = crate::BitWriter<'a, RST_SPEC, O, TX_RESET_A>;
impl<'a, const O: u8> TX_RESET_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_RESET_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_RESET_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_serdes_reset(&self) -> RX_SERDES_RESET_R {
        RX_SERDES_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ctl_an_reset(&self) -> CTL_AN_RESET_R {
        CTL_AN_RESET_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tx_serdes_reset(&self) -> TX_SERDES_RESET_R {
        TX_SERDES_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rx_reset(&self) -> RX_RESET_R {
        RX_RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_reset(&self) -> TX_RESET_R {
        TX_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_serdes_reset(&mut self) -> RX_SERDES_RESET_W<0> {
        RX_SERDES_RESET_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_reset(&mut self) -> CTL_AN_RESET_W<28> {
        CTL_AN_RESET_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn tx_serdes_reset(&mut self) -> TX_SERDES_RESET_W<29> {
        TX_SERDES_RESET_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rx_reset(&mut self) -> RX_RESET_W<30> {
        RX_RESET_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn tx_reset(&mut self) -> TX_RESET_W<31> {
        TX_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst](index.html) module"]
pub struct RST_SPEC;
impl crate::RegisterSpec for RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst::R](R) reader structure"]
impl crate::Readable for RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst::W](W) writer structure"]
impl crate::Writable for RST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rst to value 0"]
impl crate::Resettable for RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

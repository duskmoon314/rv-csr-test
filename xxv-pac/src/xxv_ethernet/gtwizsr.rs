#[doc = "Register `gtwizsr` reader"]
pub struct R(crate::R<GTWIZSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTWIZSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTWIZSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTWIZSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `gtwiz_reset_tx_done` reader - "]
pub type GTWIZ_RESET_TX_DONE_R = crate::BitReader<GTWIZ_RESET_TX_DONE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GTWIZ_RESET_TX_DONE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<GTWIZ_RESET_TX_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: GTWIZ_RESET_TX_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl GTWIZ_RESET_TX_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTWIZ_RESET_TX_DONE_A {
        match self.bits {
            false => GTWIZ_RESET_TX_DONE_A::DISABLE,
            true => GTWIZ_RESET_TX_DONE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GTWIZ_RESET_TX_DONE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GTWIZ_RESET_TX_DONE_A::ENABLE
    }
}
#[doc = "Field `gtwiz_reset_rx_done` reader - "]
pub type GTWIZ_RESET_RX_DONE_R = crate::BitReader<GTWIZ_RESET_RX_DONE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GTWIZ_RESET_RX_DONE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<GTWIZ_RESET_RX_DONE_A> for bool {
    #[inline(always)]
    fn from(variant: GTWIZ_RESET_RX_DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl GTWIZ_RESET_RX_DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTWIZ_RESET_RX_DONE_A {
        match self.bits {
            false => GTWIZ_RESET_RX_DONE_A::DISABLE,
            true => GTWIZ_RESET_RX_DONE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GTWIZ_RESET_RX_DONE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GTWIZ_RESET_RX_DONE_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gtwiz_reset_tx_done(&self) -> GTWIZ_RESET_TX_DONE_R {
        GTWIZ_RESET_TX_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gtwiz_reset_rx_done(&self) -> GTWIZ_RESET_RX_DONE_R {
        GTWIZ_RESET_RX_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Stat GT_WIZ Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtwizsr](index.html) module"]
pub struct GTWIZSR_SPEC;
impl crate::RegisterSpec for GTWIZSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtwizsr::R](R) reader structure"]
impl crate::Readable for GTWIZSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets gtwizsr to value 0"]
impl crate::Resettable for GTWIZSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

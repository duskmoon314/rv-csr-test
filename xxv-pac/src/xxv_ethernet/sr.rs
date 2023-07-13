#[doc = "Register `sr` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `stat_tx_fifo_error` reader - "]
pub type STAT_TX_FIFO_ERROR_R = crate::BitReader<STAT_TX_FIFO_ERROR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_TX_FIFO_ERROR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_TX_FIFO_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_TX_FIFO_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_TX_FIFO_ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_FIFO_ERROR_A {
        match self.bits {
            false => STAT_TX_FIFO_ERROR_A::DISABLE,
            true => STAT_TX_FIFO_ERROR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_TX_FIFO_ERROR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_TX_FIFO_ERROR_A::ENABLE
    }
}
#[doc = "Field `stat_tx_ptp_fifo_read_error` reader - "]
pub type STAT_TX_PTP_FIFO_READ_ERROR_R = crate::BitReader<STAT_TX_PTP_FIFO_READ_ERROR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_TX_PTP_FIFO_READ_ERROR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_TX_PTP_FIFO_READ_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_TX_PTP_FIFO_READ_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_TX_PTP_FIFO_READ_ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_PTP_FIFO_READ_ERROR_A {
        match self.bits {
            false => STAT_TX_PTP_FIFO_READ_ERROR_A::DISABLE,
            true => STAT_TX_PTP_FIFO_READ_ERROR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_TX_PTP_FIFO_READ_ERROR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_TX_PTP_FIFO_READ_ERROR_A::ENABLE
    }
}
#[doc = "Field `stat_tx_ptp_fifo_write_error` reader - "]
pub type STAT_TX_PTP_FIFO_WRITE_ERROR_R = crate::BitReader<STAT_TX_PTP_FIFO_WRITE_ERROR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_TX_PTP_FIFO_WRITE_ERROR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_TX_PTP_FIFO_WRITE_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_TX_PTP_FIFO_WRITE_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_TX_PTP_FIFO_WRITE_ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_PTP_FIFO_WRITE_ERROR_A {
        match self.bits {
            false => STAT_TX_PTP_FIFO_WRITE_ERROR_A::DISABLE,
            true => STAT_TX_PTP_FIFO_WRITE_ERROR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_TX_PTP_FIFO_WRITE_ERROR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_TX_PTP_FIFO_WRITE_ERROR_A::ENABLE
    }
}
#[doc = "Field `stat_rx_fifo_error` reader - "]
pub type STAT_RX_FIFO_ERROR_R = crate::BitReader<STAT_RX_FIFO_ERROR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_FIFO_ERROR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_FIFO_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_FIFO_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_FIFO_ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_FIFO_ERROR_A {
        match self.bits {
            false => STAT_RX_FIFO_ERROR_A::DISABLE,
            true => STAT_RX_FIFO_ERROR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_FIFO_ERROR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_FIFO_ERROR_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stat_tx_fifo_error(&self) -> STAT_TX_FIFO_ERROR_R {
        STAT_TX_FIFO_ERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn stat_tx_ptp_fifo_read_error(&self) -> STAT_TX_PTP_FIFO_READ_ERROR_R {
        STAT_TX_PTP_FIFO_READ_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stat_tx_ptp_fifo_write_error(&self) -> STAT_TX_PTP_FIFO_WRITE_ERROR_R {
        STAT_TX_PTP_FIFO_WRITE_ERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn stat_rx_fifo_error(&self) -> STAT_RX_FIFO_ERROR_R {
        STAT_RX_FIFO_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Stat Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sr to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `rxsr` reader"]
pub struct R(crate::R<RXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `stat_rx_status` reader - "]
pub type STAT_RX_STATUS_R = crate::BitReader<STAT_RX_STATUS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_STATUS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_STATUS_A {
        match self.bits {
            false => STAT_RX_STATUS_A::DISABLE,
            true => STAT_RX_STATUS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_STATUS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_STATUS_A::ENABLE
    }
}
#[doc = "Field `stat_rx_hi_ber` reader - "]
pub type STAT_RX_HI_BER_R = crate::BitReader<STAT_RX_HI_BER_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_HI_BER_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_HI_BER_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_HI_BER_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_HI_BER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_HI_BER_A {
        match self.bits {
            false => STAT_RX_HI_BER_A::DISABLE,
            true => STAT_RX_HI_BER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_HI_BER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_HI_BER_A::ENABLE
    }
}
#[doc = "Field `stat_rx_remote_fault` reader - "]
pub type STAT_RX_REMOTE_FAULT_R = crate::BitReader<STAT_RX_REMOTE_FAULT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_REMOTE_FAULT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_REMOTE_FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_REMOTE_FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_REMOTE_FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_REMOTE_FAULT_A {
        match self.bits {
            false => STAT_RX_REMOTE_FAULT_A::DISABLE,
            true => STAT_RX_REMOTE_FAULT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_REMOTE_FAULT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_REMOTE_FAULT_A::ENABLE
    }
}
#[doc = "Field `stat_rx_local_fault` reader - "]
pub type STAT_RX_LOCAL_FAULT_R = crate::BitReader<STAT_RX_LOCAL_FAULT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_LOCAL_FAULT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_LOCAL_FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_LOCAL_FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_LOCAL_FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_LOCAL_FAULT_A {
        match self.bits {
            false => STAT_RX_LOCAL_FAULT_A::DISABLE,
            true => STAT_RX_LOCAL_FAULT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_LOCAL_FAULT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_LOCAL_FAULT_A::ENABLE
    }
}
#[doc = "Field `stat_rx_internal_local_fault` reader - "]
pub type STAT_RX_INTERNAL_LOCAL_FAULT_R = crate::BitReader<STAT_RX_INTERNAL_LOCAL_FAULT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_INTERNAL_LOCAL_FAULT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_INTERNAL_LOCAL_FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_INTERNAL_LOCAL_FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_INTERNAL_LOCAL_FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_INTERNAL_LOCAL_FAULT_A {
        match self.bits {
            false => STAT_RX_INTERNAL_LOCAL_FAULT_A::DISABLE,
            true => STAT_RX_INTERNAL_LOCAL_FAULT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_INTERNAL_LOCAL_FAULT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_INTERNAL_LOCAL_FAULT_A::ENABLE
    }
}
#[doc = "Field `stat_rx_received_local_fault` reader - "]
pub type STAT_RX_RECEIVED_LOCAL_FAULT_R = crate::BitReader<STAT_RX_RECEIVED_LOCAL_FAULT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_RECEIVED_LOCAL_FAULT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_RECEIVED_LOCAL_FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_RECEIVED_LOCAL_FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_RECEIVED_LOCAL_FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_RECEIVED_LOCAL_FAULT_A {
        match self.bits {
            false => STAT_RX_RECEIVED_LOCAL_FAULT_A::DISABLE,
            true => STAT_RX_RECEIVED_LOCAL_FAULT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_RECEIVED_LOCAL_FAULT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_RECEIVED_LOCAL_FAULT_A::ENABLE
    }
}
#[doc = "Field `stat_rx_bad_preamble` reader - "]
pub type STAT_RX_BAD_PREAMBLE_R = crate::BitReader<STAT_RX_BAD_PREAMBLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_BAD_PREAMBLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_BAD_PREAMBLE_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_BAD_PREAMBLE_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_BAD_PREAMBLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_BAD_PREAMBLE_A {
        match self.bits {
            false => STAT_RX_BAD_PREAMBLE_A::DISABLE,
            true => STAT_RX_BAD_PREAMBLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_BAD_PREAMBLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_BAD_PREAMBLE_A::ENABLE
    }
}
#[doc = "Field `stat_rx_bad_sfd` reader - "]
pub type STAT_RX_BAD_SFD_R = crate::BitReader<STAT_RX_BAD_SFD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_BAD_SFD_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_BAD_SFD_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_BAD_SFD_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_BAD_SFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_BAD_SFD_A {
        match self.bits {
            false => STAT_RX_BAD_SFD_A::DISABLE,
            true => STAT_RX_BAD_SFD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_BAD_SFD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_BAD_SFD_A::ENABLE
    }
}
#[doc = "Field `stat_rx_got_signal_os` reader - "]
pub type STAT_RX_GOT_SIGNAL_OS_R = crate::BitReader<STAT_RX_GOT_SIGNAL_OS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_GOT_SIGNAL_OS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_GOT_SIGNAL_OS_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_GOT_SIGNAL_OS_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_GOT_SIGNAL_OS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_GOT_SIGNAL_OS_A {
        match self.bits {
            false => STAT_RX_GOT_SIGNAL_OS_A::DISABLE,
            true => STAT_RX_GOT_SIGNAL_OS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_GOT_SIGNAL_OS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_GOT_SIGNAL_OS_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stat_rx_status(&self) -> STAT_RX_STATUS_R {
        STAT_RX_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn stat_rx_hi_ber(&self) -> STAT_RX_HI_BER_R {
        STAT_RX_HI_BER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stat_rx_remote_fault(&self) -> STAT_RX_REMOTE_FAULT_R {
        STAT_RX_REMOTE_FAULT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn stat_rx_local_fault(&self) -> STAT_RX_LOCAL_FAULT_R {
        STAT_RX_LOCAL_FAULT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stat_rx_internal_local_fault(&self) -> STAT_RX_INTERNAL_LOCAL_FAULT_R {
        STAT_RX_INTERNAL_LOCAL_FAULT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn stat_rx_received_local_fault(&self) -> STAT_RX_RECEIVED_LOCAL_FAULT_R {
        STAT_RX_RECEIVED_LOCAL_FAULT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn stat_rx_bad_preamble(&self) -> STAT_RX_BAD_PREAMBLE_R {
        STAT_RX_BAD_PREAMBLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn stat_rx_bad_sfd(&self) -> STAT_RX_BAD_SFD_R {
        STAT_RX_BAD_SFD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stat_rx_got_signal_os(&self) -> STAT_RX_GOT_SIGNAL_OS_R {
        STAT_RX_GOT_SIGNAL_OS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Stat RX Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxsr](index.html) module"]
pub struct RXSR_SPEC;
impl crate::RegisterSpec for RXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxsr::R](R) reader structure"]
impl crate::Readable for RXSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxsr to value 0"]
impl crate::Resettable for RXSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

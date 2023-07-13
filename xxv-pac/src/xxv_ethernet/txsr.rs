#[doc = "Register `txsr` reader"]
pub struct R(crate::R<TXSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `stat_tx_local_fault` reader - "]
pub type STAT_TX_LOCAL_FAULT_R = crate::BitReader<STAT_TX_LOCAL_FAULT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_TX_LOCAL_FAULT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_TX_LOCAL_FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_TX_LOCAL_FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_TX_LOCAL_FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_LOCAL_FAULT_A {
        match self.bits {
            false => STAT_TX_LOCAL_FAULT_A::DISABLE,
            true => STAT_TX_LOCAL_FAULT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_TX_LOCAL_FAULT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_TX_LOCAL_FAULT_A::ENABLE
    }
}
#[doc = "Field `stat_tx_gmii_fifo_ovf_1h_r_out` reader - "]
pub type STAT_TX_GMII_FIFO_OVF_1H_R_OUT_R = crate::BitReader<STAT_TX_GMII_FIFO_OVF_1H_R_OUT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_TX_GMII_FIFO_OVF_1H_R_OUT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_TX_GMII_FIFO_OVF_1H_R_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_TX_GMII_FIFO_OVF_1H_R_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_TX_GMII_FIFO_OVF_1H_R_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_GMII_FIFO_OVF_1H_R_OUT_A {
        match self.bits {
            false => STAT_TX_GMII_FIFO_OVF_1H_R_OUT_A::DISABLE,
            true => STAT_TX_GMII_FIFO_OVF_1H_R_OUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_TX_GMII_FIFO_OVF_1H_R_OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_TX_GMII_FIFO_OVF_1H_R_OUT_A::ENABLE
    }
}
#[doc = "Field `stat_tx_gmii_fifo_unf_1h_r_out` reader - "]
pub type STAT_TX_GMII_FIFO_UNF_1H_R_OUT_R = crate::BitReader<STAT_TX_GMII_FIFO_UNF_1H_R_OUT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_TX_GMII_FIFO_UNF_1H_R_OUT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_TX_GMII_FIFO_UNF_1H_R_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_TX_GMII_FIFO_UNF_1H_R_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_TX_GMII_FIFO_UNF_1H_R_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_GMII_FIFO_UNF_1H_R_OUT_A {
        match self.bits {
            false => STAT_TX_GMII_FIFO_UNF_1H_R_OUT_A::DISABLE,
            true => STAT_TX_GMII_FIFO_UNF_1H_R_OUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_TX_GMII_FIFO_UNF_1H_R_OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_TX_GMII_FIFO_UNF_1H_R_OUT_A::ENABLE
    }
}
#[doc = "Field `stat_tx_bad_parity` reader - "]
pub type STAT_TX_BAD_PARITY_R = crate::BitReader<STAT_TX_BAD_PARITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_TX_BAD_PARITY_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_TX_BAD_PARITY_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_TX_BAD_PARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_TX_BAD_PARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_BAD_PARITY_A {
        match self.bits {
            false => STAT_TX_BAD_PARITY_A::DISABLE,
            true => STAT_TX_BAD_PARITY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_TX_BAD_PARITY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_TX_BAD_PARITY_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stat_tx_local_fault(&self) -> STAT_TX_LOCAL_FAULT_R {
        STAT_TX_LOCAL_FAULT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn stat_tx_gmii_fifo_ovf_1h_r_out(&self) -> STAT_TX_GMII_FIFO_OVF_1H_R_OUT_R {
        STAT_TX_GMII_FIFO_OVF_1H_R_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn stat_tx_gmii_fifo_unf_1h_r_out(&self) -> STAT_TX_GMII_FIFO_UNF_1H_R_OUT_R {
        STAT_TX_GMII_FIFO_UNF_1H_R_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stat_tx_bad_parity(&self) -> STAT_TX_BAD_PARITY_R {
        STAT_TX_BAD_PARITY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Stat TX Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txsr](index.html) module"]
pub struct TXSR_SPEC;
impl crate::RegisterSpec for TXSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txsr::R](R) reader structure"]
impl crate::Readable for TXSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txsr to value 0"]
impl crate::Resettable for TXSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

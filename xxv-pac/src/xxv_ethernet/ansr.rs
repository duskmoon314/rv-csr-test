#[doc = "Register `ansr` reader"]
pub struct R(crate::R<ANSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `stat_an_fec_enable` reader - "]
pub type STAT_AN_FEC_ENABLE_R = crate::BitReader<STAT_AN_FEC_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_FEC_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_FEC_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_FEC_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_FEC_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_FEC_ENABLE_A {
        match self.bits {
            false => STAT_AN_FEC_ENABLE_A::DISABLE,
            true => STAT_AN_FEC_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_FEC_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_FEC_ENABLE_A::ENABLE
    }
}
#[doc = "Field `stat_an_rs_fec_enable` reader - "]
pub type STAT_AN_RS_FEC_ENABLE_R = crate::BitReader<STAT_AN_RS_FEC_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_RS_FEC_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_RS_FEC_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_RS_FEC_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_RS_FEC_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_RS_FEC_ENABLE_A {
        match self.bits {
            false => STAT_AN_RS_FEC_ENABLE_A::DISABLE,
            true => STAT_AN_RS_FEC_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_RS_FEC_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_RS_FEC_ENABLE_A::ENABLE
    }
}
#[doc = "Field `stat_an_autoneg_complete` reader - "]
pub type STAT_AN_AUTONEG_COMPLETE_R = crate::BitReader<STAT_AN_AUTONEG_COMPLETE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_AUTONEG_COMPLETE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_AUTONEG_COMPLETE_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_AUTONEG_COMPLETE_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_AUTONEG_COMPLETE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_AUTONEG_COMPLETE_A {
        match self.bits {
            false => STAT_AN_AUTONEG_COMPLETE_A::DISABLE,
            true => STAT_AN_AUTONEG_COMPLETE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_AUTONEG_COMPLETE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_AUTONEG_COMPLETE_A::ENABLE
    }
}
#[doc = "Field `stat_an_parallel_detection_fault` reader - "]
pub type STAT_AN_PARALLEL_DETECTION_FAULT_R = crate::BitReader<STAT_AN_PARALLEL_DETECTION_FAULT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_PARALLEL_DETECTION_FAULT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_PARALLEL_DETECTION_FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_PARALLEL_DETECTION_FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_PARALLEL_DETECTION_FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_PARALLEL_DETECTION_FAULT_A {
        match self.bits {
            false => STAT_AN_PARALLEL_DETECTION_FAULT_A::DISABLE,
            true => STAT_AN_PARALLEL_DETECTION_FAULT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_PARALLEL_DETECTION_FAULT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_PARALLEL_DETECTION_FAULT_A::ENABLE
    }
}
#[doc = "Field `stat_an_tx_pause_enable` reader - "]
pub type STAT_AN_TX_PAUSE_ENABLE_R = crate::BitReader<STAT_AN_TX_PAUSE_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_TX_PAUSE_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_TX_PAUSE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_TX_PAUSE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_TX_PAUSE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_TX_PAUSE_ENABLE_A {
        match self.bits {
            false => STAT_AN_TX_PAUSE_ENABLE_A::DISABLE,
            true => STAT_AN_TX_PAUSE_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_TX_PAUSE_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_TX_PAUSE_ENABLE_A::ENABLE
    }
}
#[doc = "Field `stat_an_rx_pause_enable` reader - "]
pub type STAT_AN_RX_PAUSE_ENABLE_R = crate::BitReader<STAT_AN_RX_PAUSE_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_RX_PAUSE_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_RX_PAUSE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_RX_PAUSE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_RX_PAUSE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_RX_PAUSE_ENABLE_A {
        match self.bits {
            false => STAT_AN_RX_PAUSE_ENABLE_A::DISABLE,
            true => STAT_AN_RX_PAUSE_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_RX_PAUSE_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_RX_PAUSE_ENABLE_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_valid` reader - "]
pub type STAT_AN_LP_ABILITY_VALID_R = crate::BitReader<STAT_AN_LP_ABILITY_VALID_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_VALID_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_VALID_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_VALID_A::DISABLE,
            true => STAT_AN_LP_ABILITY_VALID_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_VALID_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_VALID_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_autoneg_able` reader - "]
pub type STAT_AN_LP_AUTONEG_ABLE_R = crate::BitReader<STAT_AN_LP_AUTONEG_ABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_AUTONEG_ABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_AUTONEG_ABLE_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_AUTONEG_ABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_AUTONEG_ABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_AUTONEG_ABLE_A {
        match self.bits {
            false => STAT_AN_LP_AUTONEG_ABLE_A::DISABLE,
            true => STAT_AN_LP_AUTONEG_ABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_AUTONEG_ABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_AUTONEG_ABLE_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_pause` reader - "]
pub type STAT_AN_LP_PAUSE_R = crate::BitReader<STAT_AN_LP_PAUSE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_PAUSE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_PAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_PAUSE_A {
        match self.bits {
            false => STAT_AN_LP_PAUSE_A::DISABLE,
            true => STAT_AN_LP_PAUSE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_PAUSE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_PAUSE_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_asm_dir` reader - "]
pub type STAT_AN_LP_ASM_DIR_R = crate::BitReader<STAT_AN_LP_ASM_DIR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ASM_DIR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ASM_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ASM_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ASM_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ASM_DIR_A {
        match self.bits {
            false => STAT_AN_LP_ASM_DIR_A::DISABLE,
            true => STAT_AN_LP_ASM_DIR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ASM_DIR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ASM_DIR_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_rf` reader - "]
pub type STAT_AN_LP_RF_R = crate::BitReader<STAT_AN_LP_RF_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_RF_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_RF_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_RF_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_RF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_RF_A {
        match self.bits {
            false => STAT_AN_LP_RF_A::DISABLE,
            true => STAT_AN_LP_RF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_RF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_RF_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_fec_10g_ability` reader - "]
pub type STAT_AN_LP_FEC_10G_ABILITY_R = crate::BitReader<STAT_AN_LP_FEC_10G_ABILITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_FEC_10G_ABILITY_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_FEC_10G_ABILITY_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_FEC_10G_ABILITY_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_FEC_10G_ABILITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_FEC_10G_ABILITY_A {
        match self.bits {
            false => STAT_AN_LP_FEC_10G_ABILITY_A::DISABLE,
            true => STAT_AN_LP_FEC_10G_ABILITY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_FEC_10G_ABILITY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_FEC_10G_ABILITY_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_fec_10g_request` reader - "]
pub type STAT_AN_LP_FEC_10G_REQUEST_R = crate::BitReader<STAT_AN_LP_FEC_10G_REQUEST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_FEC_10G_REQUEST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_FEC_10G_REQUEST_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_FEC_10G_REQUEST_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_FEC_10G_REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_FEC_10G_REQUEST_A {
        match self.bits {
            false => STAT_AN_LP_FEC_10G_REQUEST_A::DISABLE,
            true => STAT_AN_LP_FEC_10G_REQUEST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_FEC_10G_REQUEST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_FEC_10G_REQUEST_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_extended_ability_valid` reader - "]
pub type STAT_AN_LP_EXTENDED_ABILITY_VALID_R =
    crate::BitReader<STAT_AN_LP_EXTENDED_ABILITY_VALID_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_EXTENDED_ABILITY_VALID_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_EXTENDED_ABILITY_VALID_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_EXTENDED_ABILITY_VALID_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_EXTENDED_ABILITY_VALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_EXTENDED_ABILITY_VALID_A {
        match self.bits {
            false => STAT_AN_LP_EXTENDED_ABILITY_VALID_A::DISABLE,
            true => STAT_AN_LP_EXTENDED_ABILITY_VALID_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_EXTENDED_ABILITY_VALID_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_EXTENDED_ABILITY_VALID_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_extended_fec` reader - "]
pub type STAT_AN_LP_ABILITY_EXTENDED_FEC_R = crate::FieldReader;
#[doc = "Field `stat_an_lp_fec_25g_rs_request` reader - "]
pub type STAT_AN_LP_FEC_25G_RS_REQUEST_R = crate::BitReader<STAT_AN_LP_FEC_25G_RS_REQUEST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_FEC_25G_RS_REQUEST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_FEC_25G_RS_REQUEST_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_FEC_25G_RS_REQUEST_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_FEC_25G_RS_REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_FEC_25G_RS_REQUEST_A {
        match self.bits {
            false => STAT_AN_LP_FEC_25G_RS_REQUEST_A::DISABLE,
            true => STAT_AN_LP_FEC_25G_RS_REQUEST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_FEC_25G_RS_REQUEST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_FEC_25G_RS_REQUEST_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_fec_25g_baser_request` reader - "]
pub type STAT_AN_LP_FEC_25G_BASER_REQUEST_R = crate::BitReader<STAT_AN_LP_FEC_25G_BASER_REQUEST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_FEC_25G_BASER_REQUEST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_FEC_25G_BASER_REQUEST_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_FEC_25G_BASER_REQUEST_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_FEC_25G_BASER_REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_FEC_25G_BASER_REQUEST_A {
        match self.bits {
            false => STAT_AN_LP_FEC_25G_BASER_REQUEST_A::DISABLE,
            true => STAT_AN_LP_FEC_25G_BASER_REQUEST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_FEC_25G_BASER_REQUEST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_FEC_25G_BASER_REQUEST_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stat_an_fec_enable(&self) -> STAT_AN_FEC_ENABLE_R {
        STAT_AN_FEC_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn stat_an_rs_fec_enable(&self) -> STAT_AN_RS_FEC_ENABLE_R {
        STAT_AN_RS_FEC_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn stat_an_autoneg_complete(&self) -> STAT_AN_AUTONEG_COMPLETE_R {
        STAT_AN_AUTONEG_COMPLETE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn stat_an_parallel_detection_fault(&self) -> STAT_AN_PARALLEL_DETECTION_FAULT_R {
        STAT_AN_PARALLEL_DETECTION_FAULT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn stat_an_tx_pause_enable(&self) -> STAT_AN_TX_PAUSE_ENABLE_R {
        STAT_AN_TX_PAUSE_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stat_an_rx_pause_enable(&self) -> STAT_AN_RX_PAUSE_ENABLE_R {
        STAT_AN_RX_PAUSE_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn stat_an_lp_ability_valid(&self) -> STAT_AN_LP_ABILITY_VALID_R {
        STAT_AN_LP_ABILITY_VALID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stat_an_lp_autoneg_able(&self) -> STAT_AN_LP_AUTONEG_ABLE_R {
        STAT_AN_LP_AUTONEG_ABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn stat_an_lp_pause(&self) -> STAT_AN_LP_PAUSE_R {
        STAT_AN_LP_PAUSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn stat_an_lp_asm_dir(&self) -> STAT_AN_LP_ASM_DIR_R {
        STAT_AN_LP_ASM_DIR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn stat_an_lp_rf(&self) -> STAT_AN_LP_RF_R {
        STAT_AN_LP_RF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stat_an_lp_fec_10g_ability(&self) -> STAT_AN_LP_FEC_10G_ABILITY_R {
        STAT_AN_LP_FEC_10G_ABILITY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn stat_an_lp_fec_10g_request(&self) -> STAT_AN_LP_FEC_10G_REQUEST_R {
        STAT_AN_LP_FEC_10G_REQUEST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn stat_an_lp_extended_ability_valid(&self) -> STAT_AN_LP_EXTENDED_ABILITY_VALID_R {
        STAT_AN_LP_EXTENDED_ABILITY_VALID_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn stat_an_lp_ability_extended_fec(&self) -> STAT_AN_LP_ABILITY_EXTENDED_FEC_R {
        STAT_AN_LP_ABILITY_EXTENDED_FEC_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn stat_an_lp_fec_25g_rs_request(&self) -> STAT_AN_LP_FEC_25G_RS_REQUEST_R {
        STAT_AN_LP_FEC_25G_RS_REQUEST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn stat_an_lp_fec_25g_baser_request(&self) -> STAT_AN_LP_FEC_25G_BASER_REQUEST_R {
        STAT_AN_LP_FEC_25G_BASER_REQUEST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Stat AN_STATUS Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ansr](index.html) module"]
pub struct ANSR_SPEC;
impl crate::RegisterSpec for ANSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ansr::R](R) reader structure"]
impl crate::Readable for ANSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ansr to value 0"]
impl crate::Resettable for ANSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

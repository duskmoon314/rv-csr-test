#[doc = "Register `anasr` reader"]
pub struct R(crate::R<ANASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `stat_an_lp_ability_1000base_kx` reader - "]
pub type STAT_AN_LP_ABILITY_1000BASE_KX_R = crate::BitReader<STAT_AN_LP_ABILITY_1000BASE_KX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_1000BASE_KX_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_1000BASE_KX_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_1000BASE_KX_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_1000BASE_KX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_1000BASE_KX_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_1000BASE_KX_A::DISABLE,
            true => STAT_AN_LP_ABILITY_1000BASE_KX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_1000BASE_KX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_1000BASE_KX_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_10gbase_kx4` reader - "]
pub type STAT_AN_LP_ABILITY_10GBASE_KX4_R = crate::BitReader<STAT_AN_LP_ABILITY_10GBASE_KX4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_10GBASE_KX4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_10GBASE_KX4_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_10GBASE_KX4_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_10GBASE_KX4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_10GBASE_KX4_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_10GBASE_KX4_A::DISABLE,
            true => STAT_AN_LP_ABILITY_10GBASE_KX4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_10GBASE_KX4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_10GBASE_KX4_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_10gbase_kr` reader - "]
pub type STAT_AN_LP_ABILITY_10GBASE_KR_R = crate::BitReader<STAT_AN_LP_ABILITY_10GBASE_KR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_10GBASE_KR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_10GBASE_KR_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_10GBASE_KR_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_10GBASE_KR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_10GBASE_KR_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_10GBASE_KR_A::DISABLE,
            true => STAT_AN_LP_ABILITY_10GBASE_KR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_10GBASE_KR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_10GBASE_KR_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_40gbase_kr4` reader - "]
pub type STAT_AN_LP_ABILITY_40GBASE_KR4_R = crate::BitReader<STAT_AN_LP_ABILITY_40GBASE_KR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_40GBASE_KR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_40GBASE_KR4_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_40GBASE_KR4_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_40GBASE_KR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_40GBASE_KR4_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_40GBASE_KR4_A::DISABLE,
            true => STAT_AN_LP_ABILITY_40GBASE_KR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_40GBASE_KR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_40GBASE_KR4_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_40gbase_cr4` reader - "]
pub type STAT_AN_LP_ABILITY_40GBASE_CR4_R = crate::BitReader<STAT_AN_LP_ABILITY_40GBASE_CR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_40GBASE_CR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_40GBASE_CR4_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_40GBASE_CR4_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_40GBASE_CR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_40GBASE_CR4_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_40GBASE_CR4_A::DISABLE,
            true => STAT_AN_LP_ABILITY_40GBASE_CR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_40GBASE_CR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_40GBASE_CR4_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_100gbase_cr10` reader - "]
pub type STAT_AN_LP_ABILITY_100GBASE_CR10_R = crate::BitReader<STAT_AN_LP_ABILITY_100GBASE_CR10_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_100GBASE_CR10_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_100GBASE_CR10_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_100GBASE_CR10_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_100GBASE_CR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_100GBASE_CR10_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_100GBASE_CR10_A::DISABLE,
            true => STAT_AN_LP_ABILITY_100GBASE_CR10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_CR10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_CR10_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_100gbase_kp4` reader - "]
pub type STAT_AN_LP_ABILITY_100GBASE_KP4_R = crate::BitReader<STAT_AN_LP_ABILITY_100GBASE_KP4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_100GBASE_KP4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_100GBASE_KP4_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_100GBASE_KP4_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_100GBASE_KP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_100GBASE_KP4_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_100GBASE_KP4_A::DISABLE,
            true => STAT_AN_LP_ABILITY_100GBASE_KP4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_KP4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_KP4_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_100gbase_kr4` reader - "]
pub type STAT_AN_LP_ABILITY_100GBASE_KR4_R = crate::BitReader<STAT_AN_LP_ABILITY_100GBASE_KR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_100GBASE_KR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_100GBASE_KR4_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_100GBASE_KR4_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_100GBASE_KR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_100GBASE_KR4_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_100GBASE_KR4_A::DISABLE,
            true => STAT_AN_LP_ABILITY_100GBASE_KR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_KR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_KR4_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_100gbase_cr4` reader - "]
pub type STAT_AN_LP_ABILITY_100GBASE_CR4_R = crate::BitReader<STAT_AN_LP_ABILITY_100GBASE_CR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_100GBASE_CR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_100GBASE_CR4_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_100GBASE_CR4_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_100GBASE_CR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_100GBASE_CR4_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_100GBASE_CR4_A::DISABLE,
            true => STAT_AN_LP_ABILITY_100GBASE_CR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_CR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_CR4_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_25gbase_krcr_s` reader - "]
pub type STAT_AN_LP_ABILITY_25GBASE_KRCR_S_R =
    crate::BitReader<STAT_AN_LP_ABILITY_25GBASE_KRCR_S_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_25GBASE_KRCR_S_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_25GBASE_KRCR_S_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_25GBASE_KRCR_S_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_25GBASE_KRCR_S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_25GBASE_KRCR_S_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_25GBASE_KRCR_S_A::DISABLE,
            true => STAT_AN_LP_ABILITY_25GBASE_KRCR_S_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_25GBASE_KRCR_S_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_25GBASE_KRCR_S_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_25gbase_krcr` reader - "]
pub type STAT_AN_LP_ABILITY_25GBASE_KRCR_R = crate::BitReader<STAT_AN_LP_ABILITY_25GBASE_KRCR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_25GBASE_KRCR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_25GBASE_KRCR_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_25GBASE_KRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_25GBASE_KRCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_25GBASE_KRCR_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_25GBASE_KRCR_A::DISABLE,
            true => STAT_AN_LP_ABILITY_25GBASE_KRCR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_25GBASE_KRCR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_25GBASE_KRCR_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_2_5gbase_kx` reader - "]
pub type STAT_AN_LP_ABILITY_2_5GBASE_KX_R = crate::BitReader<STAT_AN_LP_ABILITY_2_5GBASE_KX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_2_5GBASE_KX_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_2_5GBASE_KX_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_2_5GBASE_KX_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_2_5GBASE_KX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_2_5GBASE_KX_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_2_5GBASE_KX_A::DISABLE,
            true => STAT_AN_LP_ABILITY_2_5GBASE_KX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_2_5GBASE_KX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_2_5GBASE_KX_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_5gbase_kr` reader - "]
pub type STAT_AN_LP_ABILITY_5GBASE_KR_R = crate::BitReader<STAT_AN_LP_ABILITY_5GBASE_KR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_5GBASE_KR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_5GBASE_KR_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_5GBASE_KR_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_5GBASE_KR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_5GBASE_KR_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_5GBASE_KR_A::DISABLE,
            true => STAT_AN_LP_ABILITY_5GBASE_KR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_5GBASE_KR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_5GBASE_KR_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_50gbase_krcr` reader - "]
pub type STAT_AN_LP_ABILITY_50GBASE_KRCR_R = crate::BitReader<STAT_AN_LP_ABILITY_50GBASE_KRCR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_50GBASE_KRCR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_50GBASE_KRCR_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_50GBASE_KRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_50GBASE_KRCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_50GBASE_KRCR_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_50GBASE_KRCR_A::DISABLE,
            true => STAT_AN_LP_ABILITY_50GBASE_KRCR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_50GBASE_KRCR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_50GBASE_KRCR_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_100gbase_kr2cr2` reader - "]
pub type STAT_AN_LP_ABILITY_100GBASE_KR2CR2_R =
    crate::BitReader<STAT_AN_LP_ABILITY_100GBASE_KR2CR2_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_100GBASE_KR2CR2_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_100GBASE_KR2CR2_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_100GBASE_KR2CR2_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_100GBASE_KR2CR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_100GBASE_KR2CR2_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_100GBASE_KR2CR2_A::DISABLE,
            true => STAT_AN_LP_ABILITY_100GBASE_KR2CR2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_KR2CR2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_100GBASE_KR2CR2_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_200gbase_kr4cr4` reader - "]
pub type STAT_AN_LP_ABILITY_200GBASE_KR4CR4_R =
    crate::BitReader<STAT_AN_LP_ABILITY_200GBASE_KR4CR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_200GBASE_KR4CR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_200GBASE_KR4CR4_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_200GBASE_KR4CR4_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_200GBASE_KR4CR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_200GBASE_KR4CR4_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_200GBASE_KR4CR4_A::DISABLE,
            true => STAT_AN_LP_ABILITY_200GBASE_KR4CR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_200GBASE_KR4CR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_200GBASE_KR4CR4_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_25gbase_kr1` reader - "]
pub type STAT_AN_LP_ABILITY_25GBASE_KR1_R = crate::BitReader<STAT_AN_LP_ABILITY_25GBASE_KR1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_25GBASE_KR1_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_25GBASE_KR1_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_25GBASE_KR1_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_25GBASE_KR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_25GBASE_KR1_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_25GBASE_KR1_A::DISABLE,
            true => STAT_AN_LP_ABILITY_25GBASE_KR1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_25GBASE_KR1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_25GBASE_KR1_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_25gbase_cr1` reader - "]
pub type STAT_AN_LP_ABILITY_25GBASE_CR1_R = crate::BitReader<STAT_AN_LP_ABILITY_25GBASE_CR1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_25GBASE_CR1_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_25GBASE_CR1_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_25GBASE_CR1_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_25GBASE_CR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_25GBASE_CR1_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_25GBASE_CR1_A::DISABLE,
            true => STAT_AN_LP_ABILITY_25GBASE_CR1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_25GBASE_CR1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_25GBASE_CR1_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_50gbase_kr2` reader - "]
pub type STAT_AN_LP_ABILITY_50GBASE_KR2_R = crate::BitReader<STAT_AN_LP_ABILITY_50GBASE_KR2_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_50GBASE_KR2_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_50GBASE_KR2_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_50GBASE_KR2_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_50GBASE_KR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_50GBASE_KR2_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_50GBASE_KR2_A::DISABLE,
            true => STAT_AN_LP_ABILITY_50GBASE_KR2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_50GBASE_KR2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_50GBASE_KR2_A::ENABLE
    }
}
#[doc = "Field `stat_an_lp_ability_50gbase_cr2` reader - "]
pub type STAT_AN_LP_ABILITY_50GBASE_CR2_R = crate::BitReader<STAT_AN_LP_ABILITY_50GBASE_CR2_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_AN_LP_ABILITY_50GBASE_CR2_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_AN_LP_ABILITY_50GBASE_CR2_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_AN_LP_ABILITY_50GBASE_CR2_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_AN_LP_ABILITY_50GBASE_CR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_AN_LP_ABILITY_50GBASE_CR2_A {
        match self.bits {
            false => STAT_AN_LP_ABILITY_50GBASE_CR2_A::DISABLE,
            true => STAT_AN_LP_ABILITY_50GBASE_CR2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_50GBASE_CR2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_AN_LP_ABILITY_50GBASE_CR2_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stat_an_lp_ability_1000base_kx(&self) -> STAT_AN_LP_ABILITY_1000BASE_KX_R {
        STAT_AN_LP_ABILITY_1000BASE_KX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn stat_an_lp_ability_10gbase_kx4(&self) -> STAT_AN_LP_ABILITY_10GBASE_KX4_R {
        STAT_AN_LP_ABILITY_10GBASE_KX4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn stat_an_lp_ability_10gbase_kr(&self) -> STAT_AN_LP_ABILITY_10GBASE_KR_R {
        STAT_AN_LP_ABILITY_10GBASE_KR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn stat_an_lp_ability_40gbase_kr4(&self) -> STAT_AN_LP_ABILITY_40GBASE_KR4_R {
        STAT_AN_LP_ABILITY_40GBASE_KR4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn stat_an_lp_ability_40gbase_cr4(&self) -> STAT_AN_LP_ABILITY_40GBASE_CR4_R {
        STAT_AN_LP_ABILITY_40GBASE_CR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stat_an_lp_ability_100gbase_cr10(&self) -> STAT_AN_LP_ABILITY_100GBASE_CR10_R {
        STAT_AN_LP_ABILITY_100GBASE_CR10_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn stat_an_lp_ability_100gbase_kp4(&self) -> STAT_AN_LP_ABILITY_100GBASE_KP4_R {
        STAT_AN_LP_ABILITY_100GBASE_KP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stat_an_lp_ability_100gbase_kr4(&self) -> STAT_AN_LP_ABILITY_100GBASE_KR4_R {
        STAT_AN_LP_ABILITY_100GBASE_KR4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn stat_an_lp_ability_100gbase_cr4(&self) -> STAT_AN_LP_ABILITY_100GBASE_CR4_R {
        STAT_AN_LP_ABILITY_100GBASE_CR4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn stat_an_lp_ability_25gbase_krcr_s(&self) -> STAT_AN_LP_ABILITY_25GBASE_KRCR_S_R {
        STAT_AN_LP_ABILITY_25GBASE_KRCR_S_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn stat_an_lp_ability_25gbase_krcr(&self) -> STAT_AN_LP_ABILITY_25GBASE_KRCR_R {
        STAT_AN_LP_ABILITY_25GBASE_KRCR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn stat_an_lp_ability_2_5gbase_kx(&self) -> STAT_AN_LP_ABILITY_2_5GBASE_KX_R {
        STAT_AN_LP_ABILITY_2_5GBASE_KX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn stat_an_lp_ability_5gbase_kr(&self) -> STAT_AN_LP_ABILITY_5GBASE_KR_R {
        STAT_AN_LP_ABILITY_5GBASE_KR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn stat_an_lp_ability_50gbase_krcr(&self) -> STAT_AN_LP_ABILITY_50GBASE_KRCR_R {
        STAT_AN_LP_ABILITY_50GBASE_KRCR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn stat_an_lp_ability_100gbase_kr2cr2(&self) -> STAT_AN_LP_ABILITY_100GBASE_KR2CR2_R {
        STAT_AN_LP_ABILITY_100GBASE_KR2CR2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stat_an_lp_ability_200gbase_kr4cr4(&self) -> STAT_AN_LP_ABILITY_200GBASE_KR4CR4_R {
        STAT_AN_LP_ABILITY_200GBASE_KR4CR4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn stat_an_lp_ability_25gbase_kr1(&self) -> STAT_AN_LP_ABILITY_25GBASE_KR1_R {
        STAT_AN_LP_ABILITY_25GBASE_KR1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn stat_an_lp_ability_25gbase_cr1(&self) -> STAT_AN_LP_ABILITY_25GBASE_CR1_R {
        STAT_AN_LP_ABILITY_25GBASE_CR1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn stat_an_lp_ability_50gbase_kr2(&self) -> STAT_AN_LP_ABILITY_50GBASE_KR2_R {
        STAT_AN_LP_ABILITY_50GBASE_KR2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn stat_an_lp_ability_50gbase_cr2(&self) -> STAT_AN_LP_ABILITY_50GBASE_CR2_R {
        STAT_AN_LP_ABILITY_50GBASE_CR2_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Stat AN_ABILITY Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anasr](index.html) module"]
pub struct ANASR_SPEC;
impl crate::RegisterSpec for ANASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anasr::R](R) reader structure"]
impl crate::Readable for ANASR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets anasr to value 0"]
impl crate::Resettable for ANASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

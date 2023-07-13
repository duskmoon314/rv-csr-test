#[doc = "Register `anacr` reader"]
pub struct R(crate::R<ANACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `anacr` writer"]
pub struct W(crate::W<ANACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANACR_SPEC>;
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
impl From<crate::W<ANACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_an_ability_1000base_kx` reader - "]
pub type CTL_AN_ABILITY_1000BASE_KX_R = crate::BitReader<CTL_AN_ABILITY_1000BASE_KX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_1000BASE_KX_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_1000BASE_KX_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_1000BASE_KX_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_1000BASE_KX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_1000BASE_KX_A {
        match self.bits {
            false => CTL_AN_ABILITY_1000BASE_KX_A::DISABLE,
            true => CTL_AN_ABILITY_1000BASE_KX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_1000BASE_KX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_1000BASE_KX_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_1000base_kx` writer - "]
pub type CTL_AN_ABILITY_1000BASE_KX_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_1000BASE_KX_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_1000BASE_KX_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_1000BASE_KX_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_1000BASE_KX_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_10gbase_kx4` reader - "]
pub type CTL_AN_ABILITY_10GBASE_KX4_R = crate::BitReader<CTL_AN_ABILITY_10GBASE_KX4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_10GBASE_KX4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_10GBASE_KX4_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_10GBASE_KX4_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_10GBASE_KX4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_10GBASE_KX4_A {
        match self.bits {
            false => CTL_AN_ABILITY_10GBASE_KX4_A::DISABLE,
            true => CTL_AN_ABILITY_10GBASE_KX4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_10GBASE_KX4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_10GBASE_KX4_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_10gbase_kx4` writer - "]
pub type CTL_AN_ABILITY_10GBASE_KX4_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_10GBASE_KX4_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_10GBASE_KX4_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_10GBASE_KX4_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_10GBASE_KX4_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_10gbase_kr` reader - "]
pub type CTL_AN_ABILITY_10GBASE_KR_R = crate::BitReader<CTL_AN_ABILITY_10GBASE_KR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_10GBASE_KR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_10GBASE_KR_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_10GBASE_KR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_10GBASE_KR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_10GBASE_KR_A {
        match self.bits {
            false => CTL_AN_ABILITY_10GBASE_KR_A::DISABLE,
            true => CTL_AN_ABILITY_10GBASE_KR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_10GBASE_KR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_10GBASE_KR_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_10gbase_kr` writer - "]
pub type CTL_AN_ABILITY_10GBASE_KR_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_10GBASE_KR_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_10GBASE_KR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_10GBASE_KR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_10GBASE_KR_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_40gbase_kr4` reader - "]
pub type CTL_AN_ABILITY_40GBASE_KR4_R = crate::BitReader<CTL_AN_ABILITY_40GBASE_KR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_40GBASE_KR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_40GBASE_KR4_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_40GBASE_KR4_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_40GBASE_KR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_40GBASE_KR4_A {
        match self.bits {
            false => CTL_AN_ABILITY_40GBASE_KR4_A::DISABLE,
            true => CTL_AN_ABILITY_40GBASE_KR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_40GBASE_KR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_40GBASE_KR4_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_40gbase_kr4` writer - "]
pub type CTL_AN_ABILITY_40GBASE_KR4_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_40GBASE_KR4_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_40GBASE_KR4_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_40GBASE_KR4_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_40GBASE_KR4_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_40gbase_cr4` reader - "]
pub type CTL_AN_ABILITY_40GBASE_CR4_R = crate::BitReader<CTL_AN_ABILITY_40GBASE_CR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_40GBASE_CR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_40GBASE_CR4_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_40GBASE_CR4_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_40GBASE_CR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_40GBASE_CR4_A {
        match self.bits {
            false => CTL_AN_ABILITY_40GBASE_CR4_A::DISABLE,
            true => CTL_AN_ABILITY_40GBASE_CR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_40GBASE_CR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_40GBASE_CR4_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_40gbase_cr4` writer - "]
pub type CTL_AN_ABILITY_40GBASE_CR4_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_40GBASE_CR4_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_40GBASE_CR4_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_40GBASE_CR4_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_40GBASE_CR4_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_100gbase_cr10` reader - "]
pub type CTL_AN_ABILITY_100GBASE_CR10_R = crate::BitReader<CTL_AN_ABILITY_100GBASE_CR10_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_100GBASE_CR10_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_100GBASE_CR10_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_100GBASE_CR10_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_100GBASE_CR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_100GBASE_CR10_A {
        match self.bits {
            false => CTL_AN_ABILITY_100GBASE_CR10_A::DISABLE,
            true => CTL_AN_ABILITY_100GBASE_CR10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_CR10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_CR10_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_100gbase_cr10` writer - "]
pub type CTL_AN_ABILITY_100GBASE_CR10_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_100GBASE_CR10_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_100GBASE_CR10_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_CR10_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_CR10_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_100gbase_kp4` reader - "]
pub type CTL_AN_ABILITY_100GBASE_KP4_R = crate::BitReader<CTL_AN_ABILITY_100GBASE_KP4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_100GBASE_KP4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_100GBASE_KP4_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_100GBASE_KP4_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_100GBASE_KP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_100GBASE_KP4_A {
        match self.bits {
            false => CTL_AN_ABILITY_100GBASE_KP4_A::DISABLE,
            true => CTL_AN_ABILITY_100GBASE_KP4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_KP4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_KP4_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_100gbase_kp4` writer - "]
pub type CTL_AN_ABILITY_100GBASE_KP4_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_100GBASE_KP4_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_100GBASE_KP4_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_KP4_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_KP4_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_100gbase_kr4` reader - "]
pub type CTL_AN_ABILITY_100GBASE_KR4_R = crate::BitReader<CTL_AN_ABILITY_100GBASE_KR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_100GBASE_KR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_100GBASE_KR4_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_100GBASE_KR4_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_100GBASE_KR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_100GBASE_KR4_A {
        match self.bits {
            false => CTL_AN_ABILITY_100GBASE_KR4_A::DISABLE,
            true => CTL_AN_ABILITY_100GBASE_KR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_KR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_KR4_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_100gbase_kr4` writer - "]
pub type CTL_AN_ABILITY_100GBASE_KR4_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_100GBASE_KR4_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_100GBASE_KR4_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_KR4_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_KR4_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_100gbase_cr4` reader - "]
pub type CTL_AN_ABILITY_100GBASE_CR4_R = crate::BitReader<CTL_AN_ABILITY_100GBASE_CR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_100GBASE_CR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_100GBASE_CR4_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_100GBASE_CR4_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_100GBASE_CR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_100GBASE_CR4_A {
        match self.bits {
            false => CTL_AN_ABILITY_100GBASE_CR4_A::DISABLE,
            true => CTL_AN_ABILITY_100GBASE_CR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_CR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_CR4_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_100gbase_cr4` writer - "]
pub type CTL_AN_ABILITY_100GBASE_CR4_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_100GBASE_CR4_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_100GBASE_CR4_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_CR4_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_CR4_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_25gbase_krcr_s` reader - "]
pub type CTL_AN_ABILITY_25GBASE_KRCR_S_R = crate::BitReader<CTL_AN_ABILITY_25GBASE_KRCR_S_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_25GBASE_KRCR_S_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_25GBASE_KRCR_S_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_25GBASE_KRCR_S_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_25GBASE_KRCR_S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_25GBASE_KRCR_S_A {
        match self.bits {
            false => CTL_AN_ABILITY_25GBASE_KRCR_S_A::DISABLE,
            true => CTL_AN_ABILITY_25GBASE_KRCR_S_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_25GBASE_KRCR_S_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_25GBASE_KRCR_S_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_25gbase_krcr_s` writer - "]
pub type CTL_AN_ABILITY_25GBASE_KRCR_S_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_25GBASE_KRCR_S_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_25GBASE_KRCR_S_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_25GBASE_KRCR_S_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_25GBASE_KRCR_S_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_25gbase_krcr` reader - "]
pub type CTL_AN_ABILITY_25GBASE_KRCR_R = crate::BitReader<CTL_AN_ABILITY_25GBASE_KRCR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_25GBASE_KRCR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_25GBASE_KRCR_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_25GBASE_KRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_25GBASE_KRCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_25GBASE_KRCR_A {
        match self.bits {
            false => CTL_AN_ABILITY_25GBASE_KRCR_A::DISABLE,
            true => CTL_AN_ABILITY_25GBASE_KRCR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_25GBASE_KRCR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_25GBASE_KRCR_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_25gbase_krcr` writer - "]
pub type CTL_AN_ABILITY_25GBASE_KRCR_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_25GBASE_KRCR_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_25GBASE_KRCR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_25GBASE_KRCR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_25GBASE_KRCR_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_2_5gbase_kx` reader - "]
pub type CTL_AN_ABILITY_2_5GBASE_KX_R = crate::BitReader<CTL_AN_ABILITY_2_5GBASE_KX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_2_5GBASE_KX_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_2_5GBASE_KX_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_2_5GBASE_KX_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_2_5GBASE_KX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_2_5GBASE_KX_A {
        match self.bits {
            false => CTL_AN_ABILITY_2_5GBASE_KX_A::DISABLE,
            true => CTL_AN_ABILITY_2_5GBASE_KX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_2_5GBASE_KX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_2_5GBASE_KX_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_2_5gbase_kx` writer - "]
pub type CTL_AN_ABILITY_2_5GBASE_KX_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_2_5GBASE_KX_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_2_5GBASE_KX_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_2_5GBASE_KX_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_2_5GBASE_KX_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_5gbase_kr` reader - "]
pub type CTL_AN_ABILITY_5GBASE_KR_R = crate::BitReader<CTL_AN_ABILITY_5GBASE_KR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_5GBASE_KR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_5GBASE_KR_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_5GBASE_KR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_5GBASE_KR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_5GBASE_KR_A {
        match self.bits {
            false => CTL_AN_ABILITY_5GBASE_KR_A::DISABLE,
            true => CTL_AN_ABILITY_5GBASE_KR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_5GBASE_KR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_5GBASE_KR_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_5gbase_kr` writer - "]
pub type CTL_AN_ABILITY_5GBASE_KR_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_5GBASE_KR_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_5GBASE_KR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_5GBASE_KR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_5GBASE_KR_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_50gbase_krcr` reader - "]
pub type CTL_AN_ABILITY_50GBASE_KRCR_R = crate::BitReader<CTL_AN_ABILITY_50GBASE_KRCR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_50GBASE_KRCR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_50GBASE_KRCR_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_50GBASE_KRCR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_50GBASE_KRCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_50GBASE_KRCR_A {
        match self.bits {
            false => CTL_AN_ABILITY_50GBASE_KRCR_A::DISABLE,
            true => CTL_AN_ABILITY_50GBASE_KRCR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_50GBASE_KRCR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_50GBASE_KRCR_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_50gbase_krcr` writer - "]
pub type CTL_AN_ABILITY_50GBASE_KRCR_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_50GBASE_KRCR_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_50GBASE_KRCR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_50GBASE_KRCR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_50GBASE_KRCR_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_100gbase_kr2cr2` reader - "]
pub type CTL_AN_ABILITY_100GBASE_KR2CR2_R = crate::BitReader<CTL_AN_ABILITY_100GBASE_KR2CR2_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_100GBASE_KR2CR2_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_100GBASE_KR2CR2_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_100GBASE_KR2CR2_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_100GBASE_KR2CR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_100GBASE_KR2CR2_A {
        match self.bits {
            false => CTL_AN_ABILITY_100GBASE_KR2CR2_A::DISABLE,
            true => CTL_AN_ABILITY_100GBASE_KR2CR2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_KR2CR2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_100GBASE_KR2CR2_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_100gbase_kr2cr2` writer - "]
pub type CTL_AN_ABILITY_100GBASE_KR2CR2_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_100GBASE_KR2CR2_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_100GBASE_KR2CR2_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_KR2CR2_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_100GBASE_KR2CR2_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_200gbase_kr4cr4` reader - "]
pub type CTL_AN_ABILITY_200GBASE_KR4CR4_R = crate::BitReader<CTL_AN_ABILITY_200GBASE_KR4CR4_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_200GBASE_KR4CR4_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_200GBASE_KR4CR4_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_200GBASE_KR4CR4_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_200GBASE_KR4CR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_200GBASE_KR4CR4_A {
        match self.bits {
            false => CTL_AN_ABILITY_200GBASE_KR4CR4_A::DISABLE,
            true => CTL_AN_ABILITY_200GBASE_KR4CR4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_200GBASE_KR4CR4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_200GBASE_KR4CR4_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_200gbase_kr4cr4` writer - "]
pub type CTL_AN_ABILITY_200GBASE_KR4CR4_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_200GBASE_KR4CR4_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_200GBASE_KR4CR4_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_200GBASE_KR4CR4_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_200GBASE_KR4CR4_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_25gbase_kr1` reader - "]
pub type CTL_AN_ABILITY_25GBASE_KR1_R = crate::BitReader<CTL_AN_ABILITY_25GBASE_KR1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_25GBASE_KR1_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_25GBASE_KR1_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_25GBASE_KR1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_25GBASE_KR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_25GBASE_KR1_A {
        match self.bits {
            false => CTL_AN_ABILITY_25GBASE_KR1_A::DISABLE,
            true => CTL_AN_ABILITY_25GBASE_KR1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_25GBASE_KR1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_25GBASE_KR1_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_25gbase_kr1` writer - "]
pub type CTL_AN_ABILITY_25GBASE_KR1_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_25GBASE_KR1_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_25GBASE_KR1_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_25GBASE_KR1_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_25GBASE_KR1_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_25gbase_cr1` reader - "]
pub type CTL_AN_ABILITY_25GBASE_CR1_R = crate::BitReader<CTL_AN_ABILITY_25GBASE_CR1_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_25GBASE_CR1_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_25GBASE_CR1_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_25GBASE_CR1_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_25GBASE_CR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_25GBASE_CR1_A {
        match self.bits {
            false => CTL_AN_ABILITY_25GBASE_CR1_A::DISABLE,
            true => CTL_AN_ABILITY_25GBASE_CR1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_25GBASE_CR1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_25GBASE_CR1_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_25gbase_cr1` writer - "]
pub type CTL_AN_ABILITY_25GBASE_CR1_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_25GBASE_CR1_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_25GBASE_CR1_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_25GBASE_CR1_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_25GBASE_CR1_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_50gbase_kr2` reader - "]
pub type CTL_AN_ABILITY_50GBASE_KR2_R = crate::BitReader<CTL_AN_ABILITY_50GBASE_KR2_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_50GBASE_KR2_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_50GBASE_KR2_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_50GBASE_KR2_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_50GBASE_KR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_50GBASE_KR2_A {
        match self.bits {
            false => CTL_AN_ABILITY_50GBASE_KR2_A::DISABLE,
            true => CTL_AN_ABILITY_50GBASE_KR2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_50GBASE_KR2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_50GBASE_KR2_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_50gbase_kr2` writer - "]
pub type CTL_AN_ABILITY_50GBASE_KR2_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_50GBASE_KR2_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_50GBASE_KR2_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_50GBASE_KR2_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_50GBASE_KR2_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_ability_50gbase_cr2` reader - "]
pub type CTL_AN_ABILITY_50GBASE_CR2_R = crate::BitReader<CTL_AN_ABILITY_50GBASE_CR2_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ABILITY_50GBASE_CR2_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ABILITY_50GBASE_CR2_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ABILITY_50GBASE_CR2_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ABILITY_50GBASE_CR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ABILITY_50GBASE_CR2_A {
        match self.bits {
            false => CTL_AN_ABILITY_50GBASE_CR2_A::DISABLE,
            true => CTL_AN_ABILITY_50GBASE_CR2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ABILITY_50GBASE_CR2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ABILITY_50GBASE_CR2_A::ENABLE
    }
}
#[doc = "Field `ctl_an_ability_50gbase_cr2` writer - "]
pub type CTL_AN_ABILITY_50GBASE_CR2_W<'a, const O: u8> =
    crate::BitWriter<'a, ANACR_SPEC, O, CTL_AN_ABILITY_50GBASE_CR2_A>;
impl<'a, const O: u8> CTL_AN_ABILITY_50GBASE_CR2_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_50GBASE_CR2_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ABILITY_50GBASE_CR2_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_an_ability_1000base_kx(&self) -> CTL_AN_ABILITY_1000BASE_KX_R {
        CTL_AN_ABILITY_1000BASE_KX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctl_an_ability_10gbase_kx4(&self) -> CTL_AN_ABILITY_10GBASE_KX4_R {
        CTL_AN_ABILITY_10GBASE_KX4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ctl_an_ability_10gbase_kr(&self) -> CTL_AN_ABILITY_10GBASE_KR_R {
        CTL_AN_ABILITY_10GBASE_KR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ctl_an_ability_40gbase_kr4(&self) -> CTL_AN_ABILITY_40GBASE_KR4_R {
        CTL_AN_ABILITY_40GBASE_KR4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ctl_an_ability_40gbase_cr4(&self) -> CTL_AN_ABILITY_40GBASE_CR4_R {
        CTL_AN_ABILITY_40GBASE_CR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ctl_an_ability_100gbase_cr10(&self) -> CTL_AN_ABILITY_100GBASE_CR10_R {
        CTL_AN_ABILITY_100GBASE_CR10_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ctl_an_ability_100gbase_kp4(&self) -> CTL_AN_ABILITY_100GBASE_KP4_R {
        CTL_AN_ABILITY_100GBASE_KP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ctl_an_ability_100gbase_kr4(&self) -> CTL_AN_ABILITY_100GBASE_KR4_R {
        CTL_AN_ABILITY_100GBASE_KR4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ctl_an_ability_100gbase_cr4(&self) -> CTL_AN_ABILITY_100GBASE_CR4_R {
        CTL_AN_ABILITY_100GBASE_CR4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ctl_an_ability_25gbase_krcr_s(&self) -> CTL_AN_ABILITY_25GBASE_KRCR_S_R {
        CTL_AN_ABILITY_25GBASE_KRCR_S_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ctl_an_ability_25gbase_krcr(&self) -> CTL_AN_ABILITY_25GBASE_KRCR_R {
        CTL_AN_ABILITY_25GBASE_KRCR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ctl_an_ability_2_5gbase_kx(&self) -> CTL_AN_ABILITY_2_5GBASE_KX_R {
        CTL_AN_ABILITY_2_5GBASE_KX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ctl_an_ability_5gbase_kr(&self) -> CTL_AN_ABILITY_5GBASE_KR_R {
        CTL_AN_ABILITY_5GBASE_KR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ctl_an_ability_50gbase_krcr(&self) -> CTL_AN_ABILITY_50GBASE_KRCR_R {
        CTL_AN_ABILITY_50GBASE_KRCR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ctl_an_ability_100gbase_kr2cr2(&self) -> CTL_AN_ABILITY_100GBASE_KR2CR2_R {
        CTL_AN_ABILITY_100GBASE_KR2CR2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ctl_an_ability_200gbase_kr4cr4(&self) -> CTL_AN_ABILITY_200GBASE_KR4CR4_R {
        CTL_AN_ABILITY_200GBASE_KR4CR4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ctl_an_ability_25gbase_kr1(&self) -> CTL_AN_ABILITY_25GBASE_KR1_R {
        CTL_AN_ABILITY_25GBASE_KR1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ctl_an_ability_25gbase_cr1(&self) -> CTL_AN_ABILITY_25GBASE_CR1_R {
        CTL_AN_ABILITY_25GBASE_CR1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ctl_an_ability_50gbase_kr2(&self) -> CTL_AN_ABILITY_50GBASE_KR2_R {
        CTL_AN_ABILITY_50GBASE_KR2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ctl_an_ability_50gbase_cr2(&self) -> CTL_AN_ABILITY_50GBASE_CR2_R {
        CTL_AN_ABILITY_50GBASE_CR2_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_1000base_kx(&mut self) -> CTL_AN_ABILITY_1000BASE_KX_W<0> {
        CTL_AN_ABILITY_1000BASE_KX_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_10gbase_kx4(&mut self) -> CTL_AN_ABILITY_10GBASE_KX4_W<1> {
        CTL_AN_ABILITY_10GBASE_KX4_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_10gbase_kr(&mut self) -> CTL_AN_ABILITY_10GBASE_KR_W<2> {
        CTL_AN_ABILITY_10GBASE_KR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_40gbase_kr4(&mut self) -> CTL_AN_ABILITY_40GBASE_KR4_W<3> {
        CTL_AN_ABILITY_40GBASE_KR4_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_40gbase_cr4(&mut self) -> CTL_AN_ABILITY_40GBASE_CR4_W<4> {
        CTL_AN_ABILITY_40GBASE_CR4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_100gbase_cr10(&mut self) -> CTL_AN_ABILITY_100GBASE_CR10_W<5> {
        CTL_AN_ABILITY_100GBASE_CR10_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_100gbase_kp4(&mut self) -> CTL_AN_ABILITY_100GBASE_KP4_W<6> {
        CTL_AN_ABILITY_100GBASE_KP4_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_100gbase_kr4(&mut self) -> CTL_AN_ABILITY_100GBASE_KR4_W<7> {
        CTL_AN_ABILITY_100GBASE_KR4_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_100gbase_cr4(&mut self) -> CTL_AN_ABILITY_100GBASE_CR4_W<8> {
        CTL_AN_ABILITY_100GBASE_CR4_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_25gbase_krcr_s(&mut self) -> CTL_AN_ABILITY_25GBASE_KRCR_S_W<9> {
        CTL_AN_ABILITY_25GBASE_KRCR_S_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_25gbase_krcr(&mut self) -> CTL_AN_ABILITY_25GBASE_KRCR_W<10> {
        CTL_AN_ABILITY_25GBASE_KRCR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_2_5gbase_kx(&mut self) -> CTL_AN_ABILITY_2_5GBASE_KX_W<11> {
        CTL_AN_ABILITY_2_5GBASE_KX_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_5gbase_kr(&mut self) -> CTL_AN_ABILITY_5GBASE_KR_W<12> {
        CTL_AN_ABILITY_5GBASE_KR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_50gbase_krcr(&mut self) -> CTL_AN_ABILITY_50GBASE_KRCR_W<13> {
        CTL_AN_ABILITY_50GBASE_KRCR_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_100gbase_kr2cr2(&mut self) -> CTL_AN_ABILITY_100GBASE_KR2CR2_W<14> {
        CTL_AN_ABILITY_100GBASE_KR2CR2_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_200gbase_kr4cr4(&mut self) -> CTL_AN_ABILITY_200GBASE_KR4CR4_W<15> {
        CTL_AN_ABILITY_200GBASE_KR4CR4_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_25gbase_kr1(&mut self) -> CTL_AN_ABILITY_25GBASE_KR1_W<16> {
        CTL_AN_ABILITY_25GBASE_KR1_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_25gbase_cr1(&mut self) -> CTL_AN_ABILITY_25GBASE_CR1_W<17> {
        CTL_AN_ABILITY_25GBASE_CR1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_50gbase_kr2(&mut self) -> CTL_AN_ABILITY_50GBASE_KR2_W<18> {
        CTL_AN_ABILITY_50GBASE_KR2_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_ability_50gbase_cr2(&mut self) -> CTL_AN_ABILITY_50GBASE_CR2_W<19> {
        CTL_AN_ABILITY_50GBASE_CR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AN_ABILITY Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anacr](index.html) module"]
pub struct ANACR_SPEC;
impl crate::RegisterSpec for ANACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anacr::R](R) reader structure"]
impl crate::Readable for ANACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anacr::W](W) writer structure"]
impl crate::Writable for ANACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets anacr to value 0"]
impl crate::Resettable for ANACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

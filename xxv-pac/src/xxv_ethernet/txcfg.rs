#[doc = "Register `txcfg` reader"]
pub struct R(crate::R<TXCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `txcfg` writer"]
pub struct W(crate::W<TXCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCFG_SPEC>;
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
impl From<crate::W<TXCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_tx_enable` reader - "]
pub type CTL_TX_ENABLE_R = crate::BitReader<CTL_TX_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_ENABLE_A {
        match self.bits {
            false => CTL_TX_ENABLE_A::DISABLE,
            true => CTL_TX_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_enable` writer - "]
pub type CTL_TX_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_ENABLE_A>;
impl<'a, const O: u8> CTL_TX_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_fcs_ins_enable` reader - "]
pub type CTL_TX_FCS_INS_ENABLE_R = crate::BitReader<CTL_TX_FCS_INS_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_FCS_INS_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_FCS_INS_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_FCS_INS_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_FCS_INS_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_FCS_INS_ENABLE_A {
        match self.bits {
            false => CTL_TX_FCS_INS_ENABLE_A::DISABLE,
            true => CTL_TX_FCS_INS_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_FCS_INS_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_FCS_INS_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_fcs_ins_enable` writer - "]
pub type CTL_TX_FCS_INS_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_FCS_INS_ENABLE_A>;
impl<'a, const O: u8> CTL_TX_FCS_INS_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_FCS_INS_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_FCS_INS_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_ignore_fcs` reader - "]
pub type CTL_TX_IGNORE_FCS_R = crate::BitReader<CTL_TX_IGNORE_FCS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_IGNORE_FCS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_IGNORE_FCS_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_IGNORE_FCS_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_IGNORE_FCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_IGNORE_FCS_A {
        match self.bits {
            false => CTL_TX_IGNORE_FCS_A::DISABLE,
            true => CTL_TX_IGNORE_FCS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_IGNORE_FCS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_IGNORE_FCS_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_ignore_fcs` writer - "]
pub type CTL_TX_IGNORE_FCS_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_IGNORE_FCS_A>;
impl<'a, const O: u8> CTL_TX_IGNORE_FCS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_IGNORE_FCS_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_IGNORE_FCS_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_send_lfi` reader - "]
pub type CTL_TX_SEND_LFI_R = crate::BitReader<CTL_TX_SEND_LFI_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_SEND_LFI_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_SEND_LFI_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_SEND_LFI_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_SEND_LFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_SEND_LFI_A {
        match self.bits {
            false => CTL_TX_SEND_LFI_A::DISABLE,
            true => CTL_TX_SEND_LFI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_SEND_LFI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_SEND_LFI_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_send_lfi` writer - "]
pub type CTL_TX_SEND_LFI_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_SEND_LFI_A>;
impl<'a, const O: u8> CTL_TX_SEND_LFI_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_SEND_LFI_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_SEND_LFI_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_send_rfi` reader - "]
pub type CTL_TX_SEND_RFI_R = crate::BitReader<CTL_TX_SEND_RFI_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_SEND_RFI_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_SEND_RFI_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_SEND_RFI_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_SEND_RFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_SEND_RFI_A {
        match self.bits {
            false => CTL_TX_SEND_RFI_A::DISABLE,
            true => CTL_TX_SEND_RFI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_SEND_RFI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_SEND_RFI_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_send_rfi` writer - "]
pub type CTL_TX_SEND_RFI_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_SEND_RFI_A>;
impl<'a, const O: u8> CTL_TX_SEND_RFI_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_SEND_RFI_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_SEND_RFI_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_send_idle` reader - "]
pub type CTL_TX_SEND_IDLE_R = crate::BitReader<CTL_TX_SEND_IDLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_SEND_IDLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_SEND_IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_SEND_IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_SEND_IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_SEND_IDLE_A {
        match self.bits {
            false => CTL_TX_SEND_IDLE_A::DISABLE,
            true => CTL_TX_SEND_IDLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_SEND_IDLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_SEND_IDLE_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_send_idle` writer - "]
pub type CTL_TX_SEND_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_SEND_IDLE_A>;
impl<'a, const O: u8> CTL_TX_SEND_IDLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_SEND_IDLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_SEND_IDLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_ipg_value` reader - "]
pub type CTL_TX_IPG_VALUE_R = crate::FieldReader;
#[doc = "Field `ctl_tx_ipg_value` writer - "]
pub type CTL_TX_IPG_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, TXCFG_SPEC, 4, O>;
#[doc = "Field `ctl_tx_test_pattern` reader - "]
pub type CTL_TX_TEST_PATTERN_R = crate::BitReader<CTL_TX_TEST_PATTERN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_TEST_PATTERN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_TEST_PATTERN_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_TEST_PATTERN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_TEST_PATTERN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_TEST_PATTERN_A {
        match self.bits {
            false => CTL_TX_TEST_PATTERN_A::DISABLE,
            true => CTL_TX_TEST_PATTERN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_TEST_PATTERN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_TEST_PATTERN_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_test_pattern` writer - "]
pub type CTL_TX_TEST_PATTERN_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_TEST_PATTERN_A>;
impl<'a, const O: u8> CTL_TX_TEST_PATTERN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_TEST_PATTERN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_TEST_PATTERN_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_test_pattern_enable` reader - "]
pub type CTL_TX_TEST_PATTERN_ENABLE_R = crate::BitReader<CTL_TX_TEST_PATTERN_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_TEST_PATTERN_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_TEST_PATTERN_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_TEST_PATTERN_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_TEST_PATTERN_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_TEST_PATTERN_ENABLE_A {
        match self.bits {
            false => CTL_TX_TEST_PATTERN_ENABLE_A::DISABLE,
            true => CTL_TX_TEST_PATTERN_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_TEST_PATTERN_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_TEST_PATTERN_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_test_pattern_enable` writer - "]
pub type CTL_TX_TEST_PATTERN_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_TEST_PATTERN_ENABLE_A>;
impl<'a, const O: u8> CTL_TX_TEST_PATTERN_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_TEST_PATTERN_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_TEST_PATTERN_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_test_pattern_select` reader - "]
pub type CTL_TX_TEST_PATTERN_SELECT_R = crate::BitReader<CTL_TX_TEST_PATTERN_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_TEST_PATTERN_SELECT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_TEST_PATTERN_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_TEST_PATTERN_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_TEST_PATTERN_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_TEST_PATTERN_SELECT_A {
        match self.bits {
            false => CTL_TX_TEST_PATTERN_SELECT_A::DISABLE,
            true => CTL_TX_TEST_PATTERN_SELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_TEST_PATTERN_SELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_TEST_PATTERN_SELECT_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_test_pattern_select` writer - "]
pub type CTL_TX_TEST_PATTERN_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_TEST_PATTERN_SELECT_A>;
impl<'a, const O: u8> CTL_TX_TEST_PATTERN_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_TEST_PATTERN_SELECT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_TEST_PATTERN_SELECT_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_data_pattern_select` reader - "]
pub type CTL_TX_DATA_PATTERN_SELECT_R = crate::BitReader<CTL_TX_DATA_PATTERN_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_DATA_PATTERN_SELECT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_DATA_PATTERN_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_DATA_PATTERN_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_DATA_PATTERN_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_DATA_PATTERN_SELECT_A {
        match self.bits {
            false => CTL_TX_DATA_PATTERN_SELECT_A::DISABLE,
            true => CTL_TX_DATA_PATTERN_SELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_DATA_PATTERN_SELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_DATA_PATTERN_SELECT_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_data_pattern_select` writer - "]
pub type CTL_TX_DATA_PATTERN_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_DATA_PATTERN_SELECT_A>;
impl<'a, const O: u8> CTL_TX_DATA_PATTERN_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_DATA_PATTERN_SELECT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_DATA_PATTERN_SELECT_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_custom_preamble_enable` reader - "]
pub type CTL_TX_CUSTOM_PREAMBLE_ENABLE_R = crate::BitReader<CTL_TX_CUSTOM_PREAMBLE_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_CUSTOM_PREAMBLE_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_CUSTOM_PREAMBLE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_CUSTOM_PREAMBLE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_CUSTOM_PREAMBLE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_CUSTOM_PREAMBLE_ENABLE_A {
        match self.bits {
            false => CTL_TX_CUSTOM_PREAMBLE_ENABLE_A::DISABLE,
            true => CTL_TX_CUSTOM_PREAMBLE_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_CUSTOM_PREAMBLE_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_CUSTOM_PREAMBLE_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_custom_preamble_enable` writer - "]
pub type CTL_TX_CUSTOM_PREAMBLE_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_CUSTOM_PREAMBLE_ENABLE_A>;
impl<'a, const O: u8> CTL_TX_CUSTOM_PREAMBLE_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_CUSTOM_PREAMBLE_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_CUSTOM_PREAMBLE_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_tx_prbs31_test_pattern_enable` reader - "]
pub type CTL_TX_PRBS31_TEST_PATTERN_ENABLE_R =
    crate::BitReader<CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_TX_PRBS31_TEST_PATTERN_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A {
        match self.bits {
            false => CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A::DISABLE,
            true => CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_tx_prbs31_test_pattern_enable` writer - "]
pub type CTL_TX_PRBS31_TEST_PATTERN_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, TXCFG_SPEC, O, CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A>;
impl<'a, const O: u8> CTL_TX_PRBS31_TEST_PATTERN_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_TX_PRBS31_TEST_PATTERN_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_tx_enable(&self) -> CTL_TX_ENABLE_R {
        CTL_TX_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctl_tx_fcs_ins_enable(&self) -> CTL_TX_FCS_INS_ENABLE_R {
        CTL_TX_FCS_INS_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ctl_tx_ignore_fcs(&self) -> CTL_TX_IGNORE_FCS_R {
        CTL_TX_IGNORE_FCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ctl_tx_send_lfi(&self) -> CTL_TX_SEND_LFI_R {
        CTL_TX_SEND_LFI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ctl_tx_send_rfi(&self) -> CTL_TX_SEND_RFI_R {
        CTL_TX_SEND_RFI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ctl_tx_send_idle(&self) -> CTL_TX_SEND_IDLE_R {
        CTL_TX_SEND_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn ctl_tx_ipg_value(&self) -> CTL_TX_IPG_VALUE_R {
        CTL_TX_IPG_VALUE_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ctl_tx_test_pattern(&self) -> CTL_TX_TEST_PATTERN_R {
        CTL_TX_TEST_PATTERN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ctl_tx_test_pattern_enable(&self) -> CTL_TX_TEST_PATTERN_ENABLE_R {
        CTL_TX_TEST_PATTERN_ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ctl_tx_test_pattern_select(&self) -> CTL_TX_TEST_PATTERN_SELECT_R {
        CTL_TX_TEST_PATTERN_SELECT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ctl_tx_data_pattern_select(&self) -> CTL_TX_DATA_PATTERN_SELECT_R {
        CTL_TX_DATA_PATTERN_SELECT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ctl_tx_custom_preamble_enable(&self) -> CTL_TX_CUSTOM_PREAMBLE_ENABLE_R {
        CTL_TX_CUSTOM_PREAMBLE_ENABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ctl_tx_prbs31_test_pattern_enable(&self) -> CTL_TX_PRBS31_TEST_PATTERN_ENABLE_R {
        CTL_TX_PRBS31_TEST_PATTERN_ENABLE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_enable(&mut self) -> CTL_TX_ENABLE_W<0> {
        CTL_TX_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_fcs_ins_enable(&mut self) -> CTL_TX_FCS_INS_ENABLE_W<1> {
        CTL_TX_FCS_INS_ENABLE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_ignore_fcs(&mut self) -> CTL_TX_IGNORE_FCS_W<2> {
        CTL_TX_IGNORE_FCS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_send_lfi(&mut self) -> CTL_TX_SEND_LFI_W<3> {
        CTL_TX_SEND_LFI_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_send_rfi(&mut self) -> CTL_TX_SEND_RFI_W<4> {
        CTL_TX_SEND_RFI_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_send_idle(&mut self) -> CTL_TX_SEND_IDLE_W<5> {
        CTL_TX_SEND_IDLE_W::new(self)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_ipg_value(&mut self) -> CTL_TX_IPG_VALUE_W<10> {
        CTL_TX_IPG_VALUE_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_test_pattern(&mut self) -> CTL_TX_TEST_PATTERN_W<14> {
        CTL_TX_TEST_PATTERN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_test_pattern_enable(&mut self) -> CTL_TX_TEST_PATTERN_ENABLE_W<15> {
        CTL_TX_TEST_PATTERN_ENABLE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_test_pattern_select(&mut self) -> CTL_TX_TEST_PATTERN_SELECT_W<16> {
        CTL_TX_TEST_PATTERN_SELECT_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_data_pattern_select(&mut self) -> CTL_TX_DATA_PATTERN_SELECT_W<17> {
        CTL_TX_DATA_PATTERN_SELECT_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_custom_preamble_enable(&mut self) -> CTL_TX_CUSTOM_PREAMBLE_ENABLE_W<18> {
        CTL_TX_CUSTOM_PREAMBLE_ENABLE_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_prbs31_test_pattern_enable(&mut self) -> CTL_TX_PRBS31_TEST_PATTERN_ENABLE_W<23> {
        CTL_TX_PRBS31_TEST_PATTERN_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcfg](index.html) module"]
pub struct TXCFG_SPEC;
impl crate::RegisterSpec for TXCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txcfg::R](R) reader structure"]
impl crate::Readable for TXCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txcfg::W](W) writer structure"]
impl crate::Writable for TXCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets txcfg to value 0"]
impl crate::Resettable for TXCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

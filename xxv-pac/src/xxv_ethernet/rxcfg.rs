#[doc = "Register `rxcfg` reader"]
pub struct R(crate::R<RXCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxcfg` writer"]
pub struct W(crate::W<RXCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXCFG_SPEC>;
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
impl From<crate::W<RXCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_rx_enable` reader - "]
pub type CTL_RX_ENABLE_R = crate::BitReader<CTL_RX_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_ENABLE_A {
        match self.bits {
            false => CTL_RX_ENABLE_A::DISABLE,
            true => CTL_RX_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_enable` writer - "]
pub type CTL_RX_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_ENABLE_A>;
impl<'a, const O: u8> CTL_RX_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_delete_fcs` reader - "]
pub type CTL_RX_DELETE_FCS_R = crate::BitReader<CTL_RX_DELETE_FCS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_DELETE_FCS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_DELETE_FCS_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_DELETE_FCS_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_DELETE_FCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_DELETE_FCS_A {
        match self.bits {
            false => CTL_RX_DELETE_FCS_A::DISABLE,
            true => CTL_RX_DELETE_FCS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_DELETE_FCS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_DELETE_FCS_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_delete_fcs` writer - "]
pub type CTL_RX_DELETE_FCS_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_DELETE_FCS_A>;
impl<'a, const O: u8> CTL_RX_DELETE_FCS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_DELETE_FCS_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_DELETE_FCS_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_ignore_fcs` reader - "]
pub type CTL_RX_IGNORE_FCS_R = crate::BitReader<CTL_RX_IGNORE_FCS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_IGNORE_FCS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_IGNORE_FCS_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_IGNORE_FCS_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_IGNORE_FCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_IGNORE_FCS_A {
        match self.bits {
            false => CTL_RX_IGNORE_FCS_A::DISABLE,
            true => CTL_RX_IGNORE_FCS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_IGNORE_FCS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_IGNORE_FCS_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_ignore_fcs` writer - "]
pub type CTL_RX_IGNORE_FCS_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_IGNORE_FCS_A>;
impl<'a, const O: u8> CTL_RX_IGNORE_FCS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_IGNORE_FCS_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_IGNORE_FCS_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_process_lfi` reader - "]
pub type CTL_RX_PROCESS_LFI_R = crate::BitReader<CTL_RX_PROCESS_LFI_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_PROCESS_LFI_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_PROCESS_LFI_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_PROCESS_LFI_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_PROCESS_LFI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_PROCESS_LFI_A {
        match self.bits {
            false => CTL_RX_PROCESS_LFI_A::DISABLE,
            true => CTL_RX_PROCESS_LFI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_PROCESS_LFI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_PROCESS_LFI_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_process_lfi` writer - "]
pub type CTL_RX_PROCESS_LFI_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_PROCESS_LFI_A>;
impl<'a, const O: u8> CTL_RX_PROCESS_LFI_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_PROCESS_LFI_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_PROCESS_LFI_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_check_sfd` reader - "]
pub type CTL_RX_CHECK_SFD_R = crate::BitReader<CTL_RX_CHECK_SFD_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_CHECK_SFD_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_CHECK_SFD_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_CHECK_SFD_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_CHECK_SFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_CHECK_SFD_A {
        match self.bits {
            false => CTL_RX_CHECK_SFD_A::DISABLE,
            true => CTL_RX_CHECK_SFD_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_CHECK_SFD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_CHECK_SFD_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_check_sfd` writer - "]
pub type CTL_RX_CHECK_SFD_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_CHECK_SFD_A>;
impl<'a, const O: u8> CTL_RX_CHECK_SFD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_CHECK_SFD_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_CHECK_SFD_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_check_preamble` reader - "]
pub type CTL_RX_CHECK_PREAMBLE_R = crate::BitReader<CTL_RX_CHECK_PREAMBLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_CHECK_PREAMBLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_CHECK_PREAMBLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_CHECK_PREAMBLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_CHECK_PREAMBLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_CHECK_PREAMBLE_A {
        match self.bits {
            false => CTL_RX_CHECK_PREAMBLE_A::DISABLE,
            true => CTL_RX_CHECK_PREAMBLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_CHECK_PREAMBLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_CHECK_PREAMBLE_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_check_preamble` writer - "]
pub type CTL_RX_CHECK_PREAMBLE_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_CHECK_PREAMBLE_A>;
impl<'a, const O: u8> CTL_RX_CHECK_PREAMBLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_CHECK_PREAMBLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_CHECK_PREAMBLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_force_resync` reader - "]
pub type CTL_RX_FORCE_RESYNC_R = crate::BitReader<CTL_RX_FORCE_RESYNC_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_FORCE_RESYNC_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_FORCE_RESYNC_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_FORCE_RESYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_FORCE_RESYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_FORCE_RESYNC_A {
        match self.bits {
            false => CTL_RX_FORCE_RESYNC_A::DISABLE,
            true => CTL_RX_FORCE_RESYNC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_FORCE_RESYNC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_FORCE_RESYNC_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_force_resync` writer - "]
pub type CTL_RX_FORCE_RESYNC_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_FORCE_RESYNC_A>;
impl<'a, const O: u8> CTL_RX_FORCE_RESYNC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_FORCE_RESYNC_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_FORCE_RESYNC_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_test_pattern` reader - "]
pub type CTL_RX_TEST_PATTERN_R = crate::BitReader<CTL_RX_TEST_PATTERN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_TEST_PATTERN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_TEST_PATTERN_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_TEST_PATTERN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_TEST_PATTERN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_TEST_PATTERN_A {
        match self.bits {
            false => CTL_RX_TEST_PATTERN_A::DISABLE,
            true => CTL_RX_TEST_PATTERN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_TEST_PATTERN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_TEST_PATTERN_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_test_pattern` writer - "]
pub type CTL_RX_TEST_PATTERN_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_TEST_PATTERN_A>;
impl<'a, const O: u8> CTL_RX_TEST_PATTERN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_TEST_PATTERN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_TEST_PATTERN_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_test_pattern_enable` reader - "]
pub type CTL_RX_TEST_PATTERN_ENABLE_R = crate::BitReader<CTL_RX_TEST_PATTERN_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_TEST_PATTERN_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_TEST_PATTERN_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_TEST_PATTERN_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_TEST_PATTERN_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_TEST_PATTERN_ENABLE_A {
        match self.bits {
            false => CTL_RX_TEST_PATTERN_ENABLE_A::DISABLE,
            true => CTL_RX_TEST_PATTERN_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_TEST_PATTERN_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_TEST_PATTERN_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_test_pattern_enable` writer - "]
pub type CTL_RX_TEST_PATTERN_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_TEST_PATTERN_ENABLE_A>;
impl<'a, const O: u8> CTL_RX_TEST_PATTERN_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_TEST_PATTERN_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_TEST_PATTERN_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_data_pattern_select` reader - "]
pub type CTL_RX_DATA_PATTERN_SELECT_R = crate::BitReader<CTL_RX_DATA_PATTERN_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_DATA_PATTERN_SELECT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_DATA_PATTERN_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_DATA_PATTERN_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_DATA_PATTERN_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_DATA_PATTERN_SELECT_A {
        match self.bits {
            false => CTL_RX_DATA_PATTERN_SELECT_A::DISABLE,
            true => CTL_RX_DATA_PATTERN_SELECT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_DATA_PATTERN_SELECT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_DATA_PATTERN_SELECT_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_data_pattern_select` writer - "]
pub type CTL_RX_DATA_PATTERN_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_DATA_PATTERN_SELECT_A>;
impl<'a, const O: u8> CTL_RX_DATA_PATTERN_SELECT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_DATA_PATTERN_SELECT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_DATA_PATTERN_SELECT_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_custom_preamble_enable` reader - "]
pub type CTL_RX_CUSTOM_PREAMBLE_ENABLE_R = crate::BitReader<CTL_RX_CUSTOM_PREAMBLE_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_CUSTOM_PREAMBLE_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_CUSTOM_PREAMBLE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_CUSTOM_PREAMBLE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_CUSTOM_PREAMBLE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_CUSTOM_PREAMBLE_ENABLE_A {
        match self.bits {
            false => CTL_RX_CUSTOM_PREAMBLE_ENABLE_A::DISABLE,
            true => CTL_RX_CUSTOM_PREAMBLE_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_CUSTOM_PREAMBLE_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_CUSTOM_PREAMBLE_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_custom_preamble_enable` writer - "]
pub type CTL_RX_CUSTOM_PREAMBLE_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_CUSTOM_PREAMBLE_ENABLE_A>;
impl<'a, const O: u8> CTL_RX_CUSTOM_PREAMBLE_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_CUSTOM_PREAMBLE_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_CUSTOM_PREAMBLE_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_rx_prbs31_test_pattern_enable` reader - "]
pub type CTL_RX_PRBS31_TEST_PATTERN_ENABLE_R =
    crate::BitReader<CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RX_PRBS31_TEST_PATTERN_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A {
        match self.bits {
            false => CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A::DISABLE,
            true => CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_rx_prbs31_test_pattern_enable` writer - "]
pub type CTL_RX_PRBS31_TEST_PATTERN_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, RXCFG_SPEC, O, CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A>;
impl<'a, const O: u8> CTL_RX_PRBS31_TEST_PATTERN_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RX_PRBS31_TEST_PATTERN_ENABLE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_rx_enable(&self) -> CTL_RX_ENABLE_R {
        CTL_RX_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctl_rx_delete_fcs(&self) -> CTL_RX_DELETE_FCS_R {
        CTL_RX_DELETE_FCS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ctl_rx_ignore_fcs(&self) -> CTL_RX_IGNORE_FCS_R {
        CTL_RX_IGNORE_FCS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ctl_rx_process_lfi(&self) -> CTL_RX_PROCESS_LFI_R {
        CTL_RX_PROCESS_LFI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ctl_rx_check_sfd(&self) -> CTL_RX_CHECK_SFD_R {
        CTL_RX_CHECK_SFD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ctl_rx_check_preamble(&self) -> CTL_RX_CHECK_PREAMBLE_R {
        CTL_RX_CHECK_PREAMBLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ctl_rx_force_resync(&self) -> CTL_RX_FORCE_RESYNC_R {
        CTL_RX_FORCE_RESYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ctl_rx_test_pattern(&self) -> CTL_RX_TEST_PATTERN_R {
        CTL_RX_TEST_PATTERN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ctl_rx_test_pattern_enable(&self) -> CTL_RX_TEST_PATTERN_ENABLE_R {
        CTL_RX_TEST_PATTERN_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ctl_rx_data_pattern_select(&self) -> CTL_RX_DATA_PATTERN_SELECT_R {
        CTL_RX_DATA_PATTERN_SELECT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ctl_rx_custom_preamble_enable(&self) -> CTL_RX_CUSTOM_PREAMBLE_ENABLE_R {
        CTL_RX_CUSTOM_PREAMBLE_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ctl_rx_prbs31_test_pattern_enable(&self) -> CTL_RX_PRBS31_TEST_PATTERN_ENABLE_R {
        CTL_RX_PRBS31_TEST_PATTERN_ENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_enable(&mut self) -> CTL_RX_ENABLE_W<0> {
        CTL_RX_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_delete_fcs(&mut self) -> CTL_RX_DELETE_FCS_W<1> {
        CTL_RX_DELETE_FCS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_ignore_fcs(&mut self) -> CTL_RX_IGNORE_FCS_W<2> {
        CTL_RX_IGNORE_FCS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_process_lfi(&mut self) -> CTL_RX_PROCESS_LFI_W<3> {
        CTL_RX_PROCESS_LFI_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_check_sfd(&mut self) -> CTL_RX_CHECK_SFD_W<4> {
        CTL_RX_CHECK_SFD_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_check_preamble(&mut self) -> CTL_RX_CHECK_PREAMBLE_W<5> {
        CTL_RX_CHECK_PREAMBLE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_force_resync(&mut self) -> CTL_RX_FORCE_RESYNC_W<6> {
        CTL_RX_FORCE_RESYNC_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_test_pattern(&mut self) -> CTL_RX_TEST_PATTERN_W<7> {
        CTL_RX_TEST_PATTERN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_test_pattern_enable(&mut self) -> CTL_RX_TEST_PATTERN_ENABLE_W<8> {
        CTL_RX_TEST_PATTERN_ENABLE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_data_pattern_select(&mut self) -> CTL_RX_DATA_PATTERN_SELECT_W<9> {
        CTL_RX_DATA_PATTERN_SELECT_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_custom_preamble_enable(&mut self) -> CTL_RX_CUSTOM_PREAMBLE_ENABLE_W<11> {
        CTL_RX_CUSTOM_PREAMBLE_ENABLE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_prbs31_test_pattern_enable(&mut self) -> CTL_RX_PRBS31_TEST_PATTERN_ENABLE_W<12> {
        CTL_RX_PRBS31_TEST_PATTERN_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcfg](index.html) module"]
pub struct RXCFG_SPEC;
impl crate::RegisterSpec for RXCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcfg::R](R) reader structure"]
impl crate::Readable for RXCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxcfg::W](W) writer structure"]
impl crate::Writable for RXCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxcfg to value 0"]
impl crate::Resettable for RXCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `rxblsr` reader"]
pub struct R(crate::R<RXBLSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBLSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBLSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBLSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `stat_rx_block_lock` reader - "]
pub type STAT_RX_BLOCK_LOCK_R = crate::BitReader<STAT_RX_BLOCK_LOCK_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STAT_RX_BLOCK_LOCK_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<STAT_RX_BLOCK_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: STAT_RX_BLOCK_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl STAT_RX_BLOCK_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_BLOCK_LOCK_A {
        match self.bits {
            false => STAT_RX_BLOCK_LOCK_A::DISABLE,
            true => STAT_RX_BLOCK_LOCK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STAT_RX_BLOCK_LOCK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == STAT_RX_BLOCK_LOCK_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stat_rx_block_lock(&self) -> STAT_RX_BLOCK_LOCK_R {
        STAT_RX_BLOCK_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Stat RX Block Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxblsr](index.html) module"]
pub struct RXBLSR_SPEC;
impl crate::RegisterSpec for RXBLSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxblsr::R](R) reader structure"]
impl crate::Readable for RXBLSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rxblsr to value 0"]
impl crate::Resettable for RXBLSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `mm2s_dmasr` reader"]
pub struct R(crate::R<MM2S_DMASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MM2S_DMASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MM2S_DMASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MM2S_DMASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mm2s_dmasr` writer"]
pub struct W(crate::W<MM2S_DMASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MM2S_DMASR_SPEC>;
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
impl From<crate::W<MM2S_DMASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MM2S_DMASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `halted` reader - DMA Channel Halted. Indicates the run/stop state of the DMA channel."]
pub type HALTED_R = crate::BitReader<HALTED_A>;
#[doc = "DMA Channel Halted. Indicates the run/stop state of the DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALTED_A {
    #[doc = "0: `0`"]
    RUNNING = 0,
    #[doc = "1: `1`"]
    HALTED = 1,
}
impl From<HALTED_A> for bool {
    #[inline(always)]
    fn from(variant: HALTED_A) -> Self {
        variant as u8 != 0
    }
}
impl HALTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALTED_A {
        match self.bits {
            false => HALTED_A::RUNNING,
            true => HALTED_A::HALTED,
        }
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == HALTED_A::RUNNING
    }
    #[doc = "Checks if the value of the field is `HALTED`"]
    #[inline(always)]
    pub fn is_halted(&self) -> bool {
        *self == HALTED_A::HALTED
    }
}
#[doc = "Field `halted` writer - DMA Channel Halted. Indicates the run/stop state of the DMA channel."]
pub type HALTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, HALTED_A, O>;
impl<'a, const O: u8> HALTED_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(HALTED_A::RUNNING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn halted(self) -> &'a mut W {
        self.variant(HALTED_A::HALTED)
    }
}
#[doc = "Field `idle` reader - DMA Channel Idle. Indicates the state of AXI DMA operations."]
pub type IDLE_R = crate::BitReader<IDLE_A>;
#[doc = "DMA Channel Idle. Indicates the state of AXI DMA operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE_A {
    #[doc = "0: `0`"]
    NOT_IDLE = 0,
    #[doc = "1: `1`"]
    IDLE = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::NOT_IDLE,
            true => IDLE_A::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_IDLE`"]
    #[inline(always)]
    pub fn is_not_idle(&self) -> bool {
        *self == IDLE_A::NOT_IDLE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLE_A::IDLE
    }
}
#[doc = "Field `idle` writer - DMA Channel Idle. Indicates the state of AXI DMA operations."]
pub type IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, IDLE_A, O>;
impl<'a, const O: u8> IDLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn not_idle(self) -> &'a mut W {
        self.variant(IDLE_A::NOT_IDLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(IDLE_A::IDLE)
    }
}
#[doc = "Field `sg_incld` reader - Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode."]
pub type SG_INCLD_R = crate::BitReader<SG_INCLD_A>;
#[doc = "Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SG_INCLD_A {
    #[doc = "0: `0`"]
    SG_DISABLED = 0,
    #[doc = "1: `1`"]
    SG_ENABLED = 1,
}
impl From<SG_INCLD_A> for bool {
    #[inline(always)]
    fn from(variant: SG_INCLD_A) -> Self {
        variant as u8 != 0
    }
}
impl SG_INCLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SG_INCLD_A {
        match self.bits {
            false => SG_INCLD_A::SG_DISABLED,
            true => SG_INCLD_A::SG_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SG_DISABLED`"]
    #[inline(always)]
    pub fn is_sg_disabled(&self) -> bool {
        *self == SG_INCLD_A::SG_DISABLED
    }
    #[doc = "Checks if the value of the field is `SG_ENABLED`"]
    #[inline(always)]
    pub fn is_sg_enabled(&self) -> bool {
        *self == SG_INCLD_A::SG_ENABLED
    }
}
#[doc = "Field `sg_incld` writer - Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode."]
pub type SG_INCLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, SG_INCLD_A, O>;
impl<'a, const O: u8> SG_INCLD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sg_disabled(self) -> &'a mut W {
        self.variant(SG_INCLD_A::SG_DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sg_enabled(self) -> &'a mut W {
        self.variant(SG_INCLD_A::SG_ENABLED)
    }
}
#[doc = "Field `dma_int_err` reader - DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0."]
pub type DMA_INT_ERR_R = crate::BitReader<DMA_INT_ERR_A>;
#[doc = "DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_INT_ERR_A {
    #[doc = "0: `0`"]
    NO_ERR = 0,
    #[doc = "1: `1`"]
    DETECTED = 1,
}
impl From<DMA_INT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_INT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_INT_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_INT_ERR_A {
        match self.bits {
            false => DMA_INT_ERR_A::NO_ERR,
            true => DMA_INT_ERR_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERR`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == DMA_INT_ERR_A::NO_ERR
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DMA_INT_ERR_A::DETECTED
    }
}
#[doc = "Field `dma_int_err` writer - DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0."]
pub type DMA_INT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, DMA_INT_ERR_A, O>;
impl<'a, const O: u8> DMA_INT_ERR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut W {
        self.variant(DMA_INT_ERR_A::NO_ERR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DMA_INT_ERR_A::DETECTED)
    }
}
#[doc = "Field `dma_slv_err` reader - DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
pub type DMA_SLV_ERR_R = crate::BitReader<DMA_SLV_ERR_A>;
#[doc = "DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_SLV_ERR_A {
    #[doc = "0: `0`"]
    NO_ERR = 0,
    #[doc = "1: `1`"]
    DETECTED = 1,
}
impl From<DMA_SLV_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_SLV_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_SLV_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_SLV_ERR_A {
        match self.bits {
            false => DMA_SLV_ERR_A::NO_ERR,
            true => DMA_SLV_ERR_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERR`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == DMA_SLV_ERR_A::NO_ERR
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DMA_SLV_ERR_A::DETECTED
    }
}
#[doc = "Field `dma_slv_err` writer - DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
pub type DMA_SLV_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, DMA_SLV_ERR_A, O>;
impl<'a, const O: u8> DMA_SLV_ERR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut W {
        self.variant(DMA_SLV_ERR_A::NO_ERR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DMA_SLV_ERR_A::DETECTED)
    }
}
#[doc = "Field `dma_dec_err` reader - DMA Decode Error. This error occurs if the address request points to an invalid address."]
pub type DMA_DEC_ERR_R = crate::BitReader<DMA_DEC_ERR_A>;
#[doc = "DMA Decode Error. This error occurs if the address request points to an invalid address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_DEC_ERR_A {
    #[doc = "0: `0`"]
    NO_ERR = 0,
    #[doc = "1: `1`"]
    DETECTED = 1,
}
impl From<DMA_DEC_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_DEC_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_DEC_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_DEC_ERR_A {
        match self.bits {
            false => DMA_DEC_ERR_A::NO_ERR,
            true => DMA_DEC_ERR_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERR`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == DMA_DEC_ERR_A::NO_ERR
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DMA_DEC_ERR_A::DETECTED
    }
}
#[doc = "Field `dma_dec_err` writer - DMA Decode Error. This error occurs if the address request points to an invalid address."]
pub type DMA_DEC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, DMA_DEC_ERR_A, O>;
impl<'a, const O: u8> DMA_DEC_ERR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut W {
        self.variant(DMA_DEC_ERR_A::NO_ERR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DMA_DEC_ERR_A::DETECTED)
    }
}
#[doc = "Field `sg_int_err` reader - Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched."]
pub type SG_INT_ERR_R = crate::BitReader<SG_INT_ERR_A>;
#[doc = "Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SG_INT_ERR_A {
    #[doc = "0: `0`"]
    NO_ERR = 0,
    #[doc = "1: `1`"]
    DETECTED = 1,
}
impl From<SG_INT_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: SG_INT_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl SG_INT_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SG_INT_ERR_A {
        match self.bits {
            false => SG_INT_ERR_A::NO_ERR,
            true => SG_INT_ERR_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERR`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == SG_INT_ERR_A::NO_ERR
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SG_INT_ERR_A::DETECTED
    }
}
#[doc = "Field `sg_int_err` writer - Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched."]
pub type SG_INT_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, SG_INT_ERR_A, O>;
impl<'a, const O: u8> SG_INT_ERR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut W {
        self.variant(SG_INT_ERR_A::NO_ERR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SG_INT_ERR_A::DETECTED)
    }
}
#[doc = "Field `sg_slv_err` reader - Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error."]
pub type SG_SLV_ERR_R = crate::BitReader<SG_SLV_ERR_A>;
#[doc = "Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SG_SLV_ERR_A {
    #[doc = "0: `0`"]
    NO_ERR = 0,
    #[doc = "1: `1`"]
    DETECTED = 1,
}
impl From<SG_SLV_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: SG_SLV_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl SG_SLV_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SG_SLV_ERR_A {
        match self.bits {
            false => SG_SLV_ERR_A::NO_ERR,
            true => SG_SLV_ERR_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERR`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == SG_SLV_ERR_A::NO_ERR
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SG_SLV_ERR_A::DETECTED
    }
}
#[doc = "Field `sg_slv_err` writer - Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error."]
pub type SG_SLV_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, SG_SLV_ERR_A, O>;
impl<'a, const O: u8> SG_SLV_ERR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut W {
        self.variant(SG_SLV_ERR_A::NO_ERR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SG_SLV_ERR_A::DETECTED)
    }
}
#[doc = "Field `sg_dec_err` reader - Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address."]
pub type SG_DEC_ERR_R = crate::BitReader<SG_DEC_ERR_A>;
#[doc = "Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SG_DEC_ERR_A {
    #[doc = "0: `0`"]
    NO_ERR = 0,
    #[doc = "1: `1`"]
    DETECTED = 1,
}
impl From<SG_DEC_ERR_A> for bool {
    #[inline(always)]
    fn from(variant: SG_DEC_ERR_A) -> Self {
        variant as u8 != 0
    }
}
impl SG_DEC_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SG_DEC_ERR_A {
        match self.bits {
            false => SG_DEC_ERR_A::NO_ERR,
            true => SG_DEC_ERR_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERR`"]
    #[inline(always)]
    pub fn is_no_err(&self) -> bool {
        *self == SG_DEC_ERR_A::NO_ERR
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SG_DEC_ERR_A::DETECTED
    }
}
#[doc = "Field `sg_dec_err` writer - Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address."]
pub type SG_DEC_ERR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, SG_DEC_ERR_A, O>;
impl<'a, const O: u8> SG_DEC_ERR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_err(self) -> &'a mut W {
        self.variant(SG_DEC_ERR_A::NO_ERR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SG_DEC_ERR_A::DETECTED)
    }
}
#[doc = "Field `ioc_irq` reader - Interrupt on Complete (IOC)"]
pub type IOC_IRQ_R = crate::BitReader<IOC_IRQ_A>;
#[doc = "Interrupt on Complete (IOC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOC_IRQ_A {
    #[doc = "0: `0`"]
    NO_INTR = 0,
    #[doc = "1: `1`"]
    DETECTED = 1,
}
impl From<IOC_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IOC_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IOC_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOC_IRQ_A {
        match self.bits {
            false => IOC_IRQ_A::NO_INTR,
            true => IOC_IRQ_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTR`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == IOC_IRQ_A::NO_INTR
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == IOC_IRQ_A::DETECTED
    }
}
#[doc = "Field `ioc_irq` writer - Interrupt on Complete (IOC)"]
pub type IOC_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, IOC_IRQ_A, O>;
impl<'a, const O: u8> IOC_IRQ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_intr(self) -> &'a mut W {
        self.variant(IOC_IRQ_A::NO_INTR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(IOC_IRQ_A::DETECTED)
    }
}
#[doc = "Field `dly_irq` reader - Interrupt on Delay Timer"]
pub type DLY_IRQ_R = crate::BitReader<DLY_IRQ_A>;
#[doc = "Interrupt on Delay Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLY_IRQ_A {
    #[doc = "0: `0`"]
    NO_INTR = 0,
    #[doc = "1: `1`"]
    DETECTED = 1,
}
impl From<DLY_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: DLY_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl DLY_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLY_IRQ_A {
        match self.bits {
            false => DLY_IRQ_A::NO_INTR,
            true => DLY_IRQ_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTR`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == DLY_IRQ_A::NO_INTR
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DLY_IRQ_A::DETECTED
    }
}
#[doc = "Field `dly_irq` writer - Interrupt on Delay Timer"]
pub type DLY_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, DLY_IRQ_A, O>;
impl<'a, const O: u8> DLY_IRQ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_intr(self) -> &'a mut W {
        self.variant(DLY_IRQ_A::NO_INTR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DLY_IRQ_A::DETECTED)
    }
}
#[doc = "Field `err_irq` reader - Interrupt on Error"]
pub type ERR_IRQ_R = crate::BitReader<ERR_IRQ_A>;
#[doc = "Interrupt on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_IRQ_A {
    #[doc = "0: `0`"]
    NO_INTR = 0,
    #[doc = "1: `1`"]
    DETECTED = 1,
}
impl From<ERR_IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_IRQ_A {
        match self.bits {
            false => ERR_IRQ_A::NO_INTR,
            true => ERR_IRQ_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_INTR`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == ERR_IRQ_A::NO_INTR
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == ERR_IRQ_A::DETECTED
    }
}
#[doc = "Field `err_irq` writer - Interrupt on Error"]
pub type ERR_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MM2S_DMASR_SPEC, ERR_IRQ_A, O>;
impl<'a, const O: u8> ERR_IRQ_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_intr(self) -> &'a mut W {
        self.variant(ERR_IRQ_A::NO_INTR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(ERR_IRQ_A::DETECTED)
    }
}
#[doc = "Field `irq_threshold_sts` reader - Interrupt Threshold Status. Indicates current interrupt threshold value."]
pub type IRQ_THRESHOLD_STS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `irq_threshold_sts` writer - Interrupt Threshold Status. Indicates current interrupt threshold value."]
pub type IRQ_THRESHOLD_STS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MM2S_DMASR_SPEC, u8, u8, 8, O>;
#[doc = "Field `irq_delay_sts` reader - Interrupt Delay Time Status. Indicates current interrupt delay time value."]
pub type IRQ_DELAY_STS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `irq_delay_sts` writer - Interrupt Delay Time Status. Indicates current interrupt delay time value."]
pub type IRQ_DELAY_STS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MM2S_DMASR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - DMA Channel Halted. Indicates the run/stop state of the DMA channel."]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel Idle. Indicates the state of AXI DMA operations."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode."]
    #[inline(always)]
    pub fn sg_incld(&self) -> SG_INCLD_R {
        SG_INCLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0."]
    #[inline(always)]
    pub fn dma_int_err(&self) -> DMA_INT_ERR_R {
        DMA_INT_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
    #[inline(always)]
    pub fn dma_slv_err(&self) -> DMA_SLV_ERR_R {
        DMA_SLV_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Decode Error. This error occurs if the address request points to an invalid address."]
    #[inline(always)]
    pub fn dma_dec_err(&self) -> DMA_DEC_ERR_R {
        DMA_DEC_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched."]
    #[inline(always)]
    pub fn sg_int_err(&self) -> SG_INT_ERR_R {
        SG_INT_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error."]
    #[inline(always)]
    pub fn sg_slv_err(&self) -> SG_SLV_ERR_R {
        SG_SLV_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address."]
    #[inline(always)]
    pub fn sg_dec_err(&self) -> SG_DEC_ERR_R {
        SG_DEC_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt on Complete (IOC)"]
    #[inline(always)]
    pub fn ioc_irq(&self) -> IOC_IRQ_R {
        IOC_IRQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt on Delay Timer"]
    #[inline(always)]
    pub fn dly_irq(&self) -> DLY_IRQ_R {
        DLY_IRQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt on Error"]
    #[inline(always)]
    pub fn err_irq(&self) -> ERR_IRQ_R {
        ERR_IRQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Status. Indicates current interrupt threshold value."]
    #[inline(always)]
    pub fn irq_threshold_sts(&self) -> IRQ_THRESHOLD_STS_R {
        IRQ_THRESHOLD_STS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt Delay Time Status. Indicates current interrupt delay time value."]
    #[inline(always)]
    pub fn irq_delay_sts(&self) -> IRQ_DELAY_STS_R {
        IRQ_DELAY_STS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel Halted. Indicates the run/stop state of the DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HALTED_W<0> {
        HALTED_W::new(self)
    }
    #[doc = "Bit 1 - DMA Channel Idle. Indicates the state of AXI DMA operations."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IDLE_W<1> {
        IDLE_W::new(self)
    }
    #[doc = "Bit 3 - Scatter Gather Engine Included. DMASR.SGIncld = 1 indicates the Scatter Gather engine is included and the AXI DMA is configured for Scatter Gather mode."]
    #[inline(always)]
    #[must_use]
    pub fn sg_incld(&mut self) -> SG_INCLD_W<3> {
        SG_INCLD_W::new(self)
    }
    #[doc = "Bit 4 - DMA Internal Error. Internal error occurs if the buffer length specified in the fetched descriptor is set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn dma_int_err(&mut self) -> DMA_INT_ERR_W<4> {
        DMA_INT_ERR_W::new(self)
    }
    #[doc = "Bit 5 - DMA Slave Error. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
    #[inline(always)]
    #[must_use]
    pub fn dma_slv_err(&mut self) -> DMA_SLV_ERR_W<5> {
        DMA_SLV_ERR_W::new(self)
    }
    #[doc = "Bit 6 - DMA Decode Error. This error occurs if the address request points to an invalid address."]
    #[inline(always)]
    #[must_use]
    pub fn dma_dec_err(&mut self) -> DMA_DEC_ERR_W<6> {
        DMA_DEC_ERR_W::new(self)
    }
    #[doc = "Bit 8 - Scatter Gather Internal Error. This error occurs if a descriptor with the “Complete bit” already set is fetched."]
    #[inline(always)]
    #[must_use]
    pub fn sg_int_err(&mut self) -> SG_INT_ERR_W<8> {
        SG_INT_ERR_W::new(self)
    }
    #[doc = "Bit 9 - Scatter Gather Slave Error. This error occurs if the slave read from on the Memory Map interface issues a Slave error."]
    #[inline(always)]
    #[must_use]
    pub fn sg_slv_err(&mut self) -> SG_SLV_ERR_W<9> {
        SG_SLV_ERR_W::new(self)
    }
    #[doc = "Bit 10 - Scatter Gather Decode Error. This error occurs if CURDESC_PTR and/or NXTDESC_PTR points to an invalid address."]
    #[inline(always)]
    #[must_use]
    pub fn sg_dec_err(&mut self) -> SG_DEC_ERR_W<10> {
        SG_DEC_ERR_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt on Complete (IOC)"]
    #[inline(always)]
    #[must_use]
    pub fn ioc_irq(&mut self) -> IOC_IRQ_W<12> {
        IOC_IRQ_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt on Delay Timer"]
    #[inline(always)]
    #[must_use]
    pub fn dly_irq(&mut self) -> DLY_IRQ_W<13> {
        DLY_IRQ_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt on Error"]
    #[inline(always)]
    #[must_use]
    pub fn err_irq(&mut self) -> ERR_IRQ_W<14> {
        ERR_IRQ_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Status. Indicates current interrupt threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn irq_threshold_sts(&mut self) -> IRQ_THRESHOLD_STS_W<16> {
        IRQ_THRESHOLD_STS_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt Delay Time Status. Indicates current interrupt delay time value."]
    #[inline(always)]
    #[must_use]
    pub fn irq_delay_sts(&mut self) -> IRQ_DELAY_STS_W<24> {
        IRQ_DELAY_STS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MM2S DMA Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mm2s_dmasr](index.html) module"]
pub struct MM2S_DMASR_SPEC;
impl crate::RegisterSpec for MM2S_DMASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mm2s_dmasr::R](R) reader structure"]
impl crate::Readable for MM2S_DMASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mm2s_dmasr::W](W) writer structure"]
impl crate::Writable for MM2S_DMASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mm2s_dmasr to value 0"]
impl crate::Resettable for MM2S_DMASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

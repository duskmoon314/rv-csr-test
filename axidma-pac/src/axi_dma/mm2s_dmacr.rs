#[doc = "Register `mm2s_dmacr` reader"]
pub struct R(crate::R<MM2S_DMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MM2S_DMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MM2S_DMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MM2S_DMACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mm2s_dmacr` writer"]
pub struct W(crate::W<MM2S_DMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MM2S_DMACR_SPEC>;
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
impl From<crate::W<MM2S_DMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MM2S_DMACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `run_stop` reader - Run / Stop control for controlling running and stopping of the DMA channel."]
pub type RUN_STOP_R = crate::BitReader<RUN_STOP_A>;
#[doc = "Run / Stop control for controlling running and stopping of the DMA channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_STOP_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    RUN = 1,
}
impl From<RUN_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_STOP_A {
        match self.bits {
            false => RUN_STOP_A::STOP,
            true => RUN_STOP_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RUN_STOP_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == RUN_STOP_A::RUN
    }
}
#[doc = "Field `run_stop` writer - Run / Stop control for controlling running and stopping of the DMA channel."]
pub type RUN_STOP_W<'a, const O: u8> = crate::BitWriter<'a, MM2S_DMACR_SPEC, O, RUN_STOP_A>;
impl<'a, const O: u8> RUN_STOP_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(RUN_STOP_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(RUN_STOP_A::RUN)
    }
}
#[doc = "Field `reset` reader - Soft reset for resetting the AXI DMA core"]
pub type RESET_R = crate::BitReader<RESET_A>;
#[doc = "Soft reset for resetting the AXI DMA core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::NORMAL,
            true => RESET_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RESET_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESET_A::RESET
    }
}
#[doc = "Field `reset` writer - Soft reset for resetting the AXI DMA core"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, MM2S_DMACR_SPEC, O, RESET_A>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RESET_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESET_A::RESET)
    }
}
#[doc = "Field `keyhole` reader - Keyhole Read"]
pub type KEYHOLE_R = crate::BitReader<KEYHOLE_A>;
#[doc = "Keyhole Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEYHOLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<KEYHOLE_A> for bool {
    #[inline(always)]
    fn from(variant: KEYHOLE_A) -> Self {
        variant as u8 != 0
    }
}
impl KEYHOLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYHOLE_A {
        match self.bits {
            false => KEYHOLE_A::DISABLE,
            true => KEYHOLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == KEYHOLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == KEYHOLE_A::ENABLE
    }
}
#[doc = "Field `keyhole` writer - Keyhole Read"]
pub type KEYHOLE_W<'a, const O: u8> = crate::BitWriter<'a, MM2S_DMACR_SPEC, O, KEYHOLE_A>;
impl<'a, const O: u8> KEYHOLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(KEYHOLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(KEYHOLE_A::ENABLE)
    }
}
#[doc = "Field `cyclic_buffer_descriptor` reader - When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention"]
pub type CYCLIC_BUFFER_DESCRIPTOR_R = crate::BitReader<CYCLIC_BUFFER_DESCRIPTOR_A>;
#[doc = "When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CYCLIC_BUFFER_DESCRIPTOR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CYCLIC_BUFFER_DESCRIPTOR_A> for bool {
    #[inline(always)]
    fn from(variant: CYCLIC_BUFFER_DESCRIPTOR_A) -> Self {
        variant as u8 != 0
    }
}
impl CYCLIC_BUFFER_DESCRIPTOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CYCLIC_BUFFER_DESCRIPTOR_A {
        match self.bits {
            false => CYCLIC_BUFFER_DESCRIPTOR_A::DISABLE,
            true => CYCLIC_BUFFER_DESCRIPTOR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CYCLIC_BUFFER_DESCRIPTOR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CYCLIC_BUFFER_DESCRIPTOR_A::ENABLE
    }
}
#[doc = "Field `cyclic_buffer_descriptor` writer - When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention"]
pub type CYCLIC_BUFFER_DESCRIPTOR_W<'a, const O: u8> =
    crate::BitWriter<'a, MM2S_DMACR_SPEC, O, CYCLIC_BUFFER_DESCRIPTOR_A>;
impl<'a, const O: u8> CYCLIC_BUFFER_DESCRIPTOR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CYCLIC_BUFFER_DESCRIPTOR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CYCLIC_BUFFER_DESCRIPTOR_A::ENABLE)
    }
}
#[doc = "Field `ioc_irq_en` reader - Interrupt on Complete (IOC) Interrupt Enable"]
pub type IOC_IRQ_EN_R = crate::BitReader<IOC_IRQ_EN_A>;
#[doc = "Interrupt on Complete (IOC) Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOC_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<IOC_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: IOC_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl IOC_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOC_IRQ_EN_A {
        match self.bits {
            false => IOC_IRQ_EN_A::DISABLE,
            true => IOC_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IOC_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IOC_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `ioc_irq_en` writer - Interrupt on Complete (IOC) Interrupt Enable"]
pub type IOC_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, MM2S_DMACR_SPEC, O, IOC_IRQ_EN_A>;
impl<'a, const O: u8> IOC_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOC_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOC_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `dly_irq_en` reader - Interrupt on Delay Timer Interrupt Enable"]
pub type DLY_IRQ_EN_R = crate::BitReader<DLY_IRQ_EN_A>;
#[doc = "Interrupt on Delay Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLY_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<DLY_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DLY_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DLY_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLY_IRQ_EN_A {
        match self.bits {
            false => DLY_IRQ_EN_A::DISABLE,
            true => DLY_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DLY_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DLY_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `dly_irq_en` writer - Interrupt on Delay Timer Interrupt Enable"]
pub type DLY_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, MM2S_DMACR_SPEC, O, DLY_IRQ_EN_A>;
impl<'a, const O: u8> DLY_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DLY_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DLY_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `err_irq_en` reader - Interrupt on Error Interrupt Enable"]
pub type ERR_IRQ_EN_R = crate::BitReader<ERR_IRQ_EN_A>;
#[doc = "Interrupt on Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<ERR_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_IRQ_EN_A {
        match self.bits {
            false => ERR_IRQ_EN_A::DISABLE,
            true => ERR_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ERR_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ERR_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `err_irq_en` writer - Interrupt on Error Interrupt Enable"]
pub type ERR_IRQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, MM2S_DMACR_SPEC, O, ERR_IRQ_EN_A>;
impl<'a, const O: u8> ERR_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERR_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERR_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `irq_threshold` reader - Interrupt Threshold"]
pub type IRQ_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `irq_threshold` writer - Interrupt Threshold"]
pub type IRQ_THRESHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, MM2S_DMACR_SPEC, 8, O>;
#[doc = "Field `irq_delay` reader - Interrupt Delay Time Out"]
pub type IRQ_DELAY_R = crate::FieldReader;
#[doc = "Field `irq_delay` writer - Interrupt Delay Time Out"]
pub type IRQ_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, MM2S_DMACR_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0 - Run / Stop control for controlling running and stopping of the DMA channel."]
    #[inline(always)]
    pub fn run_stop(&self) -> RUN_STOP_R {
        RUN_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Soft reset for resetting the AXI DMA core"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Keyhole Read"]
    #[inline(always)]
    pub fn keyhole(&self) -> KEYHOLE_R {
        KEYHOLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention"]
    #[inline(always)]
    pub fn cyclic_buffer_descriptor(&self) -> CYCLIC_BUFFER_DESCRIPTOR_R {
        CYCLIC_BUFFER_DESCRIPTOR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt on Complete (IOC) Interrupt Enable"]
    #[inline(always)]
    pub fn ioc_irq_en(&self) -> IOC_IRQ_EN_R {
        IOC_IRQ_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt on Delay Timer Interrupt Enable"]
    #[inline(always)]
    pub fn dly_irq_en(&self) -> DLY_IRQ_EN_R {
        DLY_IRQ_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt on Error Interrupt Enable"]
    #[inline(always)]
    pub fn err_irq_en(&self) -> ERR_IRQ_EN_R {
        ERR_IRQ_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold"]
    #[inline(always)]
    pub fn irq_threshold(&self) -> IRQ_THRESHOLD_R {
        IRQ_THRESHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt Delay Time Out"]
    #[inline(always)]
    pub fn irq_delay(&self) -> IRQ_DELAY_R {
        IRQ_DELAY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Run / Stop control for controlling running and stopping of the DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn run_stop(&mut self) -> RUN_STOP_W<0> {
        RUN_STOP_W::new(self)
    }
    #[doc = "Bit 2 - Soft reset for resetting the AXI DMA core"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<2> {
        RESET_W::new(self)
    }
    #[doc = "Bit 3 - Keyhole Read"]
    #[inline(always)]
    #[must_use]
    pub fn keyhole(&mut self) -> KEYHOLE_W<3> {
        KEYHOLE_W::new(self)
    }
    #[doc = "Bit 4 - When set to 1, the DMA operates in Cyclic Buffer Descriptor (BD) mode without any user intervention"]
    #[inline(always)]
    #[must_use]
    pub fn cyclic_buffer_descriptor(&mut self) -> CYCLIC_BUFFER_DESCRIPTOR_W<4> {
        CYCLIC_BUFFER_DESCRIPTOR_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt on Complete (IOC) Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ioc_irq_en(&mut self) -> IOC_IRQ_EN_W<12> {
        IOC_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt on Delay Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dly_irq_en(&mut self) -> DLY_IRQ_EN_W<13> {
        DLY_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt on Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_irq_en(&mut self) -> ERR_IRQ_EN_W<14> {
        ERR_IRQ_EN_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn irq_threshold(&mut self) -> IRQ_THRESHOLD_W<16> {
        IRQ_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt Delay Time Out"]
    #[inline(always)]
    #[must_use]
    pub fn irq_delay(&mut self) -> IRQ_DELAY_W<24> {
        IRQ_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MM2S DMA Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mm2s_dmacr](index.html) module"]
pub struct MM2S_DMACR_SPEC;
impl crate::RegisterSpec for MM2S_DMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mm2s_dmacr::R](R) reader structure"]
impl crate::Readable for MM2S_DMACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mm2s_dmacr::W](W) writer structure"]
impl crate::Writable for MM2S_DMACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mm2s_dmacr to value 0"]
impl crate::Resettable for MM2S_DMACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

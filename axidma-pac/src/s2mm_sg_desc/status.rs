#[doc = "Register `status` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tfer_bytes` reader - This value indicates the amount of data received and stored in the buffer described by this descriptor. This might or might not match the buffer length."]
pub type TFER_BYTES_R = crate::FieldReader<u32, u32>;
#[doc = "Field `rxeof` reader - End of Frame. Flag indicating buffer holds the last part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the end of the packet."]
pub type RXEOF_R = crate::BitReader<RXEOF_A>;
#[doc = "End of Frame. Flag indicating buffer holds the last part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the end of the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEOF_A {
    #[doc = "0: `0`"]
    FALSE = 0,
    #[doc = "1: `1`"]
    TRUE = 1,
}
impl From<RXEOF_A> for bool {
    #[inline(always)]
    fn from(variant: RXEOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEOF_A {
        match self.bits {
            false => RXEOF_A::FALSE,
            true => RXEOF_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == RXEOF_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == RXEOF_A::TRUE
    }
}
#[doc = "Field `rxsof` reader - Start of Frame. Flag indicating buffer holds first part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the start of the packet."]
pub type RXSOF_R = crate::BitReader<RXSOF_A>;
#[doc = "Start of Frame. Flag indicating buffer holds first part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the start of the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSOF_A {
    #[doc = "0: `0`"]
    FALSE = 0,
    #[doc = "1: `1`"]
    TRUE = 1,
}
impl From<RXSOF_A> for bool {
    #[inline(always)]
    fn from(variant: RXSOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXSOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSOF_A {
        match self.bits {
            false => RXSOF_A::FALSE,
            true => RXSOF_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == RXSOF_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == RXSOF_A::TRUE
    }
}
#[doc = "Field `dma_int_err` reader - DMA Internal Error. Internal Error detected by primary AXI DataMover. This error can occur if a 0 length bytes to transfer is fed to the AXI DataMover."]
pub type DMA_INT_ERR_R = crate::BitReader<DMA_INT_ERR_A>;
#[doc = "DMA Internal Error. Internal Error detected by primary AXI DataMover. This error can occur if a 0 length bytes to transfer is fed to the AXI DataMover.\n\nValue on reset: 0"]
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
#[doc = "Field `dma_slv_err` reader - DMA Slave Error. Slave Error detected by primary AXI DataMover. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
pub type DMA_SLV_ERR_R = crate::BitReader<DMA_SLV_ERR_A>;
#[doc = "DMA Slave Error. Slave Error detected by primary AXI DataMover. This error occurs if the slave read from the Memory Map interface issues a Slave Error.\n\nValue on reset: 0"]
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
#[doc = "Field `cmplt` reader - Completed. This indicates to the software that the DMA Engine has completed the transfer as described by the associated descriptor."]
pub type CMPLT_R = crate::BitReader<CMPLT_A>;
#[doc = "Completed. This indicates to the software that the DMA Engine has completed the transfer as described by the associated descriptor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPLT_A {
    #[doc = "0: `0`"]
    FALSE = 0,
    #[doc = "1: `1`"]
    TRUE = 1,
}
impl From<CMPLT_A> for bool {
    #[inline(always)]
    fn from(variant: CMPLT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPLT_A {
        match self.bits {
            false => CMPLT_A::FALSE,
            true => CMPLT_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == CMPLT_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == CMPLT_A::TRUE
    }
}
#[doc = "Field `dma_dec_err` reader - DMA Decode Error. Decode Error detected by primary AXI DataMover. This error occurs if the Descriptor Buffer Address points to an invalid address."]
pub type DMA_DEC_ERR_R = crate::BitReader<DMA_DEC_ERR_A>;
#[doc = "DMA Decode Error. Decode Error detected by primary AXI DataMover. This error occurs if the Descriptor Buffer Address points to an invalid address.\n\nValue on reset: 0"]
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
impl R {
    #[doc = "Bits 0:25 - This value indicates the amount of data received and stored in the buffer described by this descriptor. This might or might not match the buffer length."]
    #[inline(always)]
    pub fn tfer_bytes(&self) -> TFER_BYTES_R {
        TFER_BYTES_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - End of Frame. Flag indicating buffer holds the last part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the end of the packet."]
    #[inline(always)]
    pub fn rxeof(&self) -> RXEOF_R {
        RXEOF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Start of Frame. Flag indicating buffer holds first part of packet. This bit is set by AXI DMA to indicate to the sw/user that the buffer associated with this descriptor contains the start of the packet."]
    #[inline(always)]
    pub fn rxsof(&self) -> RXSOF_R {
        RXSOF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Internal Error. Internal Error detected by primary AXI DataMover. This error can occur if a 0 length bytes to transfer is fed to the AXI DataMover."]
    #[inline(always)]
    pub fn dma_int_err(&self) -> DMA_INT_ERR_R {
        DMA_INT_ERR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Slave Error. Slave Error detected by primary AXI DataMover. This error occurs if the slave read from the Memory Map interface issues a Slave Error."]
    #[inline(always)]
    pub fn dma_slv_err(&self) -> DMA_SLV_ERR_R {
        DMA_SLV_ERR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 29 - Completed. This indicates to the software that the DMA Engine has completed the transfer as described by the associated descriptor."]
    #[inline(always)]
    pub fn cmplt(&self) -> CMPLT_R {
        CMPLT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Decode Error. Decode Error detected by primary AXI DataMover. This error occurs if the Descriptor Buffer Address points to an invalid address."]
    #[inline(always)]
    pub fn dma_dec_err(&self) -> DMA_DEC_ERR_R {
        DMA_DEC_ERR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

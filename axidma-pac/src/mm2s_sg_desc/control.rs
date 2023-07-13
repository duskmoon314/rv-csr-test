#[doc = "Register `control` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `control` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `buf_len` reader - Indicates the size in bytes of the transfer buffer. This value indicates the amount of bytes to transmit out on the MM2S stream."]
pub type BUF_LEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `buf_len` writer - Indicates the size in bytes of the transfer buffer. This value indicates the amount of bytes to transmit out on the MM2S stream."]
pub type BUF_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u32, u32, 26, O>;
#[doc = "Field `txeof` reader - End of Frame. Flag indicating the last buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the end of the packet."]
pub type TXEOF_R = crate::BitReader<TXEOF_A>;
#[doc = "End of Frame. Flag indicating the last buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the end of the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEOF_A {
    #[doc = "0: `0`"]
    NONE = 0,
    #[doc = "1: `1`"]
    TXEOF = 1,
}
impl From<TXEOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXEOF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEOF_A {
        match self.bits {
            false => TXEOF_A::NONE,
            true => TXEOF_A::TXEOF,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TXEOF_A::NONE
    }
    #[doc = "Checks if the value of the field is `TXEOF`"]
    #[inline(always)]
    pub fn is_txeof(&self) -> bool {
        *self == TXEOF_A::TXEOF
    }
}
#[doc = "Field `txeof` writer - End of Frame. Flag indicating the last buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the end of the packet."]
pub type TXEOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, TXEOF_A, O>;
impl<'a, const O: u8> TXEOF_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TXEOF_A::NONE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn txeof(self) -> &'a mut W {
        self.variant(TXEOF_A::TXEOF)
    }
}
#[doc = "Field `txsof` reader - Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the start of the packet."]
pub type TXSOF_R = crate::BitReader<TXSOF_A>;
#[doc = "Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the start of the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSOF_A {
    #[doc = "0: `0`"]
    NONE = 0,
    #[doc = "1: `1`"]
    TXSOF = 1,
}
impl From<TXSOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXSOF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXSOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSOF_A {
        match self.bits {
            false => TXSOF_A::NONE,
            true => TXSOF_A::TXSOF,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == TXSOF_A::NONE
    }
    #[doc = "Checks if the value of the field is `TXSOF`"]
    #[inline(always)]
    pub fn is_txsof(&self) -> bool {
        *self == TXSOF_A::TXSOF
    }
}
#[doc = "Field `txsof` writer - Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the start of the packet."]
pub type TXSOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, TXSOF_A, O>;
impl<'a, const O: u8> TXSOF_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TXSOF_A::NONE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn txsof(self) -> &'a mut W {
        self.variant(TXSOF_A::TXSOF)
    }
}
impl R {
    #[doc = "Bits 0:25 - Indicates the size in bytes of the transfer buffer. This value indicates the amount of bytes to transmit out on the MM2S stream."]
    #[inline(always)]
    pub fn buf_len(&self) -> BUF_LEN_R {
        BUF_LEN_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - End of Frame. Flag indicating the last buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the end of the packet."]
    #[inline(always)]
    pub fn txeof(&self) -> TXEOF_R {
        TXEOF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the start of the packet."]
    #[inline(always)]
    pub fn txsof(&self) -> TXSOF_R {
        TXSOF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - Indicates the size in bytes of the transfer buffer. This value indicates the amount of bytes to transmit out on the MM2S stream."]
    #[inline(always)]
    #[must_use]
    pub fn buf_len(&mut self) -> BUF_LEN_W<0> {
        BUF_LEN_W::new(self)
    }
    #[doc = "Bit 26 - End of Frame. Flag indicating the last buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the end of the packet."]
    #[inline(always)]
    #[must_use]
    pub fn txeof(&mut self) -> TXEOF_W<26> {
        TXEOF_W::new(self)
    }
    #[doc = "Bit 27 - Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the CPU to indicate to AXI DMA that this descriptor describes the start of the packet."]
    #[inline(always)]
    #[must_use]
    pub fn txsof(&mut self) -> TXSOF_W<27> {
        TXSOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

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
#[doc = "Field `buf_len` reader - Indicates the amount of space in bytes available for receiving data in an S2MM stream."]
pub type BUF_LEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `buf_len` writer - Indicates the amount of space in bytes available for receiving data in an S2MM stream."]
pub type BUF_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u32, u32, 26, O>;
#[doc = "Field `rxeof` reader - End of Frame. Flag indicating the last buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the end of the packet."]
pub type RXEOF_R = crate::BitReader<RXEOF_A>;
#[doc = "End of Frame. Flag indicating the last buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the end of the packet.\n\nValue on reset: 0"]
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
#[doc = "Field `rxeof` writer - End of Frame. Flag indicating the last buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the end of the packet."]
pub type RXEOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, RXEOF_A, O>;
impl<'a, const O: u8> RXEOF_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(RXEOF_A::FALSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(RXEOF_A::TRUE)
    }
}
#[doc = "Field `rxsof` reader - Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the start of the packet."]
pub type RXSOF_R = crate::BitReader<RXSOF_A>;
#[doc = "Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the start of the packet.\n\nValue on reset: 0"]
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
#[doc = "Field `rxsof` writer - Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the start of the packet."]
pub type RXSOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, RXSOF_A, O>;
impl<'a, const O: u8> RXSOF_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn false_(self) -> &'a mut W {
        self.variant(RXSOF_A::FALSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn true_(self) -> &'a mut W {
        self.variant(RXSOF_A::TRUE)
    }
}
impl R {
    #[doc = "Bits 0:25 - Indicates the amount of space in bytes available for receiving data in an S2MM stream."]
    #[inline(always)]
    pub fn buf_len(&self) -> BUF_LEN_R {
        BUF_LEN_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - End of Frame. Flag indicating the last buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the end of the packet."]
    #[inline(always)]
    pub fn rxeof(&self) -> RXEOF_R {
        RXEOF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the start of the packet."]
    #[inline(always)]
    pub fn rxsof(&self) -> RXSOF_R {
        RXSOF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - Indicates the amount of space in bytes available for receiving data in an S2MM stream."]
    #[inline(always)]
    #[must_use]
    pub fn buf_len(&mut self) -> BUF_LEN_W<0> {
        BUF_LEN_W::new(self)
    }
    #[doc = "Bit 26 - End of Frame. Flag indicating the last buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the end of the packet."]
    #[inline(always)]
    #[must_use]
    pub fn rxeof(&mut self) -> RXEOF_W<26> {
        RXEOF_W::new(self)
    }
    #[doc = "Bit 27 - Start of Frame. Flag indicating the first buffer to be processed. This flag is set by the sw/user to indicate to AXI DMA that this descriptor describes the start of the packet."]
    #[inline(always)]
    #[must_use]
    pub fn rxsof(&mut self) -> RXSOF_W<27> {
        RXSOF_W::new(self)
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

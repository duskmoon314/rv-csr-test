#[doc = "Register `s2mm_da` reader"]
pub struct R(crate::R<S2MM_DA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S2MM_DA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S2MM_DA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S2MM_DA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `s2mm_da` writer"]
pub struct W(crate::W<S2MM_DA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S2MM_DA_SPEC>;
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
impl From<crate::W<S2MM_DA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S2MM_DA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dst_addr` reader - Indicates the destination address the AXI DMA writes to transfer data from AXI4-Stream on S2MM Channel."]
pub type DST_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `dst_addr` writer - Indicates the destination address the AXI DMA writes to transfer data from AXI4-Stream on S2MM Channel."]
pub type DST_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, S2MM_DA_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the destination address the AXI DMA writes to transfer data from AXI4-Stream on S2MM Channel."]
    #[inline(always)]
    pub fn dst_addr(&self) -> DST_ADDR_R {
        DST_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates the destination address the AXI DMA writes to transfer data from AXI4-Stream on S2MM Channel."]
    #[inline(always)]
    #[must_use]
    pub fn dst_addr(&mut self) -> DST_ADDR_W<0> {
        DST_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S2MM Destination Address. Lower 32 bit address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2mm_da](index.html) module"]
pub struct S2MM_DA_SPEC;
impl crate::RegisterSpec for S2MM_DA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s2mm_da::R](R) reader structure"]
impl crate::Readable for S2MM_DA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s2mm_da::W](W) writer structure"]
impl crate::Writable for S2MM_DA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets s2mm_da to value 0"]
impl crate::Resettable for S2MM_DA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

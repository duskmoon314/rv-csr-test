#[doc = "Register `mm2s_sa` reader"]
pub struct R(crate::R<MM2S_SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MM2S_SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MM2S_SA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MM2S_SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mm2s_sa` writer"]
pub struct W(crate::W<MM2S_SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MM2S_SA_SPEC>;
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
impl From<crate::W<MM2S_SA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MM2S_SA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `src_addr` reader - Indicates the source address AXI DMA reads from to transfer data to AXI4-Stream on the MM2S Channel."]
pub type SRC_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `src_addr` writer - Indicates the source address AXI DMA reads from to transfer data to AXI4-Stream on the MM2S Channel."]
pub type SRC_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, MM2S_SA_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the source address AXI DMA reads from to transfer data to AXI4-Stream on the MM2S Channel."]
    #[inline(always)]
    pub fn src_addr(&self) -> SRC_ADDR_R {
        SRC_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates the source address AXI DMA reads from to transfer data to AXI4-Stream on the MM2S Channel."]
    #[inline(always)]
    #[must_use]
    pub fn src_addr(&mut self) -> SRC_ADDR_W<0> {
        SRC_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MM2S Source Address. Lower 32 bits of the address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mm2s_sa](index.html) module"]
pub struct MM2S_SA_SPEC;
impl crate::RegisterSpec for MM2S_SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mm2s_sa::R](R) reader structure"]
impl crate::Readable for MM2S_SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mm2s_sa::W](W) writer structure"]
impl crate::Writable for MM2S_SA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mm2s_sa to value 0"]
impl crate::Resettable for MM2S_SA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

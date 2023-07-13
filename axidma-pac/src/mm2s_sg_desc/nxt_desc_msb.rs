#[doc = "Register `nxt_desc_msb` reader"]
pub struct R(crate::R<NXT_DESC_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NXT_DESC_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NXT_DESC_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NXT_DESC_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `nxt_desc_msb` writer"]
pub struct W(crate::W<NXT_DESC_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NXT_DESC_MSB_SPEC>;
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
impl From<crate::W<NXT_DESC_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NXT_DESC_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `nxt_desc_ptr` reader - Indicates the MSB 32 bits of the pointer pointing to the first word of the next descriptor"]
pub type NXT_DESC_PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `nxt_desc_ptr` writer - Indicates the MSB 32 bits of the pointer pointing to the first word of the next descriptor"]
pub type NXT_DESC_PTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NXT_DESC_MSB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Indicates the MSB 32 bits of the pointer pointing to the first word of the next descriptor"]
    #[inline(always)]
    pub fn nxt_desc_ptr(&self) -> NXT_DESC_PTR_R {
        NXT_DESC_PTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates the MSB 32 bits of the pointer pointing to the first word of the next descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn nxt_desc_ptr(&mut self) -> NXT_DESC_PTR_W<0> {
        NXT_DESC_PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper 32 bits of Next Descriptor Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nxt_desc_msb](index.html) module"]
pub struct NXT_DESC_MSB_SPEC;
impl crate::RegisterSpec for NXT_DESC_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nxt_desc_msb::R](R) reader structure"]
impl crate::Readable for NXT_DESC_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nxt_desc_msb::W](W) writer structure"]
impl crate::Writable for NXT_DESC_MSB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets nxt_desc_msb to value 0"]
impl crate::Resettable for NXT_DESC_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

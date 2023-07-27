#[doc = "Register `s2mm_taildesc_msb` reader"]
pub struct R(crate::R<S2MM_TAILDESC_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S2MM_TAILDESC_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S2MM_TAILDESC_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S2MM_TAILDESC_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `s2mm_taildesc_msb` writer"]
pub struct W(crate::W<S2MM_TAILDESC_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S2MM_TAILDESC_MSB_SPEC>;
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
impl From<crate::W<S2MM_TAILDESC_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S2MM_TAILDESC_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `taildesc_ptr` reader - Indicates the pause pointer in a descriptor chain."]
pub type TAILDESC_PTR_R = crate::FieldReader<u32>;
#[doc = "Field `taildesc_ptr` writer - Indicates the pause pointer in a descriptor chain."]
pub type TAILDESC_PTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, S2MM_TAILDESC_MSB_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the pause pointer in a descriptor chain."]
    #[inline(always)]
    pub fn taildesc_ptr(&self) -> TAILDESC_PTR_R {
        TAILDESC_PTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indicates the pause pointer in a descriptor chain."]
    #[inline(always)]
    #[must_use]
    pub fn taildesc_ptr(&mut self) -> TAILDESC_PTR_W<0> {
        TAILDESC_PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S2MM Tail Descriptor Pointer. Upper 32 address bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2mm_taildesc_msb](index.html) module"]
pub struct S2MM_TAILDESC_MSB_SPEC;
impl crate::RegisterSpec for S2MM_TAILDESC_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s2mm_taildesc_msb::R](R) reader structure"]
impl crate::Readable for S2MM_TAILDESC_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s2mm_taildesc_msb::W](W) writer structure"]
impl crate::Writable for S2MM_TAILDESC_MSB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets s2mm_taildesc_msb to value 0"]
impl crate::Resettable for S2MM_TAILDESC_MSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

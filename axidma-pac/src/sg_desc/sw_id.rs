#[doc = "Register `sw_id` reader"]
pub struct R(crate::R<SW_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SW_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SW_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SW_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sw_id` writer"]
pub struct W(crate::W<SW_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SW_ID_SPEC>;
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
impl From<crate::W<SW_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SW_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `id` reader - "]
pub type ID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `id` writer - "]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SW_ID_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_id](index.html) module"]
pub struct SW_ID_SPEC;
impl crate::RegisterSpec for SW_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sw_id::R](R) reader structure"]
impl crate::Readable for SW_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sw_id::W](W) writer structure"]
impl crate::Writable for SW_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sw_id to value 0"]
impl crate::Resettable for SW_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

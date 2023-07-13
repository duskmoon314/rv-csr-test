#[doc = "Register `mm2s_curdesc` reader"]
pub struct R(crate::R<MM2S_CURDESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MM2S_CURDESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MM2S_CURDESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MM2S_CURDESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mm2s_curdesc` writer"]
pub struct W(crate::W<MM2S_CURDESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MM2S_CURDESC_SPEC>;
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
impl From<crate::W<MM2S_CURDESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MM2S_CURDESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `curdesc_ptr` reader - Indicates the pointer of the current descriptor being worked on."]
pub type CURDESC_PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `curdesc_ptr` writer - Indicates the pointer of the current descriptor being worked on."]
pub type CURDESC_PTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MM2S_CURDESC_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 6:31 - Indicates the pointer of the current descriptor being worked on."]
    #[inline(always)]
    pub fn curdesc_ptr(&self) -> CURDESC_PTR_R {
        CURDESC_PTR_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 6:31 - Indicates the pointer of the current descriptor being worked on."]
    #[inline(always)]
    #[must_use]
    pub fn curdesc_ptr(&mut self) -> CURDESC_PTR_W<6> {
        CURDESC_PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MM2S Current Descriptor Pointer. Lower 32 bits of the address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mm2s_curdesc](index.html) module"]
pub struct MM2S_CURDESC_SPEC;
impl crate::RegisterSpec for MM2S_CURDESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mm2s_curdesc::R](R) reader structure"]
impl crate::Readable for MM2S_CURDESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mm2s_curdesc::W](W) writer structure"]
impl crate::Writable for MM2S_CURDESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mm2s_curdesc to value 0"]
impl crate::Resettable for MM2S_CURDESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

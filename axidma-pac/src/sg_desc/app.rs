#[doc = "Register `app[%s]` reader"]
pub struct R(crate::R<APP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `app[%s]` writer"]
pub struct W(crate::W<APP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_SPEC>;
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
impl From<crate::W<APP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `app` reader - Specifies user-specific application data."]
pub type APP_R = crate::FieldReader<u32>;
#[doc = "Field `app` writer - Specifies user-specific application data."]
pub type APP_W<'a, const O: u8> = crate::FieldWriter<'a, APP_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies user-specific application data."]
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies user-specific application data."]
    #[inline(always)]
    #[must_use]
    pub fn app(&mut self) -> APP_W<0> {
        APP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Application Field \\[%s\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app](index.html) module"]
pub struct APP_SPEC;
impl crate::RegisterSpec for APP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app::R](R) reader structure"]
impl crate::Readable for APP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app::W](W) writer structure"]
impl crate::Writable for APP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets app[%s]
to value 0"]
impl crate::Resettable for APP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

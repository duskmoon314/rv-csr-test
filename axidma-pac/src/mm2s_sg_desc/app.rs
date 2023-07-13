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
#[doc = "Field `app` reader - Specifies user-specific application data. When Status Control Stream is enabled, the Application (APP) fields of the Start of Frame (SOF) Descriptor are transmitted to the AXI Control Stream."]
pub type APP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies user-specific application data. When Status Control Stream is enabled, the Application (APP) fields of the Start of Frame (SOF) Descriptor are transmitted to the AXI Control Stream."]
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(self.bits)
    }
}
#[doc = "User Application Field \\[%s\\]\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app](index.html) module"]
pub struct APP_SPEC;
impl crate::RegisterSpec for APP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app::R](R) reader structure"]
impl crate::Readable for APP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets app[%s]
to value 0"]
impl crate::Resettable for APP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

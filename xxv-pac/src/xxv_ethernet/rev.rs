#[doc = "Register `rev` reader"]
pub struct R(crate::R<REV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `major_rev` reader - "]
pub type MAJOR_REV_R = crate::FieldReader;
#[doc = "Field `minor_rev` reader - "]
pub type MINOR_REV_R = crate::FieldReader;
#[doc = "Field `patch_rev` reader - "]
pub type PATCH_REV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn major_rev(&self) -> MAJOR_REV_R {
        MAJOR_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn minor_rev(&self) -> MINOR_REV_R {
        MINOR_REV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn patch_rev(&self) -> PATCH_REV_R {
        PATCH_REV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Revision Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rev](index.html) module"]
pub struct REV_SPEC;
impl crate::RegisterSpec for REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rev::R](R) reader structure"]
impl crate::Readable for REV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rev to value 0"]
impl crate::Resettable for REV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

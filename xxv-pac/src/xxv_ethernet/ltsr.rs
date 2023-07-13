#[doc = "Register `ltsr` reader"]
pub struct R(crate::R<LTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ltsr` writer"]
pub struct W(crate::W<LTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTSR_SPEC>;
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
impl From<crate::W<LTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_lt_pseudo_seed0` reader - "]
pub type CTL_LT_PSEUDO_SEED0_R = crate::FieldReader<u16>;
#[doc = "Field `ctl_lt_pseudo_seed0` writer - "]
pub type CTL_LT_PSEUDO_SEED0_W<'a, const O: u8> = crate::FieldWriter<'a, LTSR_SPEC, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn ctl_lt_pseudo_seed0(&self) -> CTL_LT_PSEUDO_SEED0_R {
        CTL_LT_PSEUDO_SEED0_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_pseudo_seed0(&mut self) -> CTL_LT_PSEUDO_SEED0_W<0> {
        CTL_LT_PSEUDO_SEED0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LT_SEED Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltsr](index.html) module"]
pub struct LTSR_SPEC;
impl crate::RegisterSpec for LTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltsr::R](R) reader structure"]
impl crate::Readable for LTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltsr::W](W) writer structure"]
impl crate::Writable for LTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ltsr to value 0"]
impl crate::Resettable for LTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

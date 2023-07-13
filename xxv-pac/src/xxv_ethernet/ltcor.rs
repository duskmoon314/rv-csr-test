#[doc = "Register `ltcor` reader"]
pub struct R(crate::R<LTCOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTCOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTCOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTCOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ltcor` writer"]
pub struct W(crate::W<LTCOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTCOR_SPEC>;
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
impl From<crate::W<LTCOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTCOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_lt_k_p1_to_tx0` reader - "]
pub type CTL_LT_K_P1_TO_TX0_R = crate::FieldReader;
#[doc = "Field `ctl_lt_k_p1_to_tx0` writer - "]
pub type CTL_LT_K_P1_TO_TX0_W<'a, const O: u8> = crate::FieldWriter<'a, LTCOR_SPEC, 2, O>;
#[doc = "Field `ctl_lt_k0_to_tx0` reader - "]
pub type CTL_LT_K0_TO_TX0_R = crate::FieldReader;
#[doc = "Field `ctl_lt_k0_to_tx0` writer - "]
pub type CTL_LT_K0_TO_TX0_W<'a, const O: u8> = crate::FieldWriter<'a, LTCOR_SPEC, 2, O>;
#[doc = "Field `ctl_lt_k_m1_to_tx0` reader - "]
pub type CTL_LT_K_M1_TO_TX0_R = crate::FieldReader;
#[doc = "Field `ctl_lt_k_m1_to_tx0` writer - "]
pub type CTL_LT_K_M1_TO_TX0_W<'a, const O: u8> = crate::FieldWriter<'a, LTCOR_SPEC, 2, O>;
#[doc = "Field `ctl_lt_stat_p1_to_tx0` reader - "]
pub type CTL_LT_STAT_P1_TO_TX0_R = crate::FieldReader;
#[doc = "Field `ctl_lt_stat_p1_to_tx0` writer - "]
pub type CTL_LT_STAT_P1_TO_TX0_W<'a, const O: u8> = crate::FieldWriter<'a, LTCOR_SPEC, 2, O>;
#[doc = "Field `ctl_lt_stat0_to_tx0` reader - "]
pub type CTL_LT_STAT0_TO_TX0_R = crate::FieldReader;
#[doc = "Field `ctl_lt_stat0_to_tx0` writer - "]
pub type CTL_LT_STAT0_TO_TX0_W<'a, const O: u8> = crate::FieldWriter<'a, LTCOR_SPEC, 2, O>;
#[doc = "Field `ctl_lt_stat_m1_to_tx0` reader - "]
pub type CTL_LT_STAT_M1_TO_TX0_R = crate::FieldReader;
#[doc = "Field `ctl_lt_stat_m1_to_tx0` writer - "]
pub type CTL_LT_STAT_M1_TO_TX0_W<'a, const O: u8> = crate::FieldWriter<'a, LTCOR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ctl_lt_k_p1_to_tx0(&self) -> CTL_LT_K_P1_TO_TX0_R {
        CTL_LT_K_P1_TO_TX0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ctl_lt_k0_to_tx0(&self) -> CTL_LT_K0_TO_TX0_R {
        CTL_LT_K0_TO_TX0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ctl_lt_k_m1_to_tx0(&self) -> CTL_LT_K_M1_TO_TX0_R {
        CTL_LT_K_M1_TO_TX0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn ctl_lt_stat_p1_to_tx0(&self) -> CTL_LT_STAT_P1_TO_TX0_R {
        CTL_LT_STAT_P1_TO_TX0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ctl_lt_stat0_to_tx0(&self) -> CTL_LT_STAT0_TO_TX0_R {
        CTL_LT_STAT0_TO_TX0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ctl_lt_stat_m1_to_tx0(&self) -> CTL_LT_STAT_M1_TO_TX0_R {
        CTL_LT_STAT_M1_TO_TX0_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_k_p1_to_tx0(&mut self) -> CTL_LT_K_P1_TO_TX0_W<0> {
        CTL_LT_K_P1_TO_TX0_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_k0_to_tx0(&mut self) -> CTL_LT_K0_TO_TX0_W<2> {
        CTL_LT_K0_TO_TX0_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_k_m1_to_tx0(&mut self) -> CTL_LT_K_M1_TO_TX0_W<4> {
        CTL_LT_K_M1_TO_TX0_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_stat_p1_to_tx0(&mut self) -> CTL_LT_STAT_P1_TO_TX0_W<6> {
        CTL_LT_STAT_P1_TO_TX0_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_stat0_to_tx0(&mut self) -> CTL_LT_STAT0_TO_TX0_W<8> {
        CTL_LT_STAT0_TO_TX0_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_stat_m1_to_tx0(&mut self) -> CTL_LT_STAT_M1_TO_TX0_W<10> {
        CTL_LT_STAT_M1_TO_TX0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LT_COEFFICIENT Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltcor](index.html) module"]
pub struct LTCOR_SPEC;
impl crate::RegisterSpec for LTCOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltcor::R](R) reader structure"]
impl crate::Readable for LTCOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltcor::W](W) writer structure"]
impl crate::Writable for LTCOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ltcor to value 0"]
impl crate::Resettable for LTCOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

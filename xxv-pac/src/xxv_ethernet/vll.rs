#[doc = "Register `vll` reader"]
pub struct R(crate::R<VLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vll` writer"]
pub struct W(crate::W<VLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLL_SPEC>;
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
impl From<crate::W<VLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_tx_vl_length_minus1` reader - "]
pub type CTL_TX_VL_LENGTH_MINUS1_R = crate::FieldReader<u16>;
#[doc = "Field `ctl_tx_vl_length_minus1` writer - "]
pub type CTL_TX_VL_LENGTH_MINUS1_W<'a, const O: u8> = crate::FieldWriter<'a, VLL_SPEC, 16, O, u16>;
#[doc = "Field `ctl_rx_vl_length_minus1` reader - "]
pub type CTL_RX_VL_LENGTH_MINUS1_R = crate::FieldReader<u16>;
#[doc = "Field `ctl_rx_vl_length_minus1` writer - "]
pub type CTL_RX_VL_LENGTH_MINUS1_W<'a, const O: u8> = crate::FieldWriter<'a, VLL_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ctl_tx_vl_length_minus1(&self) -> CTL_TX_VL_LENGTH_MINUS1_R {
        CTL_TX_VL_LENGTH_MINUS1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ctl_rx_vl_length_minus1(&self) -> CTL_RX_VL_LENGTH_MINUS1_R {
        CTL_RX_VL_LENGTH_MINUS1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_tx_vl_length_minus1(&mut self) -> CTL_TX_VL_LENGTH_MINUS1_W<0> {
        CTL_TX_VL_LENGTH_MINUS1_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_vl_length_minus1(&mut self) -> CTL_RX_VL_LENGTH_MINUS1_W<16> {
        CTL_RX_VL_LENGTH_MINUS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VL Length Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vll](index.html) module"]
pub struct VLL_SPEC;
impl crate::RegisterSpec for VLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vll::R](R) reader structure"]
impl crate::Readable for VLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vll::W](W) writer structure"]
impl crate::Writable for VLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets vll to value 0"]
impl crate::Resettable for VLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

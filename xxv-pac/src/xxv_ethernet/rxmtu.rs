#[doc = "Register `rxmtu` reader"]
pub struct R(crate::R<RXMTU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMTU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMTU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMTU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxmtu` writer"]
pub struct W(crate::W<RXMTU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMTU_SPEC>;
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
impl From<crate::W<RXMTU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMTU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_rx_min_packet_len` reader - "]
pub type CTL_RX_MIN_PACKET_LEN_R = crate::FieldReader;
#[doc = "Field `ctl_rx_min_packet_len` writer - "]
pub type CTL_RX_MIN_PACKET_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, RXMTU_SPEC, 8, O>;
#[doc = "Field `ctl_rx_max_packet_len` reader - "]
pub type CTL_RX_MAX_PACKET_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `ctl_rx_max_packet_len` writer - "]
pub type CTL_RX_MAX_PACKET_LEN_W<'a, const O: u8> = crate::FieldWriter<'a, RXMTU_SPEC, 15, O, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ctl_rx_min_packet_len(&self) -> CTL_RX_MIN_PACKET_LEN_R {
        CTL_RX_MIN_PACKET_LEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    pub fn ctl_rx_max_packet_len(&self) -> CTL_RX_MAX_PACKET_LEN_R {
        CTL_RX_MAX_PACKET_LEN_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_min_packet_len(&mut self) -> CTL_RX_MIN_PACKET_LEN_W<0> {
        CTL_RX_MIN_PACKET_LEN_W::new(self)
    }
    #[doc = "Bits 16:30"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_rx_max_packet_len(&mut self) -> CTL_RX_MAX_PACKET_LEN_W<16> {
        CTL_RX_MAX_PACKET_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx MTU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmtu](index.html) module"]
pub struct RXMTU_SPEC;
impl crate::RegisterSpec for RXMTU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmtu::R](R) reader structure"]
impl crate::Readable for RXMTU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmtu::W](W) writer structure"]
impl crate::Writable for RXMTU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxmtu to value 0"]
impl crate::Resettable for RXMTU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `sg_ctl` reader"]
pub struct R(crate::R<SG_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SG_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SG_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SG_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sg_ctl` writer"]
pub struct W(crate::W<SG_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SG_CTL_SPEC>;
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
impl From<crate::W<SG_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SG_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sg_cache` reader - Scatter/Gather Cache Control. Values written in this register reflect on the m_axi_sg_arcache and m_axi_sg_awcache signals of the M_AXI_SG interface."]
pub type SG_CACHE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sg_cache` writer - Scatter/Gather Cache Control. Values written in this register reflect on the m_axi_sg_arcache and m_axi_sg_awcache signals of the M_AXI_SG interface."]
pub type SG_CACHE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SG_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `sg_user` reader - Scatter/Gather User Control. Values written in this register reflect on the m_axi_sg_aruser and m_axi_sg_awuser signals of the M_AXI_SG interface."]
pub type SG_USER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sg_user` writer - Scatter/Gather User Control. Values written in this register reflect on the m_axi_sg_aruser and m_axi_sg_awuser signals of the M_AXI_SG interface."]
pub type SG_USER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SG_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Scatter/Gather Cache Control. Values written in this register reflect on the m_axi_sg_arcache and m_axi_sg_awcache signals of the M_AXI_SG interface."]
    #[inline(always)]
    pub fn sg_cache(&self) -> SG_CACHE_R {
        SG_CACHE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Scatter/Gather User Control. Values written in this register reflect on the m_axi_sg_aruser and m_axi_sg_awuser signals of the M_AXI_SG interface."]
    #[inline(always)]
    pub fn sg_user(&self) -> SG_USER_R {
        SG_USER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Scatter/Gather Cache Control. Values written in this register reflect on the m_axi_sg_arcache and m_axi_sg_awcache signals of the M_AXI_SG interface."]
    #[inline(always)]
    #[must_use]
    pub fn sg_cache(&mut self) -> SG_CACHE_W<0> {
        SG_CACHE_W::new(self)
    }
    #[doc = "Bits 8:11 - Scatter/Gather User Control. Values written in this register reflect on the m_axi_sg_aruser and m_axi_sg_awuser signals of the M_AXI_SG interface."]
    #[inline(always)]
    #[must_use]
    pub fn sg_user(&mut self) -> SG_USER_W<8> {
        SG_USER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scatter/Gather User and Cache\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sg_ctl](index.html) module"]
pub struct SG_CTL_SPEC;
impl crate::RegisterSpec for SG_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sg_ctl::R](R) reader structure"]
impl crate::Readable for SG_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sg_ctl::W](W) writer structure"]
impl crate::Writable for SG_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sg_ctl to value 0"]
impl crate::Resettable for SG_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

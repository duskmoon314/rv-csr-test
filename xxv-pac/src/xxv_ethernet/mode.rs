#[doc = "Register `mode` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mode` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `en_wr_slverr_indication` reader - "]
pub type EN_WR_SLVERR_INDICATION_R = crate::BitReader<EN_WR_SLVERR_INDICATION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_WR_SLVERR_INDICATION_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<EN_WR_SLVERR_INDICATION_A> for bool {
    #[inline(always)]
    fn from(variant: EN_WR_SLVERR_INDICATION_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_WR_SLVERR_INDICATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_WR_SLVERR_INDICATION_A {
        match self.bits {
            false => EN_WR_SLVERR_INDICATION_A::DISABLE,
            true => EN_WR_SLVERR_INDICATION_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_WR_SLVERR_INDICATION_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_WR_SLVERR_INDICATION_A::ENABLE
    }
}
#[doc = "Field `en_wr_slverr_indication` writer - "]
pub type EN_WR_SLVERR_INDICATION_W<'a, const O: u8> =
    crate::BitWriter<'a, MODE_SPEC, O, EN_WR_SLVERR_INDICATION_A>;
impl<'a, const O: u8> EN_WR_SLVERR_INDICATION_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_WR_SLVERR_INDICATION_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_WR_SLVERR_INDICATION_A::ENABLE)
    }
}
#[doc = "Field `en_rd_slverr_indication` reader - "]
pub type EN_RD_SLVERR_INDICATION_R = crate::BitReader<EN_RD_SLVERR_INDICATION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_RD_SLVERR_INDICATION_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<EN_RD_SLVERR_INDICATION_A> for bool {
    #[inline(always)]
    fn from(variant: EN_RD_SLVERR_INDICATION_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_RD_SLVERR_INDICATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_RD_SLVERR_INDICATION_A {
        match self.bits {
            false => EN_RD_SLVERR_INDICATION_A::DISABLE,
            true => EN_RD_SLVERR_INDICATION_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_RD_SLVERR_INDICATION_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_RD_SLVERR_INDICATION_A::ENABLE
    }
}
#[doc = "Field `en_rd_slverr_indication` writer - "]
pub type EN_RD_SLVERR_INDICATION_W<'a, const O: u8> =
    crate::BitWriter<'a, MODE_SPEC, O, EN_RD_SLVERR_INDICATION_A>;
impl<'a, const O: u8> EN_RD_SLVERR_INDICATION_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_RD_SLVERR_INDICATION_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_RD_SLVERR_INDICATION_A::ENABLE)
    }
}
#[doc = "Field `tick_reg_mode_sel` reader - "]
pub type TICK_REG_MODE_SEL_R = crate::BitReader<TICK_REG_MODE_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TICK_REG_MODE_SEL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TICK_REG_MODE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TICK_REG_MODE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TICK_REG_MODE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICK_REG_MODE_SEL_A {
        match self.bits {
            false => TICK_REG_MODE_SEL_A::DISABLE,
            true => TICK_REG_MODE_SEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TICK_REG_MODE_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TICK_REG_MODE_SEL_A::ENABLE
    }
}
#[doc = "Field `tick_reg_mode_sel` writer - "]
pub type TICK_REG_MODE_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, MODE_SPEC, O, TICK_REG_MODE_SEL_A>;
impl<'a, const O: u8> TICK_REG_MODE_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TICK_REG_MODE_SEL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TICK_REG_MODE_SEL_A::ENABLE)
    }
}
#[doc = "Field `ctl_local_loopback` reader - "]
pub type CTL_LOCAL_LOOPBACK_R = crate::BitReader<CTL_LOCAL_LOOPBACK_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_LOCAL_LOOPBACK_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_LOCAL_LOOPBACK_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_LOCAL_LOOPBACK_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_LOCAL_LOOPBACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_LOCAL_LOOPBACK_A {
        match self.bits {
            false => CTL_LOCAL_LOOPBACK_A::DISABLE,
            true => CTL_LOCAL_LOOPBACK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_LOCAL_LOOPBACK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_LOCAL_LOOPBACK_A::ENABLE
    }
}
#[doc = "Field `ctl_local_loopback` writer - "]
pub type CTL_LOCAL_LOOPBACK_W<'a, const O: u8> =
    crate::BitWriter<'a, MODE_SPEC, O, CTL_LOCAL_LOOPBACK_A>;
impl<'a, const O: u8> CTL_LOCAL_LOOPBACK_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_LOCAL_LOOPBACK_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_LOCAL_LOOPBACK_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en_wr_slverr_indication(&self) -> EN_WR_SLVERR_INDICATION_R {
        EN_WR_SLVERR_INDICATION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn en_rd_slverr_indication(&self) -> EN_RD_SLVERR_INDICATION_R {
        EN_RD_SLVERR_INDICATION_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tick_reg_mode_sel(&self) -> TICK_REG_MODE_SEL_R {
        TICK_REG_MODE_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ctl_local_loopback(&self) -> CTL_LOCAL_LOOPBACK_R {
        CTL_LOCAL_LOOPBACK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en_wr_slverr_indication(&mut self) -> EN_WR_SLVERR_INDICATION_W<0> {
        EN_WR_SLVERR_INDICATION_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn en_rd_slverr_indication(&mut self) -> EN_RD_SLVERR_INDICATION_W<1> {
        EN_RD_SLVERR_INDICATION_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn tick_reg_mode_sel(&mut self) -> TICK_REG_MODE_SEL_W<30> {
        TICK_REG_MODE_SEL_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_local_loopback(&mut self) -> CTL_LOCAL_LOOPBACK_W<31> {
        CTL_LOCAL_LOOPBACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mode to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `ancr2` reader"]
pub struct R(crate::R<ANCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ancr2` writer"]
pub struct W(crate::W<ANCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANCR2_SPEC>;
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
impl From<crate::W<ANCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_an_pause` reader - "]
pub type CTL_AN_PAUSE_R = crate::BitReader<CTL_AN_PAUSE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_PAUSE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_PAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_PAUSE_A {
        match self.bits {
            false => CTL_AN_PAUSE_A::DISABLE,
            true => CTL_AN_PAUSE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_PAUSE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_PAUSE_A::ENABLE
    }
}
#[doc = "Field `ctl_an_pause` writer - "]
pub type CTL_AN_PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, ANCR2_SPEC, O, CTL_AN_PAUSE_A>;
impl<'a, const O: u8> CTL_AN_PAUSE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_PAUSE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_PAUSE_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_asmdir` reader - "]
pub type CTL_AN_ASMDIR_R = crate::BitReader<CTL_AN_ASMDIR_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_ASMDIR_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_ASMDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_ASMDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_ASMDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_ASMDIR_A {
        match self.bits {
            false => CTL_AN_ASMDIR_A::DISABLE,
            true => CTL_AN_ASMDIR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_ASMDIR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_ASMDIR_A::ENABLE
    }
}
#[doc = "Field `ctl_an_asmdir` writer - "]
pub type CTL_AN_ASMDIR_W<'a, const O: u8> = crate::BitWriter<'a, ANCR2_SPEC, O, CTL_AN_ASMDIR_A>;
impl<'a, const O: u8> CTL_AN_ASMDIR_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_ASMDIR_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_ASMDIR_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_fec_10g_request` reader - "]
pub type CTL_AN_FEC_10G_REQUEST_R = crate::BitReader<CTL_AN_FEC_10G_REQUEST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_FEC_10G_REQUEST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_FEC_10G_REQUEST_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_FEC_10G_REQUEST_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_FEC_10G_REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_FEC_10G_REQUEST_A {
        match self.bits {
            false => CTL_AN_FEC_10G_REQUEST_A::DISABLE,
            true => CTL_AN_FEC_10G_REQUEST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_FEC_10G_REQUEST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_FEC_10G_REQUEST_A::ENABLE
    }
}
#[doc = "Field `ctl_an_fec_10g_request` writer - "]
pub type CTL_AN_FEC_10G_REQUEST_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR2_SPEC, O, CTL_AN_FEC_10G_REQUEST_A>;
impl<'a, const O: u8> CTL_AN_FEC_10G_REQUEST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_FEC_10G_REQUEST_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_FEC_10G_REQUEST_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_fec_ability_override` reader - "]
pub type CTL_AN_FEC_ABILITY_OVERRIDE_R = crate::BitReader<CTL_AN_FEC_ABILITY_OVERRIDE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_FEC_ABILITY_OVERRIDE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_FEC_ABILITY_OVERRIDE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_FEC_ABILITY_OVERRIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_FEC_ABILITY_OVERRIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_FEC_ABILITY_OVERRIDE_A {
        match self.bits {
            false => CTL_AN_FEC_ABILITY_OVERRIDE_A::DISABLE,
            true => CTL_AN_FEC_ABILITY_OVERRIDE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_FEC_ABILITY_OVERRIDE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_FEC_ABILITY_OVERRIDE_A::ENABLE
    }
}
#[doc = "Field `ctl_an_fec_ability_override` writer - "]
pub type CTL_AN_FEC_ABILITY_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR2_SPEC, O, CTL_AN_FEC_ABILITY_OVERRIDE_A>;
impl<'a, const O: u8> CTL_AN_FEC_ABILITY_OVERRIDE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_FEC_ABILITY_OVERRIDE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_FEC_ABILITY_OVERRIDE_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_cl91_fec_request` reader - "]
pub type CTL_AN_CL91_FEC_REQUEST_R = crate::BitReader<CTL_AN_CL91_FEC_REQUEST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_CL91_FEC_REQUEST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_CL91_FEC_REQUEST_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_CL91_FEC_REQUEST_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_CL91_FEC_REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_CL91_FEC_REQUEST_A {
        match self.bits {
            false => CTL_AN_CL91_FEC_REQUEST_A::DISABLE,
            true => CTL_AN_CL91_FEC_REQUEST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_CL91_FEC_REQUEST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_CL91_FEC_REQUEST_A::ENABLE
    }
}
#[doc = "Field `ctl_an_cl91_fec_request` writer - "]
pub type CTL_AN_CL91_FEC_REQUEST_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR2_SPEC, O, CTL_AN_CL91_FEC_REQUEST_A>;
impl<'a, const O: u8> CTL_AN_CL91_FEC_REQUEST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_CL91_FEC_REQUEST_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_CL91_FEC_REQUEST_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_cl91_fec_ability` reader - "]
pub type CTL_AN_CL91_FEC_ABILITY_R = crate::BitReader<CTL_AN_CL91_FEC_ABILITY_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_CL91_FEC_ABILITY_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_CL91_FEC_ABILITY_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_CL91_FEC_ABILITY_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_CL91_FEC_ABILITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_CL91_FEC_ABILITY_A {
        match self.bits {
            false => CTL_AN_CL91_FEC_ABILITY_A::DISABLE,
            true => CTL_AN_CL91_FEC_ABILITY_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_CL91_FEC_ABILITY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_CL91_FEC_ABILITY_A::ENABLE
    }
}
#[doc = "Field `ctl_an_cl91_fec_ability` writer - "]
pub type CTL_AN_CL91_FEC_ABILITY_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR2_SPEC, O, CTL_AN_CL91_FEC_ABILITY_A>;
impl<'a, const O: u8> CTL_AN_CL91_FEC_ABILITY_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_CL91_FEC_ABILITY_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_CL91_FEC_ABILITY_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_fec_25g_rs_request` reader - "]
pub type CTL_AN_FEC_25G_RS_REQUEST_R = crate::BitReader<CTL_AN_FEC_25G_RS_REQUEST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_FEC_25G_RS_REQUEST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_FEC_25G_RS_REQUEST_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_FEC_25G_RS_REQUEST_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_FEC_25G_RS_REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_FEC_25G_RS_REQUEST_A {
        match self.bits {
            false => CTL_AN_FEC_25G_RS_REQUEST_A::DISABLE,
            true => CTL_AN_FEC_25G_RS_REQUEST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_FEC_25G_RS_REQUEST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_FEC_25G_RS_REQUEST_A::ENABLE
    }
}
#[doc = "Field `ctl_an_fec_25g_rs_request` writer - "]
pub type CTL_AN_FEC_25G_RS_REQUEST_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR2_SPEC, O, CTL_AN_FEC_25G_RS_REQUEST_A>;
impl<'a, const O: u8> CTL_AN_FEC_25G_RS_REQUEST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_FEC_25G_RS_REQUEST_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_FEC_25G_RS_REQUEST_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_fec_25g_baser_request` reader - "]
pub type CTL_AN_FEC_25G_BASER_REQUEST_R = crate::BitReader<CTL_AN_FEC_25G_BASER_REQUEST_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_FEC_25G_BASER_REQUEST_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_FEC_25G_BASER_REQUEST_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_FEC_25G_BASER_REQUEST_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_FEC_25G_BASER_REQUEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_FEC_25G_BASER_REQUEST_A {
        match self.bits {
            false => CTL_AN_FEC_25G_BASER_REQUEST_A::DISABLE,
            true => CTL_AN_FEC_25G_BASER_REQUEST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_FEC_25G_BASER_REQUEST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_FEC_25G_BASER_REQUEST_A::ENABLE
    }
}
#[doc = "Field `ctl_an_fec_25g_baser_request` writer - "]
pub type CTL_AN_FEC_25G_BASER_REQUEST_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR2_SPEC, O, CTL_AN_FEC_25G_BASER_REQUEST_A>;
impl<'a, const O: u8> CTL_AN_FEC_25G_BASER_REQUEST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_FEC_25G_BASER_REQUEST_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_FEC_25G_BASER_REQUEST_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_an_pause(&self) -> CTL_AN_PAUSE_R {
        CTL_AN_PAUSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctl_an_asmdir(&self) -> CTL_AN_ASMDIR_R {
        CTL_AN_ASMDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ctl_an_fec_10g_request(&self) -> CTL_AN_FEC_10G_REQUEST_R {
        CTL_AN_FEC_10G_REQUEST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ctl_an_fec_ability_override(&self) -> CTL_AN_FEC_ABILITY_OVERRIDE_R {
        CTL_AN_FEC_ABILITY_OVERRIDE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ctl_an_cl91_fec_request(&self) -> CTL_AN_CL91_FEC_REQUEST_R {
        CTL_AN_CL91_FEC_REQUEST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ctl_an_cl91_fec_ability(&self) -> CTL_AN_CL91_FEC_ABILITY_R {
        CTL_AN_CL91_FEC_ABILITY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ctl_an_fec_25g_rs_request(&self) -> CTL_AN_FEC_25G_RS_REQUEST_R {
        CTL_AN_FEC_25G_RS_REQUEST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ctl_an_fec_25g_baser_request(&self) -> CTL_AN_FEC_25G_BASER_REQUEST_R {
        CTL_AN_FEC_25G_BASER_REQUEST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_pause(&mut self) -> CTL_AN_PAUSE_W<0> {
        CTL_AN_PAUSE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_asmdir(&mut self) -> CTL_AN_ASMDIR_W<1> {
        CTL_AN_ASMDIR_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_fec_10g_request(&mut self) -> CTL_AN_FEC_10G_REQUEST_W<16> {
        CTL_AN_FEC_10G_REQUEST_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_fec_ability_override(&mut self) -> CTL_AN_FEC_ABILITY_OVERRIDE_W<17> {
        CTL_AN_FEC_ABILITY_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_cl91_fec_request(&mut self) -> CTL_AN_CL91_FEC_REQUEST_W<18> {
        CTL_AN_CL91_FEC_REQUEST_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_cl91_fec_ability(&mut self) -> CTL_AN_CL91_FEC_ABILITY_W<19> {
        CTL_AN_CL91_FEC_ABILITY_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_fec_25g_rs_request(&mut self) -> CTL_AN_FEC_25G_RS_REQUEST_W<20> {
        CTL_AN_FEC_25G_RS_REQUEST_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_fec_25g_baser_request(&mut self) -> CTL_AN_FEC_25G_BASER_REQUEST_W<21> {
        CTL_AN_FEC_25G_BASER_REQUEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AN_CONTROL2 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ancr2](index.html) module"]
pub struct ANCR2_SPEC;
impl crate::RegisterSpec for ANCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ancr2::R](R) reader structure"]
impl crate::Readable for ANCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ancr2::W](W) writer structure"]
impl crate::Writable for ANCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ancr2 to value 0"]
impl crate::Resettable for ANCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

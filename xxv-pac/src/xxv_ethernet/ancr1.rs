#[doc = "Register `ancr1` reader"]
pub struct R(crate::R<ANCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ancr1` writer"]
pub struct W(crate::W<ANCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANCR1_SPEC>;
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
impl From<crate::W<ANCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_autoneg_enable` reader - "]
pub type CTL_AUTONEG_ENABLE_R = crate::BitReader<CTL_AUTONEG_ENABLE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AUTONEG_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AUTONEG_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AUTONEG_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AUTONEG_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AUTONEG_ENABLE_A {
        match self.bits {
            false => CTL_AUTONEG_ENABLE_A::DISABLE,
            true => CTL_AUTONEG_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AUTONEG_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AUTONEG_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ctl_autoneg_enable` writer - "]
pub type CTL_AUTONEG_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR1_SPEC, O, CTL_AUTONEG_ENABLE_A>;
impl<'a, const O: u8> CTL_AUTONEG_ENABLE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AUTONEG_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AUTONEG_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ctl_autoneg_bypass` reader - "]
pub type CTL_AUTONEG_BYPASS_R = crate::BitReader<CTL_AUTONEG_BYPASS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AUTONEG_BYPASS_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AUTONEG_BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AUTONEG_BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AUTONEG_BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AUTONEG_BYPASS_A {
        match self.bits {
            false => CTL_AUTONEG_BYPASS_A::DISABLE,
            true => CTL_AUTONEG_BYPASS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AUTONEG_BYPASS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AUTONEG_BYPASS_A::ENABLE
    }
}
#[doc = "Field `ctl_autoneg_bypass` writer - "]
pub type CTL_AUTONEG_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR1_SPEC, O, CTL_AUTONEG_BYPASS_A>;
impl<'a, const O: u8> CTL_AUTONEG_BYPASS_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AUTONEG_BYPASS_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AUTONEG_BYPASS_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_nonce_seed` reader - "]
pub type CTL_AN_NONCE_SEED_R = crate::FieldReader;
#[doc = "Field `ctl_an_nonce_seed` writer - "]
pub type CTL_AN_NONCE_SEED_W<'a, const O: u8> = crate::FieldWriter<'a, ANCR1_SPEC, 8, O>;
#[doc = "Field `ctl_an_pseudo_sel` reader - "]
pub type CTL_AN_PSEUDO_SEL_R = crate::BitReader<CTL_AN_PSEUDO_SEL_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_PSEUDO_SEL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_PSEUDO_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_PSEUDO_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_PSEUDO_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_PSEUDO_SEL_A {
        match self.bits {
            false => CTL_AN_PSEUDO_SEL_A::DISABLE,
            true => CTL_AN_PSEUDO_SEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_PSEUDO_SEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_PSEUDO_SEL_A::ENABLE
    }
}
#[doc = "Field `ctl_an_pseudo_sel` writer - "]
pub type CTL_AN_PSEUDO_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR1_SPEC, O, CTL_AN_PSEUDO_SEL_A>;
impl<'a, const O: u8> CTL_AN_PSEUDO_SEL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_PSEUDO_SEL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_PSEUDO_SEL_A::ENABLE)
    }
}
#[doc = "Field `ctl_restart_negotiation` reader - "]
pub type CTL_RESTART_NEGOTIATION_R = crate::BitReader<CTL_RESTART_NEGOTIATION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_RESTART_NEGOTIATION_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_RESTART_NEGOTIATION_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_RESTART_NEGOTIATION_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_RESTART_NEGOTIATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_RESTART_NEGOTIATION_A {
        match self.bits {
            false => CTL_RESTART_NEGOTIATION_A::DISABLE,
            true => CTL_RESTART_NEGOTIATION_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_RESTART_NEGOTIATION_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_RESTART_NEGOTIATION_A::ENABLE
    }
}
#[doc = "Field `ctl_restart_negotiation` writer - "]
pub type CTL_RESTART_NEGOTIATION_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR1_SPEC, O, CTL_RESTART_NEGOTIATION_A>;
impl<'a, const O: u8> CTL_RESTART_NEGOTIATION_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_RESTART_NEGOTIATION_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_RESTART_NEGOTIATION_A::ENABLE)
    }
}
#[doc = "Field `ctl_an_local_fault` reader - "]
pub type CTL_AN_LOCAL_FAULT_R = crate::BitReader<CTL_AN_LOCAL_FAULT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_AN_LOCAL_FAULT_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_AN_LOCAL_FAULT_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_AN_LOCAL_FAULT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_AN_LOCAL_FAULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_AN_LOCAL_FAULT_A {
        match self.bits {
            false => CTL_AN_LOCAL_FAULT_A::DISABLE,
            true => CTL_AN_LOCAL_FAULT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_AN_LOCAL_FAULT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_AN_LOCAL_FAULT_A::ENABLE
    }
}
#[doc = "Field `ctl_an_local_fault` writer - "]
pub type CTL_AN_LOCAL_FAULT_W<'a, const O: u8> =
    crate::BitWriter<'a, ANCR1_SPEC, O, CTL_AN_LOCAL_FAULT_A>;
impl<'a, const O: u8> CTL_AN_LOCAL_FAULT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_AN_LOCAL_FAULT_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_AN_LOCAL_FAULT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_autoneg_enable(&self) -> CTL_AUTONEG_ENABLE_R {
        CTL_AUTONEG_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctl_autoneg_bypass(&self) -> CTL_AUTONEG_BYPASS_R {
        CTL_AUTONEG_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9"]
    #[inline(always)]
    pub fn ctl_an_nonce_seed(&self) -> CTL_AN_NONCE_SEED_R {
        CTL_AN_NONCE_SEED_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ctl_an_pseudo_sel(&self) -> CTL_AN_PSEUDO_SEL_R {
        CTL_AN_PSEUDO_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ctl_restart_negotiation(&self) -> CTL_RESTART_NEGOTIATION_R {
        CTL_RESTART_NEGOTIATION_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ctl_an_local_fault(&self) -> CTL_AN_LOCAL_FAULT_R {
        CTL_AN_LOCAL_FAULT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_autoneg_enable(&mut self) -> CTL_AUTONEG_ENABLE_W<0> {
        CTL_AUTONEG_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_autoneg_bypass(&mut self) -> CTL_AUTONEG_BYPASS_W<1> {
        CTL_AUTONEG_BYPASS_W::new(self)
    }
    #[doc = "Bits 2:9"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_nonce_seed(&mut self) -> CTL_AN_NONCE_SEED_W<2> {
        CTL_AN_NONCE_SEED_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_pseudo_sel(&mut self) -> CTL_AN_PSEUDO_SEL_W<10> {
        CTL_AN_PSEUDO_SEL_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_restart_negotiation(&mut self) -> CTL_RESTART_NEGOTIATION_W<11> {
        CTL_RESTART_NEGOTIATION_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_an_local_fault(&mut self) -> CTL_AN_LOCAL_FAULT_W<12> {
        CTL_AN_LOCAL_FAULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AN_CONTROL1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ancr1](index.html) module"]
pub struct ANCR1_SPEC;
impl crate::RegisterSpec for ANCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ancr1::R](R) reader structure"]
impl crate::Readable for ANCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ancr1::W](W) writer structure"]
impl crate::Writable for ANCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ancr1 to value 0"]
impl crate::Resettable for ANCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

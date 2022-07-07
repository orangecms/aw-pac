#[doc = "Register `TVD_BGR` reader"]
pub struct R(crate::R<TVD_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVD_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVD_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVD_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TVD_BGR` writer"]
pub struct W(crate::W<TVD_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVD_BGR_SPEC>;
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
impl From<crate::W<TVD_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVD_BGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Reset"]
pub type RST_R = crate::BitReader<RST_A>;
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::ASSERT,
            true => RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == RST_A::DEASSERT
    }
}
#[doc = "Field `RST` writer - Reset"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TVD_BGR_SPEC, RST_A, O>;
impl<'a, const O: u8> RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(RST_A::DEASSERT)
    }
}
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOP_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<TOP_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TOP_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOP_RST` reader - Reset"]
pub type TOP_RST_R = crate::BitReader<TOP_RST_A>;
impl TOP_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOP_RST_A {
        match self.bits {
            false => TOP_RST_A::ASSERT,
            true => TOP_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == TOP_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == TOP_RST_A::DEASSERT
    }
}
#[doc = "Field `TOP_RST` writer - Reset"]
pub type TOP_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TVD_BGR_SPEC, TOP_RST_A, O>;
impl<'a, const O: u8> TOP_RST_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(TOP_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(TOP_RST_A::DEASSERT)
    }
}
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<GATING_A> for bool {
    #[inline(always)]
    fn from(variant: GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GATING` reader - Gating Clock"]
pub type GATING_R = crate::BitReader<GATING_A>;
impl GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::MASK,
            true => GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == GATING_A::PASS
    }
}
#[doc = "Field `GATING` writer - Gating Clock"]
pub type GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, TVD_BGR_SPEC, GATING_A, O>;
impl<'a, const O: u8> GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(GATING_A::PASS)
    }
}
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOP_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<TOP_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: TOP_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOP_GATING` reader - Gating Clock"]
pub type TOP_GATING_R = crate::BitReader<TOP_GATING_A>;
impl TOP_GATING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOP_GATING_A {
        match self.bits {
            false => TOP_GATING_A::MASK,
            true => TOP_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == TOP_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == TOP_GATING_A::PASS
    }
}
#[doc = "Field `TOP_GATING` writer - Gating Clock"]
pub type TOP_GATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, TVD_BGR_SPEC, TOP_GATING_A, O>;
impl<'a, const O: u8> TOP_GATING_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(TOP_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(TOP_GATING_A::PASS)
    }
}
impl R {
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn top_rst(&self) -> TOP_RST_R {
        TOP_RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn top_gating(&self) -> TOP_GATING_R {
        TOP_GATING_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<17> {
        RST_W::new(self)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn top_rst(&mut self) -> TOP_RST_W<16> {
        TOP_RST_W::new(self)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn gating(&mut self) -> GATING_W<1> {
        GATING_W::new(self)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn top_gating(&mut self) -> TOP_GATING_W<0> {
        TOP_GATING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TVD Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tvd_bgr](index.html) module"]
pub struct TVD_BGR_SPEC;
impl crate::RegisterSpec for TVD_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tvd_bgr::R](R) reader structure"]
impl crate::Readable for TVD_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tvd_bgr::W](W) writer structure"]
impl crate::Writable for TVD_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TVD_BGR to value 0"]
impl crate::Resettable for TVD_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

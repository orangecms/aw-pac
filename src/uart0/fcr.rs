#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT_AW {
    #[doc = "0: `0`"]
    ONE_CHARACTER = 0,
    #[doc = "1: `1`"]
    QUARTER_FULL = 1,
    #[doc = "2: `10`"]
    HALF_FULL = 2,
    #[doc = "3: `11`"]
    TWO_LESS_THAN_FULL = 3,
}
impl From<RT_AW> for u8 {
    #[inline(always)]
    fn from(variant: RT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `rt` writer - "]
pub struct RT_W<'a> {
    w: &'a mut W,
}
impl<'a> RT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT_AW) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn one_character(self) -> &'a mut W {
        self.variant(RT_AW::ONE_CHARACTER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn quarter_full(self) -> &'a mut W {
        self.variant(RT_AW::QUARTER_FULL)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut W {
        self.variant(RT_AW::HALF_FULL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn two_less_than_full(self) -> &'a mut W {
        self.variant(RT_AW::TWO_LESS_THAN_FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TFT_AW {
    #[doc = "0: `0`"]
    EMPTY = 0,
    #[doc = "1: `1`"]
    TWO_CHARACTERS = 1,
    #[doc = "2: `10`"]
    QUARTER_FULL = 2,
    #[doc = "3: `11`"]
    HALF_FULL = 3,
}
impl From<TFT_AW> for u8 {
    #[inline(always)]
    fn from(variant: TFT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `tft` writer - "]
pub struct TFT_W<'a> {
    w: &'a mut W,
}
impl<'a> TFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFT_AW) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TFT_AW::EMPTY)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn two_characters(self) -> &'a mut W {
        self.variant(TFT_AW::TWO_CHARACTERS)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn quarter_full(self) -> &'a mut W {
        self.variant(TFT_AW::QUARTER_FULL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn half_full(self) -> &'a mut W {
        self.variant(TFT_AW::HALF_FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAM_AW {
    #[doc = "0: `0`"]
    MODE_0 = 0,
    #[doc = "1: `1`"]
    MODE_1 = 1,
}
impl From<DMAM_AW> for bool {
    #[inline(always)]
    fn from(variant: DMAM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dmam` writer - "]
pub struct DMAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(DMAM_AW::MODE_0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(DMAM_AW::MODE_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `xfifor` writer - "]
pub struct XFIFOR_W<'a> {
    w: &'a mut W,
}
impl<'a> XFIFOR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `rfifor` writer - "]
pub struct RFIFOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIFOR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `fifoe` writer - "]
pub struct FIFOE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn rt(&mut self) -> RT_W {
        RT_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tft(&mut self) -> TFT_W {
        TFT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dmam(&mut self) -> DMAM_W {
        DMAM_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn xfifor(&mut self) -> XFIFOR_W {
        XFIFOR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfifor(&mut self) -> RFIFOR_W {
        RFIFOR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifoe(&mut self) -> FIFOE_W {
        FIFOE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

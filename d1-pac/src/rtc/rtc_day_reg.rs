#[doc = "Register `RTC_DAY_REG` reader"]
pub struct R(crate::R<RTC_DAY_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_DAY_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_DAY_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_DAY_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_DAY_REG` writer"]
pub struct W(crate::W<RTC_DAY_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_DAY_REG_SPEC>;
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
impl From<crate::W<RTC_DAY_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_DAY_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAY` reader - Set Day Range from 0 to 65535."]
pub type DAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DAY` writer - Set Day Range from 0 to 65535."]
pub type DAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTC_DAY_REG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Set Day Range from 0 to 65535."]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set Day Range from 0 to 65535."]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W<0> {
        DAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Year-Month-Day Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_day_reg](index.html) module"]
pub struct RTC_DAY_REG_SPEC;
impl crate::RegisterSpec for RTC_DAY_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_day_reg::R](R) reader structure"]
impl crate::Readable for RTC_DAY_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_day_reg::W](W) writer structure"]
impl crate::Writable for RTC_DAY_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_DAY_REG to value 0"]
impl crate::Resettable for RTC_DAY_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

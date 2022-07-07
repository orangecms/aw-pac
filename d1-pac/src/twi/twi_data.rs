#[doc = "Register `TWI_DATA` reader"]
pub struct R(crate::R<TWI_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWI_DATA` writer"]
pub struct W(crate::W<TWI_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DATA_SPEC>;
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
impl From<crate::W<TWI_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data` reader - Data byte transmitted or received"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `data` writer - Data byte transmitted or received"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TWI_DATA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data byte transmitted or received"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte transmitted or received"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Data Byte Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_data](index.html) module"]
pub struct TWI_DATA_SPEC;
impl crate::RegisterSpec for TWI_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_data::R](R) reader structure"]
impl crate::Readable for TWI_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_data::W](W) writer structure"]
impl crate::Writable for TWI_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_DATA to value 0"]
impl crate::Resettable for TWI_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

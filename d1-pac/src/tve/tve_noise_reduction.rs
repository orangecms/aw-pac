#[doc = "Register `tve_noise_reduction` reader"]
pub struct R(crate::R<TVE_NOISE_REDUCTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_NOISE_REDUCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_NOISE_REDUCTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_NOISE_REDUCTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_noise_reduction` writer"]
pub struct W(crate::W<TVE_NOISE_REDUCTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_NOISE_REDUCTION_SPEC>;
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
impl From<crate::W<TVE_NOISE_REDUCTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_NOISE_REDUCTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `en` reader - "]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `en` writer - "]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TVE_NOISE_REDUCTION_SPEC, bool, O>;
#[doc = "Field `t_value` reader - "]
pub type T_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `t_value` writer - "]
pub type T_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_NOISE_REDUCTION_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn t_value(&self) -> T_VALUE_R {
        T_VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn t_value(&mut self) -> T_VALUE_W<16> {
        T_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Noise Reduction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_noise_reduction](index.html) module"]
pub struct TVE_NOISE_REDUCTION_SPEC;
impl crate::RegisterSpec for TVE_NOISE_REDUCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_noise_reduction::R](R) reader structure"]
impl crate::Readable for TVE_NOISE_REDUCTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_noise_reduction::W](W) writer structure"]
impl crate::Writable for TVE_NOISE_REDUCTION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tve_noise_reduction to value 0"]
impl crate::Resettable for TVE_NOISE_REDUCTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
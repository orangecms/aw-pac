#[doc = "Register `dmic_sr` reader"]
pub struct R(crate::R<DMIC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMIC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMIC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMIC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dmic_sr` writer"]
pub struct W(crate::W<DMIC_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMIC_SR_SPEC>;
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
impl From<crate::W<DMIC_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMIC_SR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMIC Sample Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmic_sr](index.html) module"]
pub struct DMIC_SR_SPEC;
impl crate::RegisterSpec for DMIC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmic_sr::R](R) reader structure"]
impl crate::Readable for DMIC_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmic_sr::W](W) writer structure"]
impl crate::Writable for DMIC_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dmic_sr to value 0"]
impl crate::Resettable for DMIC_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

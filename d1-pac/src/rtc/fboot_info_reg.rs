#[doc = "Register `FBOOT_INFO_REG%s` reader"]
pub struct R(crate::R<FBOOT_INFO_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBOOT_INFO_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBOOT_INFO_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBOOT_INFO_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBOOT_INFO_REG%s` writer"]
pub struct W(crate::W<FBOOT_INFO_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBOOT_INFO_REG_SPEC>;
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
impl From<crate::W<FBOOT_INFO_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBOOT_INFO_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBOOT_INFO` reader - Fast Boot Information \\[i\\], refer to BROM spec."]
pub type FBOOT_INFO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FBOOT_INFO` writer - Fast Boot Information \\[i\\], refer to BROM spec."]
pub type FBOOT_INFO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FBOOT_INFO_REG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Fast Boot Information \\[i\\], refer to BROM spec."]
    #[inline(always)]
    pub fn fboot_info(&self) -> FBOOT_INFO_R {
        FBOOT_INFO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fast Boot Information \\[i\\], refer to BROM spec."]
    #[inline(always)]
    pub fn fboot_info(&mut self) -> FBOOT_INFO_W<0> {
        FBOOT_INFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Boot Information Register \\[01\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fboot_info_reg](index.html) module"]
pub struct FBOOT_INFO_REG_SPEC;
impl crate::RegisterSpec for FBOOT_INFO_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fboot_info_reg::R](R) reader structure"]
impl crate::Readable for FBOOT_INFO_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fboot_info_reg::W](W) writer structure"]
impl crate::Writable for FBOOT_INFO_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBOOT_INFO_REG%s to value 0"]
impl crate::Resettable for FBOOT_INFO_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

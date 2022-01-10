#[doc = "Register `THS_ALARM_INTC` reader"]
pub struct R(crate::R<THS_ALARM_INTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_ALARM_INTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_ALARM_INTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_ALARM_INTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THS_ALARM_INTC` writer"]
pub struct W(crate::W<THS_ALARM_INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THS_ALARM_INTC_SPEC>;
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
impl From<crate::W<THS_ALARM_INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THS_ALARM_INTC_SPEC>) -> Self {
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
#[doc = "THS Alarm Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_alarm_intc](index.html) module"]
pub struct THS_ALARM_INTC_SPEC;
impl crate::RegisterSpec for THS_ALARM_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_alarm_intc::R](R) reader structure"]
impl crate::Readable for THS_ALARM_INTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ths_alarm_intc::W](W) writer structure"]
impl crate::Writable for THS_ALARM_INTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THS_ALARM_INTC to value 0"]
impl crate::Resettable for THS_ALARM_INTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
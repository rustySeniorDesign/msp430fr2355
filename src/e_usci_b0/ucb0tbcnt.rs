#[doc = "Reader of register UCB0TBCNT"]
pub type R = crate::R<u16, super::UCB0TBCNT>;
#[doc = "Writer for register UCB0TBCNT"]
pub type W = crate::W<u16, super::UCB0TBCNT>;
#[doc = "Register UCB0TBCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB0TBCNT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Byte counter threshold value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTBCNT_A {}
impl From<UCTBCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: UCTBCNT_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `UCTBCNT`"]
pub type UCTBCNT_R = crate::R<u8, UCTBCNT_A>;
impl UCTBCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UCTBCNT_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `UCTBCNT`"]
pub struct UCTBCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTBCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTBCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&self) -> UCTBCNT_R {
        UCTBCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&mut self) -> UCTBCNT_W {
        UCTBCNT_W { w: self }
    }
}
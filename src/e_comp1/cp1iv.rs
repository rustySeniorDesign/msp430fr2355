#[doc = "Reader of register CP1IV"]
pub type R = crate::R<u16, super::CP1IV>;
#[doc = "Writer for register CP1IV"]
pub type W = crate::W<u16, super::CP1IV>;
#[doc = "Register CP1IV `reset()`'s with value 0"]
impl crate::ResetValue for super::CP1IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparator interrupt vector word register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPIV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: CPIFG"]
    CPIFG,
    #[doc = "4: CPIIFG"]
    CPIIFG,
}
impl From<CPIV_A> for u16 {
    #[inline(always)]
    fn from(variant: CPIV_A) -> Self {
        match variant {
            CPIV_A::NONE => 0,
            CPIV_A::CPIFG => 2,
            CPIV_A::CPIIFG => 4,
        }
    }
}
#[doc = "Reader of field `CPIV`"]
pub type CPIV_R = crate::R<u16, CPIV_A>;
impl CPIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, CPIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CPIV_A::NONE),
            2 => Val(CPIV_A::CPIFG),
            4 => Val(CPIV_A::CPIIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CPIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `CPIFG`"]
    #[inline(always)]
    pub fn is_cpifg(&self) -> bool {
        *self == CPIV_A::CPIFG
    }
    #[doc = "Checks if the value of the field is `CPIIFG`"]
    #[inline(always)]
    pub fn is_cpiifg(&self) -> bool {
        *self == CPIV_A::CPIIFG
    }
}
#[doc = "Write proxy for field `CPIV`"]
pub struct CPIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CPIV_A::NONE)
    }
    #[doc = "CPIFG"]
    #[inline(always)]
    pub fn cpifg(self) -> &'a mut W {
        self.variant(CPIV_A::CPIFG)
    }
    #[doc = "CPIIFG"]
    #[inline(always)]
    pub fn cpiifg(self) -> &'a mut W {
        self.variant(CPIV_A::CPIIFG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn cpiv(&self) -> CPIV_R {
        CPIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn cpiv(&mut self) -> CPIV_W {
        CPIV_W { w: self }
    }
}

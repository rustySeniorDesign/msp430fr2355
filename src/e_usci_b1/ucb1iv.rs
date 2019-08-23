#[doc = "Reader of register UCB1IV"]
pub type R = crate::R<u16, super::UCB1IV>;
#[doc = "Writer for register UCB1IV"]
pub type W = crate::W<u16, super::UCB1IV>;
#[doc = "Register UCB1IV `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB1IV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "eUSCI_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    UCRXIFG,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest"]
    UCTXIFG,
}
impl From<UCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: UCIV_A) -> Self {
        match variant {
            UCIV_A::NONE => 0,
            UCIV_A::UCRXIFG => 2,
            UCIV_A::UCTXIFG => 4,
        }
    }
}
#[doc = "Reader of field `UCIV`"]
pub type UCIV_R = crate::R<u16, UCIV_A>;
impl UCIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, UCIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UCIV_A::NONE),
            2 => Val(UCIV_A::UCRXIFG),
            4 => Val(UCIV_A::UCTXIFG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == UCIV_A::NONE
    }
    #[doc = "Checks if the value of the field is `UCRXIFG`"]
    #[inline(always)]
    pub fn is_ucrxifg(&self) -> bool {
        *self == UCIV_A::UCRXIFG
    }
    #[doc = "Checks if the value of the field is `UCTXIFG`"]
    #[inline(always)]
    pub fn is_uctxifg(&self) -> bool {
        *self == UCIV_A::UCTXIFG
    }
}
#[doc = "Write proxy for field `UCIV`"]
pub struct UCIV_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(UCIV_A::NONE)
    }
    #[doc = "Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn ucrxifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCRXIFG)
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn uctxifg(self) -> &'a mut W {
        self.variant(UCIV_A::UCTXIFG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&mut self) -> UCIV_W {
        UCIV_W { w: self }
    }
}

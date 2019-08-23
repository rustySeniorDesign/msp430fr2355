#[doc = "Reader of register UCB1ADDMASK"]
pub type R = crate::R<u16, super::UCB1ADDMASK>;
#[doc = "Writer for register UCB1ADDMASK"]
pub type W = crate::W<u16, super::UCB1ADDMASK>;
#[doc = "Register UCB1ADDMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::UCB1ADDMASK {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDMASK_A {}
impl From<ADDMASK_A> for u16 {
    #[inline(always)]
    fn from(variant: ADDMASK_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `ADDMASK`"]
pub type ADDMASK_R = crate::R<u16, ADDMASK_A>;
impl ADDMASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, ADDMASK_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `ADDMASK`"]
pub struct ADDMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDMASK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u16) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&self) -> ADDMASK_R {
        ADDMASK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&mut self) -> ADDMASK_W {
        ADDMASK_W { w: self }
    }
}

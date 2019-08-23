#[doc = "Reader of register UCA1IRCTL"]
pub type R = crate::R<u16, super::UCA1IRCTL>;
#[doc = "Writer for register UCA1IRCTL"]
pub type W = crate::W<u16, super::UCA1IRCTL>;
#[doc = "Register UCA1IRCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::UCA1IRCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "IrDA encoder/decoder enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIREN_A {
    #[doc = "0: IrDA encoder/decoder disabled"]
    UCIREN_0,
    #[doc = "1: IrDA encoder/decoder enabled"]
    UCIREN_1,
}
impl From<UCIREN_A> for bool {
    #[inline(always)]
    fn from(variant: UCIREN_A) -> Self {
        match variant {
            UCIREN_A::UCIREN_0 => false,
            UCIREN_A::UCIREN_1 => true,
        }
    }
}
#[doc = "Reader of field `UCIREN`"]
pub type UCIREN_R = crate::R<bool, UCIREN_A>;
impl UCIREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIREN_A {
        match self.bits {
            false => UCIREN_A::UCIREN_0,
            true => UCIREN_A::UCIREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIREN_0`"]
    #[inline(always)]
    pub fn is_uciren_0(&self) -> bool {
        *self == UCIREN_A::UCIREN_0
    }
    #[doc = "Checks if the value of the field is `UCIREN_1`"]
    #[inline(always)]
    pub fn is_uciren_1(&self) -> bool {
        *self == UCIREN_A::UCIREN_1
    }
}
#[doc = "Write proxy for field `UCIREN`"]
pub struct UCIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IrDA encoder/decoder disabled"]
    #[inline(always)]
    pub fn uciren_0(self) -> &'a mut W {
        self.variant(UCIREN_A::UCIREN_0)
    }
    #[doc = "IrDA encoder/decoder enabled"]
    #[inline(always)]
    pub fn uciren_1(self) -> &'a mut W {
        self.variant(UCIREN_A::UCIREN_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "IrDA transmit pulse clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIRTXCLK_A {
    #[doc = "0: BRCLK"]
    UCIRTXCLK_0,
    #[doc = "1: BITCLK16 when UCOS16 = 1. Otherwise, BRCLK."]
    UCIRTXCLK_1,
}
impl From<UCIRTXCLK_A> for bool {
    #[inline(always)]
    fn from(variant: UCIRTXCLK_A) -> Self {
        match variant {
            UCIRTXCLK_A::UCIRTXCLK_0 => false,
            UCIRTXCLK_A::UCIRTXCLK_1 => true,
        }
    }
}
#[doc = "Reader of field `UCIRTXCLK`"]
pub type UCIRTXCLK_R = crate::R<bool, UCIRTXCLK_A>;
impl UCIRTXCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIRTXCLK_A {
        match self.bits {
            false => UCIRTXCLK_A::UCIRTXCLK_0,
            true => UCIRTXCLK_A::UCIRTXCLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIRTXCLK_0`"]
    #[inline(always)]
    pub fn is_ucirtxclk_0(&self) -> bool {
        *self == UCIRTXCLK_A::UCIRTXCLK_0
    }
    #[doc = "Checks if the value of the field is `UCIRTXCLK_1`"]
    #[inline(always)]
    pub fn is_ucirtxclk_1(&self) -> bool {
        *self == UCIRTXCLK_A::UCIRTXCLK_1
    }
}
#[doc = "Write proxy for field `UCIRTXCLK`"]
pub struct UCIRTXCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIRTXCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BRCLK"]
    #[inline(always)]
    pub fn ucirtxclk_0(self) -> &'a mut W {
        self.variant(UCIRTXCLK_A::UCIRTXCLK_0)
    }
    #[doc = "BITCLK16 when UCOS16 = 1. Otherwise, BRCLK."]
    #[inline(always)]
    pub fn ucirtxclk_1(self) -> &'a mut W {
        self.variant(UCIRTXCLK_A::UCIRTXCLK_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Transmit pulse length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIRTXPL_A {}
impl From<UCIRTXPL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCIRTXPL_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `UCIRTXPL`"]
pub type UCIRTXPL_R = crate::R<u8, UCIRTXPL_A>;
impl UCIRTXPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UCIRTXPL_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `UCIRTXPL`"]
pub struct UCIRTXPL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIRTXPL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u16) & 0x3f) << 2);
        self.w
    }
}
#[doc = "IrDA receive filter enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIRRXFE_A {
    #[doc = "0: Receive filter disabled"]
    UCIRRXFE_0,
    #[doc = "1: Receive filter enabled"]
    UCIRRXFE_1,
}
impl From<UCIRRXFE_A> for bool {
    #[inline(always)]
    fn from(variant: UCIRRXFE_A) -> Self {
        match variant {
            UCIRRXFE_A::UCIRRXFE_0 => false,
            UCIRRXFE_A::UCIRRXFE_1 => true,
        }
    }
}
#[doc = "Reader of field `UCIRRXFE`"]
pub type UCIRRXFE_R = crate::R<bool, UCIRRXFE_A>;
impl UCIRRXFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIRRXFE_A {
        match self.bits {
            false => UCIRRXFE_A::UCIRRXFE_0,
            true => UCIRRXFE_A::UCIRRXFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIRRXFE_0`"]
    #[inline(always)]
    pub fn is_ucirrxfe_0(&self) -> bool {
        *self == UCIRRXFE_A::UCIRRXFE_0
    }
    #[doc = "Checks if the value of the field is `UCIRRXFE_1`"]
    #[inline(always)]
    pub fn is_ucirrxfe_1(&self) -> bool {
        *self == UCIRRXFE_A::UCIRRXFE_1
    }
}
#[doc = "Write proxy for field `UCIRRXFE`"]
pub struct UCIRRXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIRRXFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive filter disabled"]
    #[inline(always)]
    pub fn ucirrxfe_0(self) -> &'a mut W {
        self.variant(UCIRRXFE_A::UCIRRXFE_0)
    }
    #[doc = "Receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe_1(self) -> &'a mut W {
        self.variant(UCIRRXFE_A::UCIRRXFE_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "IrDA receive input UCAxRXD polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIRRXPL_A {
    #[doc = "0: IrDA transceiver delivers a high pulse when a light pulse is seen"]
    HIGH,
    #[doc = "1: IrDA transceiver delivers a low pulse when a light pulse is seen"]
    LOW,
}
impl From<UCIRRXPL_A> for bool {
    #[inline(always)]
    fn from(variant: UCIRRXPL_A) -> Self {
        match variant {
            UCIRRXPL_A::HIGH => false,
            UCIRRXPL_A::LOW => true,
        }
    }
}
#[doc = "Reader of field `UCIRRXPL`"]
pub type UCIRRXPL_R = crate::R<bool, UCIRRXPL_A>;
impl UCIRRXPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIRRXPL_A {
        match self.bits {
            false => UCIRRXPL_A::HIGH,
            true => UCIRRXPL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == UCIRRXPL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == UCIRRXPL_A::LOW
    }
}
#[doc = "Write proxy for field `UCIRRXPL`"]
pub struct UCIRRXPL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIRRXPL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IrDA transceiver delivers a high pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(UCIRRXPL_A::HIGH)
    }
    #[doc = "IrDA transceiver delivers a low pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(UCIRRXPL_A::LOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Receive filter length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIRRXFL_A {}
impl From<UCIRRXFL_A> for u8 {
    #[inline(always)]
    fn from(variant: UCIRRXFL_A) -> Self {
        match variant {}
    }
}
#[doc = "Reader of field `UCIRRXFL`"]
pub type UCIRRXFL_R = crate::R<u8, UCIRRXFL_A>;
impl UCIRRXFL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UCIRRXFL_A> {
        use crate::Variant::*;
        match self.bits {
            i => Res(i),
        }
    }
}
#[doc = "Write proxy for field `UCIRRXFL`"]
pub struct UCIRRXFL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIRRXFL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u16) & 0x3f) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UCIREN_R {
        UCIREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UCIRTXCLK_R {
        UCIRTXCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&self) -> UCIRTXPL_R {
        UCIRTXPL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - IrDA receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:15 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl(&self) -> UCIRRXFL_R {
        UCIRRXFL_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UCIREN_W {
        UCIREN_W { w: self }
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UCIRTXCLK_W {
        UCIRTXCLK_W { w: self }
    }
    #[doc = "Bits 2:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&mut self) -> UCIRTXPL_W {
        UCIRTXPL_W { w: self }
    }
    #[doc = "Bit 8 - IrDA receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W {
        UCIRRXFE_W { w: self }
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W {
        UCIRRXPL_W { w: self }
    }
    #[doc = "Bits 10:15 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl(&mut self) -> UCIRRXFL_W {
        UCIRRXFL_W { w: self }
    }
}

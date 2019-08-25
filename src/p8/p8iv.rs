#[doc = "Reader of register P8IV"]
pub type R = crate::R<u16, super::P8IV>;
#[doc = "Port 8 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P8IV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Interrupt Source: Port 8.0 interrupt; Interrupt Flag: P8IFG0; Interrupt Priority: Highest"]
    P8IFG0,
    #[doc = "4: Interrupt Source: Port 8.1 interrupt; Interrupt Flag: P8IFG1"]
    P8IFG1,
    #[doc = "6: Interrupt Source: Port 8.2 interrupt; Interrupt Flag: P8IFG2"]
    P8IFG2,
    #[doc = "8: Interrupt Source: Port 8.3 interrupt; Interrupt Flag: P8IFG3"]
    P8IFG3,
    #[doc = "10: Interrupt Source: Port 8.4 interrupt; Interrupt Flag: P8IFG4"]
    P8IFG4,
    #[doc = "12: Interrupt Source: Port 8.5 interrupt; Interrupt Flag: P8IFG5"]
    P8IFG5,
    #[doc = "14: Interrupt Source: Port 8.6 interrupt; Interrupt Flag: P8IFG6"]
    P8IFG6,
    #[doc = "16: Interrupt Source: Port 8.7 interrupt; Interrupt Flag: P8IFG7; Interrupt Priority: Lowest"]
    P8IFG7,
}
impl From<P8IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P8IV_A) -> Self {
        match variant {
            P8IV_A::NONE => 0,
            P8IV_A::P8IFG0 => 2,
            P8IV_A::P8IFG1 => 4,
            P8IV_A::P8IFG2 => 6,
            P8IV_A::P8IFG3 => 8,
            P8IV_A::P8IFG4 => 10,
            P8IV_A::P8IFG5 => 12,
            P8IV_A::P8IFG6 => 14,
            P8IV_A::P8IFG7 => 16,
        }
    }
}
#[doc = "Reader of field `P8IV`"]
pub type P8IV_R = crate::R<u8, P8IV_A>;
impl P8IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P8IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P8IV_A::NONE),
            2 => Val(P8IV_A::P8IFG0),
            4 => Val(P8IV_A::P8IFG1),
            6 => Val(P8IV_A::P8IFG2),
            8 => Val(P8IV_A::P8IFG3),
            10 => Val(P8IV_A::P8IFG4),
            12 => Val(P8IV_A::P8IFG5),
            14 => Val(P8IV_A::P8IFG6),
            16 => Val(P8IV_A::P8IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P8IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P8IFG0`"]
    #[inline(always)]
    pub fn is_p8ifg0(&self) -> bool {
        *self == P8IV_A::P8IFG0
    }
    #[doc = "Checks if the value of the field is `P8IFG1`"]
    #[inline(always)]
    pub fn is_p8ifg1(&self) -> bool {
        *self == P8IV_A::P8IFG1
    }
    #[doc = "Checks if the value of the field is `P8IFG2`"]
    #[inline(always)]
    pub fn is_p8ifg2(&self) -> bool {
        *self == P8IV_A::P8IFG2
    }
    #[doc = "Checks if the value of the field is `P8IFG3`"]
    #[inline(always)]
    pub fn is_p8ifg3(&self) -> bool {
        *self == P8IV_A::P8IFG3
    }
    #[doc = "Checks if the value of the field is `P8IFG4`"]
    #[inline(always)]
    pub fn is_p8ifg4(&self) -> bool {
        *self == P8IV_A::P8IFG4
    }
    #[doc = "Checks if the value of the field is `P8IFG5`"]
    #[inline(always)]
    pub fn is_p8ifg5(&self) -> bool {
        *self == P8IV_A::P8IFG5
    }
    #[doc = "Checks if the value of the field is `P8IFG6`"]
    #[inline(always)]
    pub fn is_p8ifg6(&self) -> bool {
        *self == P8IV_A::P8IFG6
    }
    #[doc = "Checks if the value of the field is `P8IFG7`"]
    #[inline(always)]
    pub fn is_p8ifg7(&self) -> bool {
        *self == P8IV_A::P8IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 8 interrupt vector value"]
    #[inline(always)]
    pub fn p8iv(&self) -> P8IV_R {
        P8IV_R::new((self.bits & 0x1f) as u8)
    }
}

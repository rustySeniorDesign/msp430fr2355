#[doc = "Reader of register P9IV"]
pub type R = crate::R<u16, super::P9IV>;
#[doc = "Port 9 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P9IV_A {
    #[doc = "0: No interrupt pending"]
    NONE,
    #[doc = "2: Interrupt Source: Port 9.0 interrupt; Interrupt Flag: P9IFG0; Interrupt Priority: Highest"]
    P9IFG0,
    #[doc = "4: Interrupt Source: Port 9.1 interrupt; Interrupt Flag: P9IFG1"]
    P9IFG1,
    #[doc = "6: Interrupt Source: Port 9.2 interrupt; Interrupt Flag: P9IFG2"]
    P9IFG2,
    #[doc = "8: Interrupt Source: Port 9.3 interrupt; Interrupt Flag: P9IFG3"]
    P9IFG3,
    #[doc = "10: Interrupt Source: Port 9.4 interrupt; Interrupt Flag: P9IFG4"]
    P9IFG4,
    #[doc = "12: Interrupt Source: Port 9.5 interrupt; Interrupt Flag: P9IFG5"]
    P9IFG5,
    #[doc = "14: Interrupt Source: Port 9.6 interrupt; Interrupt Flag: P9IFG6"]
    P9IFG6,
    #[doc = "16: Interrupt Source: Port 9.7 interrupt; Interrupt Flag: P9IFG7; Interrupt Priority: Lowest"]
    P9IFG7,
}
impl From<P9IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P9IV_A) -> Self {
        match variant {
            P9IV_A::NONE => 0,
            P9IV_A::P9IFG0 => 2,
            P9IV_A::P9IFG1 => 4,
            P9IV_A::P9IFG2 => 6,
            P9IV_A::P9IFG3 => 8,
            P9IV_A::P9IFG4 => 10,
            P9IV_A::P9IFG5 => 12,
            P9IV_A::P9IFG6 => 14,
            P9IV_A::P9IFG7 => 16,
        }
    }
}
#[doc = "Reader of field `P9IV`"]
pub type P9IV_R = crate::R<u8, P9IV_A>;
impl P9IV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, P9IV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(P9IV_A::NONE),
            2 => Val(P9IV_A::P9IFG0),
            4 => Val(P9IV_A::P9IFG1),
            6 => Val(P9IV_A::P9IFG2),
            8 => Val(P9IV_A::P9IFG3),
            10 => Val(P9IV_A::P9IFG4),
            12 => Val(P9IV_A::P9IFG5),
            14 => Val(P9IV_A::P9IFG6),
            16 => Val(P9IV_A::P9IFG7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == P9IV_A::NONE
    }
    #[doc = "Checks if the value of the field is `P9IFG0`"]
    #[inline(always)]
    pub fn is_p9ifg0(&self) -> bool {
        *self == P9IV_A::P9IFG0
    }
    #[doc = "Checks if the value of the field is `P9IFG1`"]
    #[inline(always)]
    pub fn is_p9ifg1(&self) -> bool {
        *self == P9IV_A::P9IFG1
    }
    #[doc = "Checks if the value of the field is `P9IFG2`"]
    #[inline(always)]
    pub fn is_p9ifg2(&self) -> bool {
        *self == P9IV_A::P9IFG2
    }
    #[doc = "Checks if the value of the field is `P9IFG3`"]
    #[inline(always)]
    pub fn is_p9ifg3(&self) -> bool {
        *self == P9IV_A::P9IFG3
    }
    #[doc = "Checks if the value of the field is `P9IFG4`"]
    #[inline(always)]
    pub fn is_p9ifg4(&self) -> bool {
        *self == P9IV_A::P9IFG4
    }
    #[doc = "Checks if the value of the field is `P9IFG5`"]
    #[inline(always)]
    pub fn is_p9ifg5(&self) -> bool {
        *self == P9IV_A::P9IFG5
    }
    #[doc = "Checks if the value of the field is `P9IFG6`"]
    #[inline(always)]
    pub fn is_p9ifg6(&self) -> bool {
        *self == P9IV_A::P9IFG6
    }
    #[doc = "Checks if the value of the field is `P9IFG7`"]
    #[inline(always)]
    pub fn is_p9ifg7(&self) -> bool {
        *self == P9IV_A::P9IFG7
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 9 interrupt vector value"]
    #[inline(always)]
    pub fn p9iv(&self) -> P9IV_R {
        P9IV_R::new((self.bits & 0x1f) as u8)
    }
}

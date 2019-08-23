#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Modulation Control Word Register"]
    pub uca1mctlw: UCA1MCTLW,
    _reserved1: [u8; 4usize],
    _reserved_1_uca1: [u8; 2usize],
    #[doc = "0x08 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub uca1abctl: UCA1ABCTL,
    #[doc = "0x0a - eUSCI_Ax IrDA Control Word Register"]
    pub uca1irctl: UCA1IRCTL,
    _reserved4: [u8; 10usize],
    _reserved_4_uca1: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x06 - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca1txbuf(&self) -> &UCA1TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA1TXBUF) }
    }
    #[doc = "0x06 - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca1txbuf_mut(&self) -> &mut UCA1TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA1TXBUF) }
    }
    #[doc = "0x16 - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca1iv(&self) -> &UCA1IV {
        unsafe { &*(((self as *const Self) as *const u8).add(22usize) as *const UCA1IV) }
    }
    #[doc = "0x16 - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca1iv_mut(&self) -> &mut UCA1IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(22usize) as *mut UCA1IV) }
    }
}
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca1mctlw](uca1mctlw) module"]
pub type UCA1MCTLW = crate::Reg<u16, _UCA1MCTLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1MCTLW;
#[doc = "`read()` method returns [uca1mctlw::R](uca1mctlw::R) reader structure"]
impl crate::Readable for UCA1MCTLW {}
#[doc = "`write(|w| ..)` method takes [uca1mctlw::W](uca1mctlw::W) writer structure"]
impl crate::Writable for UCA1MCTLW {}
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca1mctlw;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca1txbuf](uca1txbuf) module"]
pub type UCA1TXBUF = crate::Reg<u16, _UCA1TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1TXBUF;
#[doc = "`read()` method returns [uca1txbuf::R](uca1txbuf::R) reader structure"]
impl crate::Readable for UCA1TXBUF {}
#[doc = "`write(|w| ..)` method takes [uca1txbuf::W](uca1txbuf::W) writer structure"]
impl crate::Writable for UCA1TXBUF {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca1txbuf;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca1abctl](uca1abctl) module"]
pub type UCA1ABCTL = crate::Reg<u16, _UCA1ABCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1ABCTL;
#[doc = "`read()` method returns [uca1abctl::R](uca1abctl::R) reader structure"]
impl crate::Readable for UCA1ABCTL {}
#[doc = "`write(|w| ..)` method takes [uca1abctl::W](uca1abctl::W) writer structure"]
impl crate::Writable for UCA1ABCTL {}
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca1abctl;
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca1irctl](uca1irctl) module"]
pub type UCA1IRCTL = crate::Reg<u16, _UCA1IRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IRCTL;
#[doc = "`read()` method returns [uca1irctl::R](uca1irctl::R) reader structure"]
impl crate::Readable for UCA1IRCTL {}
#[doc = "`write(|w| ..)` method takes [uca1irctl::W](uca1irctl::W) writer structure"]
impl crate::Writable for UCA1IRCTL {}
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca1irctl;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca1iv](uca1iv) module"]
pub type UCA1IV = crate::Reg<u16, _UCA1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA1IV;
#[doc = "`read()` method returns [uca1iv::R](uca1iv::R) reader structure"]
impl crate::Readable for UCA1IV {}
#[doc = "`write(|w| ..)` method takes [uca1iv::W](uca1iv::W) writer structure"]
impl crate::Writable for UCA1IV {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca1iv;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Modulation Control Word Register"]
    pub uca0mctlw: UCA0MCTLW,
    _reserved1: [u8; 4usize],
    _reserved_1_uca0: [u8; 2usize],
    #[doc = "0x08 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub uca0abctl: UCA0ABCTL,
    #[doc = "0x0a - eUSCI_Ax IrDA Control Word Register"]
    pub uca0irctl: UCA0IRCTL,
    _reserved4: [u8; 10usize],
    _reserved_4_uca0: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x06 - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca0txbuf(&self) -> &UCA0TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCA0TXBUF) }
    }
    #[doc = "0x06 - eUSCI_Ax Transmit Buffer Register"]
    #[inline(always)]
    pub fn uca0txbuf_mut(&self) -> &mut UCA0TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCA0TXBUF) }
    }
    #[doc = "0x16 - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca0iv(&self) -> &UCA0IV {
        unsafe { &*(((self as *const Self) as *const u8).add(22usize) as *const UCA0IV) }
    }
    #[doc = "0x16 - eUSCI_Ax Interrupt Vector Register"]
    #[inline(always)]
    pub fn uca0iv_mut(&self) -> &mut UCA0IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(22usize) as *mut UCA0IV) }
    }
}
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0mctlw](uca0mctlw) module"]
pub type UCA0MCTLW = crate::Reg<u16, _UCA0MCTLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0MCTLW;
#[doc = "`read()` method returns [uca0mctlw::R](uca0mctlw::R) reader structure"]
impl crate::Readable for UCA0MCTLW {}
#[doc = "`write(|w| ..)` method takes [uca0mctlw::W](uca0mctlw::W) writer structure"]
impl crate::Writable for UCA0MCTLW {}
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod uca0mctlw;
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0txbuf](uca0txbuf) module"]
pub type UCA0TXBUF = crate::Reg<u16, _UCA0TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0TXBUF;
#[doc = "`read()` method returns [uca0txbuf::R](uca0txbuf::R) reader structure"]
impl crate::Readable for UCA0TXBUF {}
#[doc = "`write(|w| ..)` method takes [uca0txbuf::W](uca0txbuf::W) writer structure"]
impl crate::Writable for UCA0TXBUF {}
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod uca0txbuf;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0abctl](uca0abctl) module"]
pub type UCA0ABCTL = crate::Reg<u16, _UCA0ABCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0ABCTL;
#[doc = "`read()` method returns [uca0abctl::R](uca0abctl::R) reader structure"]
impl crate::Readable for UCA0ABCTL {}
#[doc = "`write(|w| ..)` method takes [uca0abctl::W](uca0abctl::W) writer structure"]
impl crate::Writable for UCA0ABCTL {}
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod uca0abctl;
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0irctl](uca0irctl) module"]
pub type UCA0IRCTL = crate::Reg<u16, _UCA0IRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IRCTL;
#[doc = "`read()` method returns [uca0irctl::R](uca0irctl::R) reader structure"]
impl crate::Readable for UCA0IRCTL {}
#[doc = "`write(|w| ..)` method takes [uca0irctl::W](uca0irctl::W) writer structure"]
impl crate::Writable for UCA0IRCTL {}
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod uca0irctl;
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [uca0iv](uca0iv) module"]
pub type UCA0IV = crate::Reg<u16, _UCA0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCA0IV;
#[doc = "`read()` method returns [uca0iv::R](uca0iv::R) reader structure"]
impl crate::Readable for UCA0IV {}
#[doc = "`write(|w| ..)` method takes [uca0iv::W](uca0iv::W) writer structure"]
impl crate::Writable for UCA0IV {}
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod uca0iv;

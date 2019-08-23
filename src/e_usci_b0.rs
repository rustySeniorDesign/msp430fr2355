#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ucb0: [u8; 2usize],
    _reserved1: [u8; 4usize],
    _reserved_1_ucb0: [u8; 2usize],
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - eUSCI_Bx I2C Own Address 0 Register"]
    pub ucb0i2coa0: UCB0I2COA0,
    _reserved3: [u8; 8usize],
    #[doc = "0x16 - eUSCI_Bx I2C Address Mask Register"]
    pub ucb0addmask: UCB0ADDMASK,
    _reserved4: [u8; 14usize],
    _reserved_4_ucb0: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x00 - UCB0STATW_SPI"]
    #[inline(always)]
    pub fn ucb0statw(&self) -> &UCB0STATW {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UCB0STATW) }
    }
    #[doc = "0x00 - UCB0STATW_SPI"]
    #[inline(always)]
    pub fn ucb0statw_mut(&self) -> &mut UCB0STATW {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UCB0STATW) }
    }
    #[doc = "0x06 - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb0txbuf(&self) -> &UCB0TXBUF {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const UCB0TXBUF) }
    }
    #[doc = "0x06 - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub fn ucb0txbuf_mut(&self) -> &mut UCB0TXBUF {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut UCB0TXBUF) }
    }
    #[doc = "0x26 - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb0iv(&self) -> &UCB0IV {
        unsafe { &*(((self as *const Self) as *const u8).add(38usize) as *const UCB0IV) }
    }
    #[doc = "0x26 - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub fn ucb0iv_mut(&self) -> &mut UCB0IV {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(38usize) as *mut UCB0IV) }
    }
}
#[doc = "eUSCI_Bx Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0statw](ucb0statw) module"]
pub type UCB0STATW = crate::Reg<u16, _UCB0STATW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0STATW;
#[doc = "`read()` method returns [ucb0statw::R](ucb0statw::R) reader structure"]
impl crate::Readable for UCB0STATW {}
#[doc = "`write(|w| ..)` method takes [ucb0statw::W](ucb0statw::W) writer structure"]
impl crate::Writable for UCB0STATW {}
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb0statw;
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0txbuf](ucb0txbuf) module"]
pub type UCB0TXBUF = crate::Reg<u16, _UCB0TXBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0TXBUF;
#[doc = "`read()` method returns [ucb0txbuf::R](ucb0txbuf::R) reader structure"]
impl crate::Readable for UCB0TXBUF {}
#[doc = "`write(|w| ..)` method takes [ucb0txbuf::W](ucb0txbuf::W) writer structure"]
impl crate::Writable for UCB0TXBUF {}
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb0txbuf;
#[doc = "eUSCI_Bx I2C Own Address 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0i2coa0](ucb0i2coa0) module"]
pub type UCB0I2COA0 = crate::Reg<u16, _UCB0I2COA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0I2COA0;
#[doc = "`read()` method returns [ucb0i2coa0::R](ucb0i2coa0::R) reader structure"]
impl crate::Readable for UCB0I2COA0 {}
#[doc = "`write(|w| ..)` method takes [ucb0i2coa0::W](ucb0i2coa0::W) writer structure"]
impl crate::Writable for UCB0I2COA0 {}
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb0i2coa0;
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0addmask](ucb0addmask) module"]
pub type UCB0ADDMASK = crate::Reg<u16, _UCB0ADDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0ADDMASK;
#[doc = "`read()` method returns [ucb0addmask::R](ucb0addmask::R) reader structure"]
impl crate::Readable for UCB0ADDMASK {}
#[doc = "`write(|w| ..)` method takes [ucb0addmask::W](ucb0addmask::W) writer structure"]
impl crate::Writable for UCB0ADDMASK {}
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb0addmask;
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucb0iv](ucb0iv) module"]
pub type UCB0IV = crate::Reg<u16, _UCB0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCB0IV;
#[doc = "`read()` method returns [ucb0iv::R](ucb0iv::R) reader structure"]
impl crate::Readable for UCB0IV {}
#[doc = "`write(|w| ..)` method takes [ucb0iv::W](ucb0iv::W) writer structure"]
impl crate::Writable for UCB0IV {}
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb0iv;

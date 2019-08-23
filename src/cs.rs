#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock System Control 4"]
    pub csctl4: CSCTL4,
    _reserved1: [u8; 4usize],
    #[doc = "0x06 - Clock System Control Register 7"]
    pub csctl7: CSCTL7,
    #[doc = "0x08 - Clock System Control Register 8"]
    pub csctl8: CSCTL8,
}
#[doc = "Clock System Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl4](csctl4) module"]
pub type CSCTL4 = crate::Reg<u16, _CSCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL4;
#[doc = "`read()` method returns [csctl4::R](csctl4::R) reader structure"]
impl crate::Readable for CSCTL4 {}
#[doc = "`write(|w| ..)` method takes [csctl4::W](csctl4::W) writer structure"]
impl crate::Writable for CSCTL4 {}
#[doc = "Clock System Control 4"]
pub mod csctl4;
#[doc = "Clock System Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl7](csctl7) module"]
pub type CSCTL7 = crate::Reg<u16, _CSCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL7;
#[doc = "`read()` method returns [csctl7::R](csctl7::R) reader structure"]
impl crate::Readable for CSCTL7 {}
#[doc = "`write(|w| ..)` method takes [csctl7::W](csctl7::W) writer structure"]
impl crate::Writable for CSCTL7 {}
#[doc = "Clock System Control Register 7"]
pub mod csctl7;
#[doc = "Clock System Control Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csctl8](csctl8) module"]
pub type CSCTL8 = crate::Reg<u16, _CSCTL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCTL8;
#[doc = "`read()` method returns [csctl8::R](csctl8::R) reader structure"]
impl crate::Readable for CSCTL8 {}
#[doc = "`write(|w| ..)` method takes [csctl8::W](csctl8::W) writer structure"]
impl crate::Writable for CSCTL8 {}
#[doc = "Clock System Control Register 8"]
pub mod csctl8;

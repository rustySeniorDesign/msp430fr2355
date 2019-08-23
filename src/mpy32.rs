#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 16-bit operand two"]
    pub op2: OP2,
    _reserved1: [u8; 4usize],
    #[doc = "0x06 - 16x16-bit sum extension register"]
    pub sumext: SUMEXT,
    #[doc = "0x08 - 32-bit operand 1 multiply low word"]
    pub mpy32l: MPY32L,
    #[doc = "0x0a - 32-bit operand 1 multiply high word"]
    pub mpy32h: MPY32H,
    #[doc = "0x0c - 32-bit operand 1 signed multiply low word"]
    pub mpys32l: MPYS32L,
    _reserved5: [u8; 8usize],
    #[doc = "0x16 - 32-bit operand 1 signed multiply accumulate high word"]
    pub macs32h: MACS32H,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - 32x32-bit result 2"]
    pub res2: RES2,
}
#[doc = "16-bit operand two\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [op2](op2) module"]
pub type OP2 = crate::Reg<u16, _OP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OP2;
#[doc = "`read()` method returns [op2::R](op2::R) reader structure"]
impl crate::Readable for OP2 {}
#[doc = "`write(|w| ..)` method takes [op2::W](op2::W) writer structure"]
impl crate::Writable for OP2 {}
#[doc = "16-bit operand two"]
pub mod op2;
#[doc = "16x16-bit sum extension register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sumext](sumext) module"]
pub type SUMEXT = crate::Reg<u16, _SUMEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUMEXT;
#[doc = "`read()` method returns [sumext::R](sumext::R) reader structure"]
impl crate::Readable for SUMEXT {}
#[doc = "`write(|w| ..)` method takes [sumext::W](sumext::W) writer structure"]
impl crate::Writable for SUMEXT {}
#[doc = "16x16-bit sum extension register"]
pub mod sumext;
#[doc = "32-bit operand 1 multiply low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpy32l](mpy32l) module"]
pub type MPY32L = crate::Reg<u16, _MPY32L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPY32L;
#[doc = "`read()` method returns [mpy32l::R](mpy32l::R) reader structure"]
impl crate::Readable for MPY32L {}
#[doc = "`write(|w| ..)` method takes [mpy32l::W](mpy32l::W) writer structure"]
impl crate::Writable for MPY32L {}
#[doc = "32-bit operand 1 multiply low word"]
pub mod mpy32l;
#[doc = "32-bit operand 1 multiply high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpy32h](mpy32h) module"]
pub type MPY32H = crate::Reg<u16, _MPY32H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPY32H;
#[doc = "`read()` method returns [mpy32h::R](mpy32h::R) reader structure"]
impl crate::Readable for MPY32H {}
#[doc = "`write(|w| ..)` method takes [mpy32h::W](mpy32h::W) writer structure"]
impl crate::Writable for MPY32H {}
#[doc = "32-bit operand 1 multiply high word"]
pub mod mpy32h;
#[doc = "32-bit operand 1 signed multiply low word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mpys32l](mpys32l) module"]
pub type MPYS32L = crate::Reg<u16, _MPYS32L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPYS32L;
#[doc = "`read()` method returns [mpys32l::R](mpys32l::R) reader structure"]
impl crate::Readable for MPYS32L {}
#[doc = "`write(|w| ..)` method takes [mpys32l::W](mpys32l::W) writer structure"]
impl crate::Writable for MPYS32L {}
#[doc = "32-bit operand 1 signed multiply low word"]
pub mod mpys32l;
#[doc = "32-bit operand 1 signed multiply accumulate high word\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [macs32h](macs32h) module"]
pub type MACS32H = crate::Reg<u16, _MACS32H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MACS32H;
#[doc = "`read()` method returns [macs32h::R](macs32h::R) reader structure"]
impl crate::Readable for MACS32H {}
#[doc = "`write(|w| ..)` method takes [macs32h::W](macs32h::W) writer structure"]
impl crate::Writable for MACS32H {}
#[doc = "32-bit operand 1 signed multiply accumulate high word"]
pub mod macs32h;
#[doc = "32x32-bit result 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [res2](res2) module"]
pub type RES2 = crate::Reg<u16, _RES2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES2;
#[doc = "`read()` method returns [res2::R](res2::R) reader structure"]
impl crate::Readable for RES2 {}
#[doc = "`write(|w| ..)` method takes [res2::W](res2::W) writer structure"]
impl crate::Writable for RES2 {}
#[doc = "32x32-bit result 2"]
pub mod res2;

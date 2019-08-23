#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1usize],
    #[doc = "0x01 - Port 10 Input"]
    pub p10in: P10IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x03 - Port 10 Output"]
    pub p10out: P10OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x05 - Port 10 Direction"]
    pub p10dir: P10DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x07 - Port 10 Resistor Enable"]
    pub p10ren: P10REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x0b - Port 10 Select 0"]
    pub p10sel0: P10SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x0d - Port 10 Select 1"]
    pub p10sel1: P10SEL1,
    _reserved6: [u8; 9usize],
    #[doc = "0x17 - Port 10 Complement Select"]
    pub p10selc: P10SELC,
    _reserved7: [u8; 1usize],
    #[doc = "0x19 - Port 10 Interrupt Edge Select"]
    pub p10ies: P10IES,
    _reserved8: [u8; 1usize],
    #[doc = "0x1b - Port 10 Interrupt Enable"]
    pub p10ie: P10IE,
    _reserved9: [u8; 1usize],
    #[doc = "0x1d - Port 10 Interrupt Flag"]
    pub p10ifg: P10IFG,
}
#[doc = "Port 10 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10in](p10in) module"]
pub type P10IN = crate::Reg<u8, _P10IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10IN;
#[doc = "`read()` method returns [p10in::R](p10in::R) reader structure"]
impl crate::Readable for P10IN {}
#[doc = "`write(|w| ..)` method takes [p10in::W](p10in::W) writer structure"]
impl crate::Writable for P10IN {}
#[doc = "Port 10 Input"]
pub mod p10in;
#[doc = "Port 10 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10out](p10out) module"]
pub type P10OUT = crate::Reg<u8, _P10OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10OUT;
#[doc = "`read()` method returns [p10out::R](p10out::R) reader structure"]
impl crate::Readable for P10OUT {}
#[doc = "`write(|w| ..)` method takes [p10out::W](p10out::W) writer structure"]
impl crate::Writable for P10OUT {}
#[doc = "Port 10 Output"]
pub mod p10out;
#[doc = "Port 10 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10dir](p10dir) module"]
pub type P10DIR = crate::Reg<u8, _P10DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10DIR;
#[doc = "`read()` method returns [p10dir::R](p10dir::R) reader structure"]
impl crate::Readable for P10DIR {}
#[doc = "`write(|w| ..)` method takes [p10dir::W](p10dir::W) writer structure"]
impl crate::Writable for P10DIR {}
#[doc = "Port 10 Direction"]
pub mod p10dir;
#[doc = "Port 10 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10ren](p10ren) module"]
pub type P10REN = crate::Reg<u8, _P10REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10REN;
#[doc = "`read()` method returns [p10ren::R](p10ren::R) reader structure"]
impl crate::Readable for P10REN {}
#[doc = "`write(|w| ..)` method takes [p10ren::W](p10ren::W) writer structure"]
impl crate::Writable for P10REN {}
#[doc = "Port 10 Resistor Enable"]
pub mod p10ren;
#[doc = "Port 10 Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10sel0](p10sel0) module"]
pub type P10SEL0 = crate::Reg<u8, _P10SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10SEL0;
#[doc = "`read()` method returns [p10sel0::R](p10sel0::R) reader structure"]
impl crate::Readable for P10SEL0 {}
#[doc = "`write(|w| ..)` method takes [p10sel0::W](p10sel0::W) writer structure"]
impl crate::Writable for P10SEL0 {}
#[doc = "Port 10 Select 0"]
pub mod p10sel0;
#[doc = "Port 10 Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10sel1](p10sel1) module"]
pub type P10SEL1 = crate::Reg<u8, _P10SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10SEL1;
#[doc = "`read()` method returns [p10sel1::R](p10sel1::R) reader structure"]
impl crate::Readable for P10SEL1 {}
#[doc = "`write(|w| ..)` method takes [p10sel1::W](p10sel1::W) writer structure"]
impl crate::Writable for P10SEL1 {}
#[doc = "Port 10 Select 1"]
pub mod p10sel1;
#[doc = "Port 10 Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10selc](p10selc) module"]
pub type P10SELC = crate::Reg<u8, _P10SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10SELC;
#[doc = "`read()` method returns [p10selc::R](p10selc::R) reader structure"]
impl crate::Readable for P10SELC {}
#[doc = "`write(|w| ..)` method takes [p10selc::W](p10selc::W) writer structure"]
impl crate::Writable for P10SELC {}
#[doc = "Port 10 Complement Select"]
pub mod p10selc;
#[doc = "Port 10 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10ies](p10ies) module"]
pub type P10IES = crate::Reg<u8, _P10IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10IES;
#[doc = "`read()` method returns [p10ies::R](p10ies::R) reader structure"]
impl crate::Readable for P10IES {}
#[doc = "`write(|w| ..)` method takes [p10ies::W](p10ies::W) writer structure"]
impl crate::Writable for P10IES {}
#[doc = "Port 10 Interrupt Edge Select"]
pub mod p10ies;
#[doc = "Port 10 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10ie](p10ie) module"]
pub type P10IE = crate::Reg<u8, _P10IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10IE;
#[doc = "`read()` method returns [p10ie::R](p10ie::R) reader structure"]
impl crate::Readable for P10IE {}
#[doc = "`write(|w| ..)` method takes [p10ie::W](p10ie::W) writer structure"]
impl crate::Writable for P10IE {}
#[doc = "Port 10 Interrupt Enable"]
pub mod p10ie;
#[doc = "Port 10 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p10ifg](p10ifg) module"]
pub type P10IFG = crate::Reg<u8, _P10IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P10IFG;
#[doc = "`read()` method returns [p10ifg::R](p10ifg::R) reader structure"]
impl crate::Readable for P10IFG {}
#[doc = "`write(|w| ..)` method takes [p10ifg::W](p10ifg::W) writer structure"]
impl crate::Writable for P10IFG {}
#[doc = "Port 10 Interrupt Flag"]
pub mod p10ifg;

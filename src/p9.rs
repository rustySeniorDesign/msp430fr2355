#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 9 Input"]
    pub p9in: P9IN,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Port 9 Output"]
    pub p9out: P9OUT,
    _reserved2: [u8; 1usize],
    #[doc = "0x04 - Port 9 Direction"]
    pub p9dir: P9DIR,
    _reserved3: [u8; 1usize],
    #[doc = "0x06 - Port 9 Resistor Enable"]
    pub p9ren: P9REN,
    _reserved4: [u8; 3usize],
    #[doc = "0x0a - Port 9 Select 0"]
    pub p9sel0: P9SEL0,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - Port 9 Select 1"]
    pub p9sel1: P9SEL1,
    _reserved6: [u8; 1usize],
    #[doc = "0x0e - Port 9 Interrupt Vector Register"]
    pub p9iv: P9IV,
    _reserved7: [u8; 6usize],
    #[doc = "0x16 - Port 9 Complement Select"]
    pub p9selc: P9SELC,
    _reserved8: [u8; 1usize],
    #[doc = "0x18 - Port 9 Interrupt Edge Select"]
    pub p9ies: P9IES,
    _reserved9: [u8; 1usize],
    #[doc = "0x1a - Port 9 Interrupt Enable"]
    pub p9ie: P9IE,
    _reserved10: [u8; 1usize],
    #[doc = "0x1c - Port 9 Interrupt Flag"]
    pub p9ifg: P9IFG,
}
#[doc = "Port 9 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9in](p9in) module"]
pub type P9IN = crate::Reg<u8, _P9IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9IN;
#[doc = "`read()` method returns [p9in::R](p9in::R) reader structure"]
impl crate::Readable for P9IN {}
#[doc = "`write(|w| ..)` method takes [p9in::W](p9in::W) writer structure"]
impl crate::Writable for P9IN {}
#[doc = "Port 9 Input"]
pub mod p9in;
#[doc = "Port 9 Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9out](p9out) module"]
pub type P9OUT = crate::Reg<u8, _P9OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9OUT;
#[doc = "`read()` method returns [p9out::R](p9out::R) reader structure"]
impl crate::Readable for P9OUT {}
#[doc = "`write(|w| ..)` method takes [p9out::W](p9out::W) writer structure"]
impl crate::Writable for P9OUT {}
#[doc = "Port 9 Output"]
pub mod p9out;
#[doc = "Port 9 Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9dir](p9dir) module"]
pub type P9DIR = crate::Reg<u8, _P9DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9DIR;
#[doc = "`read()` method returns [p9dir::R](p9dir::R) reader structure"]
impl crate::Readable for P9DIR {}
#[doc = "`write(|w| ..)` method takes [p9dir::W](p9dir::W) writer structure"]
impl crate::Writable for P9DIR {}
#[doc = "Port 9 Direction"]
pub mod p9dir;
#[doc = "Port 9 Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9ren](p9ren) module"]
pub type P9REN = crate::Reg<u8, _P9REN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9REN;
#[doc = "`read()` method returns [p9ren::R](p9ren::R) reader structure"]
impl crate::Readable for P9REN {}
#[doc = "`write(|w| ..)` method takes [p9ren::W](p9ren::W) writer structure"]
impl crate::Writable for P9REN {}
#[doc = "Port 9 Resistor Enable"]
pub mod p9ren;
#[doc = "Port 9 Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9sel0](p9sel0) module"]
pub type P9SEL0 = crate::Reg<u8, _P9SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9SEL0;
#[doc = "`read()` method returns [p9sel0::R](p9sel0::R) reader structure"]
impl crate::Readable for P9SEL0 {}
#[doc = "`write(|w| ..)` method takes [p9sel0::W](p9sel0::W) writer structure"]
impl crate::Writable for P9SEL0 {}
#[doc = "Port 9 Select 0"]
pub mod p9sel0;
#[doc = "Port 9 Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9sel1](p9sel1) module"]
pub type P9SEL1 = crate::Reg<u8, _P9SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9SEL1;
#[doc = "`read()` method returns [p9sel1::R](p9sel1::R) reader structure"]
impl crate::Readable for P9SEL1 {}
#[doc = "`write(|w| ..)` method takes [p9sel1::W](p9sel1::W) writer structure"]
impl crate::Writable for P9SEL1 {}
#[doc = "Port 9 Select 1"]
pub mod p9sel1;
#[doc = "Port 9 Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9selc](p9selc) module"]
pub type P9SELC = crate::Reg<u8, _P9SELC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9SELC;
#[doc = "`read()` method returns [p9selc::R](p9selc::R) reader structure"]
impl crate::Readable for P9SELC {}
#[doc = "`write(|w| ..)` method takes [p9selc::W](p9selc::W) writer structure"]
impl crate::Writable for P9SELC {}
#[doc = "Port 9 Complement Select"]
pub mod p9selc;
#[doc = "Port 9 Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9ies](p9ies) module"]
pub type P9IES = crate::Reg<u8, _P9IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9IES;
#[doc = "`read()` method returns [p9ies::R](p9ies::R) reader structure"]
impl crate::Readable for P9IES {}
#[doc = "`write(|w| ..)` method takes [p9ies::W](p9ies::W) writer structure"]
impl crate::Writable for P9IES {}
#[doc = "Port 9 Interrupt Edge Select"]
pub mod p9ies;
#[doc = "Port 9 Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9ie](p9ie) module"]
pub type P9IE = crate::Reg<u8, _P9IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9IE;
#[doc = "`read()` method returns [p9ie::R](p9ie::R) reader structure"]
impl crate::Readable for P9IE {}
#[doc = "`write(|w| ..)` method takes [p9ie::W](p9ie::W) writer structure"]
impl crate::Writable for P9IE {}
#[doc = "Port 9 Interrupt Enable"]
pub mod p9ie;
#[doc = "Port 9 Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9ifg](p9ifg) module"]
pub type P9IFG = crate::Reg<u8, _P9IFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9IFG;
#[doc = "`read()` method returns [p9ifg::R](p9ifg::R) reader structure"]
impl crate::Readable for P9IFG {}
#[doc = "`write(|w| ..)` method takes [p9ifg::W](p9ifg::W) writer structure"]
impl crate::Writable for P9IFG {}
#[doc = "Port 9 Interrupt Flag"]
pub mod p9ifg;
#[doc = "Port 9 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [p9iv](p9iv) module"]
pub type P9IV = crate::Reg<u16, _P9IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _P9IV;
#[doc = "`read()` method returns [p9iv::R](p9iv::R) reader structure"]
impl crate::Readable for P9IV {}
#[doc = "`write(|w| ..)` method takes [p9iv::W](p9iv::W) writer structure"]
impl crate::Writable for P9IV {}
#[doc = "Port 9 Interrupt Vector Register"]
pub mod p9iv;

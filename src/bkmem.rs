#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backup Memory 4."]
    pub bakmem4: BAKMEM4,
    _reserved1: [u8; 4usize],
    #[doc = "0x06 - Backup Memory 7."]
    pub bakmem7: BAKMEM7,
    #[doc = "0x08 - Backup Memory 8."]
    pub bakmem8: BAKMEM8,
    #[doc = "0x0a - Backup Memory 9."]
    pub bakmem9: BAKMEM9,
    #[doc = "0x0c - Backup Memory registers. Backup Memory 10."]
    pub bakmem10: BAKMEM10,
    _reserved5: [u8; 8usize],
    #[doc = "0x16 - Backup Memory 15."]
    pub bakmem15: BAKMEM15,
}
#[doc = "Backup Memory 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem4](bakmem4) module"]
pub type BAKMEM4 = crate::Reg<u16, _BAKMEM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM4;
#[doc = "`read()` method returns [bakmem4::R](bakmem4::R) reader structure"]
impl crate::Readable for BAKMEM4 {}
#[doc = "`write(|w| ..)` method takes [bakmem4::W](bakmem4::W) writer structure"]
impl crate::Writable for BAKMEM4 {}
#[doc = "Backup Memory 4."]
pub mod bakmem4;
#[doc = "Backup Memory 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem7](bakmem7) module"]
pub type BAKMEM7 = crate::Reg<u16, _BAKMEM7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM7;
#[doc = "`read()` method returns [bakmem7::R](bakmem7::R) reader structure"]
impl crate::Readable for BAKMEM7 {}
#[doc = "`write(|w| ..)` method takes [bakmem7::W](bakmem7::W) writer structure"]
impl crate::Writable for BAKMEM7 {}
#[doc = "Backup Memory 7."]
pub mod bakmem7;
#[doc = "Backup Memory 8.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem8](bakmem8) module"]
pub type BAKMEM8 = crate::Reg<u16, _BAKMEM8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM8;
#[doc = "`read()` method returns [bakmem8::R](bakmem8::R) reader structure"]
impl crate::Readable for BAKMEM8 {}
#[doc = "`write(|w| ..)` method takes [bakmem8::W](bakmem8::W) writer structure"]
impl crate::Writable for BAKMEM8 {}
#[doc = "Backup Memory 8."]
pub mod bakmem8;
#[doc = "Backup Memory 9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem9](bakmem9) module"]
pub type BAKMEM9 = crate::Reg<u16, _BAKMEM9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM9;
#[doc = "`read()` method returns [bakmem9::R](bakmem9::R) reader structure"]
impl crate::Readable for BAKMEM9 {}
#[doc = "`write(|w| ..)` method takes [bakmem9::W](bakmem9::W) writer structure"]
impl crate::Writable for BAKMEM9 {}
#[doc = "Backup Memory 9."]
pub mod bakmem9;
#[doc = "Backup Memory registers. Backup Memory 10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem10](bakmem10) module"]
pub type BAKMEM10 = crate::Reg<u16, _BAKMEM10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM10;
#[doc = "`read()` method returns [bakmem10::R](bakmem10::R) reader structure"]
impl crate::Readable for BAKMEM10 {}
#[doc = "`write(|w| ..)` method takes [bakmem10::W](bakmem10::W) writer structure"]
impl crate::Writable for BAKMEM10 {}
#[doc = "Backup Memory registers. Backup Memory 10."]
pub mod bakmem10;
#[doc = "Backup Memory 15.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bakmem15](bakmem15) module"]
pub type BAKMEM15 = crate::Reg<u16, _BAKMEM15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAKMEM15;
#[doc = "`read()` method returns [bakmem15::R](bakmem15::R) reader structure"]
impl crate::Readable for BAKMEM15 {}
#[doc = "`write(|w| ..)` method takes [bakmem15::W](bakmem15::W) writer structure"]
impl crate::Writable for BAKMEM15 {}
#[doc = "Backup Memory 15."]
pub mod bakmem15;

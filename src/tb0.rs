#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B count register"]
    pub tb0r: TB0R,
    #[doc = "0x02 - Timer_B Capture/Compare Register"]
    pub tb0ccr0: TB0CCR0,
    #[doc = "0x04 - Timer_B Capture/Compare Register"]
    pub tb0ccr1: TB0CCR1,
    _reserved3: [u8; 24usize],
    #[doc = "0x1e - Timer_Bx Interrupt Vector Register"]
    pub tb0iv: TB0IV,
}
#[doc = "Timer_B count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb0r](tb0r) module"]
pub type TB0R = crate::Reg<u16, _TB0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0R;
#[doc = "`read()` method returns [tb0r::R](tb0r::R) reader structure"]
impl crate::Readable for TB0R {}
#[doc = "`write(|w| ..)` method takes [tb0r::W](tb0r::W) writer structure"]
impl crate::Writable for TB0R {}
#[doc = "Timer_B count register"]
pub mod tb0r;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb0ccr0](tb0ccr0) module"]
pub type TB0CCR0 = crate::Reg<u16, _TB0CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR0;
#[doc = "`read()` method returns [tb0ccr0::R](tb0ccr0::R) reader structure"]
impl crate::Readable for TB0CCR0 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr0::W](tb0ccr0::W) writer structure"]
impl crate::Writable for TB0CCR0 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr0;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb0ccr1](tb0ccr1) module"]
pub type TB0CCR1 = crate::Reg<u16, _TB0CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0CCR1;
#[doc = "`read()` method returns [tb0ccr1::R](tb0ccr1::R) reader structure"]
impl crate::Readable for TB0CCR1 {}
#[doc = "`write(|w| ..)` method takes [tb0ccr1::W](tb0ccr1::W) writer structure"]
impl crate::Writable for TB0CCR1 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb0ccr1;
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb0iv](tb0iv) module"]
pub type TB0IV = crate::Reg<u16, _TB0IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB0IV;
#[doc = "`read()` method returns [tb0iv::R](tb0iv::R) reader structure"]
impl crate::Readable for TB0IV {}
#[doc = "`write(|w| ..)` method takes [tb0iv::W](tb0iv::W) writer structure"]
impl crate::Writable for TB0IV {}
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb0iv;

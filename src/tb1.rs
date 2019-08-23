#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B count register"]
    pub tb1r: TB1R,
    #[doc = "0x02 - Timer_B Capture/Compare Register"]
    pub tb1ccr0: TB1CCR0,
    #[doc = "0x04 - Timer_B Capture/Compare Register"]
    pub tb1ccr1: TB1CCR1,
    _reserved3: [u8; 24usize],
    #[doc = "0x1e - Timer_Bx Interrupt Vector Register"]
    pub tb1iv: TB1IV,
}
#[doc = "Timer_B count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1r](tb1r) module"]
pub type TB1R = crate::Reg<u16, _TB1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1R;
#[doc = "`read()` method returns [tb1r::R](tb1r::R) reader structure"]
impl crate::Readable for TB1R {}
#[doc = "`write(|w| ..)` method takes [tb1r::W](tb1r::W) writer structure"]
impl crate::Writable for TB1R {}
#[doc = "Timer_B count register"]
pub mod tb1r;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1ccr0](tb1ccr0) module"]
pub type TB1CCR0 = crate::Reg<u16, _TB1CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1CCR0;
#[doc = "`read()` method returns [tb1ccr0::R](tb1ccr0::R) reader structure"]
impl crate::Readable for TB1CCR0 {}
#[doc = "`write(|w| ..)` method takes [tb1ccr0::W](tb1ccr0::W) writer structure"]
impl crate::Writable for TB1CCR0 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr0;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1ccr1](tb1ccr1) module"]
pub type TB1CCR1 = crate::Reg<u16, _TB1CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1CCR1;
#[doc = "`read()` method returns [tb1ccr1::R](tb1ccr1::R) reader structure"]
impl crate::Readable for TB1CCR1 {}
#[doc = "`write(|w| ..)` method takes [tb1ccr1::W](tb1ccr1::W) writer structure"]
impl crate::Writable for TB1CCR1 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb1ccr1;
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb1iv](tb1iv) module"]
pub type TB1IV = crate::Reg<u16, _TB1IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB1IV;
#[doc = "`read()` method returns [tb1iv::R](tb1iv::R) reader structure"]
impl crate::Readable for TB1IV {}
#[doc = "`write(|w| ..)` method takes [tb1iv::W](tb1iv::W) writer structure"]
impl crate::Writable for TB1IV {}
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb1iv;

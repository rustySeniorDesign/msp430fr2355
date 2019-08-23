#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer_B count register"]
    pub tb2r: TB2R,
    #[doc = "0x02 - Timer_B Capture/Compare Register"]
    pub tb2ccr0: TB2CCR0,
    #[doc = "0x04 - Timer_B Capture/Compare Register"]
    pub tb2ccr1: TB2CCR1,
    _reserved3: [u8; 24usize],
    #[doc = "0x1e - Timer_Bx Interrupt Vector Register"]
    pub tb2iv: TB2IV,
}
#[doc = "Timer_B count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb2r](tb2r) module"]
pub type TB2R = crate::Reg<u16, _TB2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2R;
#[doc = "`read()` method returns [tb2r::R](tb2r::R) reader structure"]
impl crate::Readable for TB2R {}
#[doc = "`write(|w| ..)` method takes [tb2r::W](tb2r::W) writer structure"]
impl crate::Writable for TB2R {}
#[doc = "Timer_B count register"]
pub mod tb2r;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb2ccr0](tb2ccr0) module"]
pub type TB2CCR0 = crate::Reg<u16, _TB2CCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2CCR0;
#[doc = "`read()` method returns [tb2ccr0::R](tb2ccr0::R) reader structure"]
impl crate::Readable for TB2CCR0 {}
#[doc = "`write(|w| ..)` method takes [tb2ccr0::W](tb2ccr0::W) writer structure"]
impl crate::Writable for TB2CCR0 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr0;
#[doc = "Timer_B Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb2ccr1](tb2ccr1) module"]
pub type TB2CCR1 = crate::Reg<u16, _TB2CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2CCR1;
#[doc = "`read()` method returns [tb2ccr1::R](tb2ccr1::R) reader structure"]
impl crate::Readable for TB2CCR1 {}
#[doc = "`write(|w| ..)` method takes [tb2ccr1::W](tb2ccr1::W) writer structure"]
impl crate::Writable for TB2CCR1 {}
#[doc = "Timer_B Capture/Compare Register"]
pub mod tb2ccr1;
#[doc = "Timer_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tb2iv](tb2iv) module"]
pub type TB2IV = crate::Reg<u16, _TB2IV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TB2IV;
#[doc = "`read()` method returns [tb2iv::R](tb2iv::R) reader structure"]
impl crate::Readable for TB2IV {}
#[doc = "`write(|w| ..)` method takes [tb2iv::W](tb2iv::W) writer structure"]
impl crate::Writable for TB2IV {}
#[doc = "Timer_Bx Interrupt Vector Register"]
pub mod tb2iv;

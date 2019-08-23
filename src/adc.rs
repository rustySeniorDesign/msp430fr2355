#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Window Comparator High Threshold Register"]
    pub adchi: ADCHI,
    _reserved1: [u8; 8usize],
    #[doc = "0x0a - ADC Conversion Memory Register"]
    pub adcmem0: ADCMEM0,
    _reserved2: [u8; 10usize],
    #[doc = "0x16 - ADC Interrupt Vector"]
    pub adciv: ADCIV,
}
#[doc = "ADC Window Comparator High Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adchi](adchi) module"]
pub type ADCHI = crate::Reg<u16, _ADCHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCHI;
#[doc = "`read()` method returns [adchi::R](adchi::R) reader structure"]
impl crate::Readable for ADCHI {}
#[doc = "`write(|w| ..)` method takes [adchi::W](adchi::W) writer structure"]
impl crate::Writable for ADCHI {}
#[doc = "ADC Window Comparator High Threshold Register"]
pub mod adchi;
#[doc = "ADC Conversion Memory Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adcmem0](adcmem0) module"]
pub type ADCMEM0 = crate::Reg<u16, _ADCMEM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCMEM0;
#[doc = "`read()` method returns [adcmem0::R](adcmem0::R) reader structure"]
impl crate::Readable for ADCMEM0 {}
#[doc = "`write(|w| ..)` method takes [adcmem0::W](adcmem0::W) writer structure"]
impl crate::Writable for ADCMEM0 {}
#[doc = "ADC Conversion Memory Register"]
pub mod adcmem0;
#[doc = "ADC Interrupt Vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adciv](adciv) module"]
pub type ADCIV = crate::Reg<u16, _ADCIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCIV;
#[doc = "`read()` method returns [adciv::R](adciv::R) reader structure"]
impl crate::Readable for ADCIV {}
#[doc = "`write(|w| ..)` method takes [adciv::W](adciv::W) writer structure"]
impl crate::Writable for ADCIV {}
#[doc = "ADC Interrupt Vector"]
pub mod adciv;

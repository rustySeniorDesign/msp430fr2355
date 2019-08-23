#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Interrupt Vector Word Register"]
    pub cpiv: CPIV,
    _reserved1: [u8; 6usize],
    #[doc = "0x08 - 6-bit Comparator built-in DAC Control Register"]
    pub cpdacctl: CPDACCTL,
    #[doc = "0x0a - 6-bit Comparator built-in DAC Data Register"]
    pub cpdacdata: CPDACDATA,
}
#[doc = "Comparator Interrupt Vector Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpiv](cpiv) module"]
pub type CPIV = crate::Reg<u16, _CPIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPIV;
#[doc = "`read()` method returns [cpiv::R](cpiv::R) reader structure"]
impl crate::Readable for CPIV {}
#[doc = "`write(|w| ..)` method takes [cpiv::W](cpiv::W) writer structure"]
impl crate::Writable for CPIV {}
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cpiv;
#[doc = "6-bit Comparator built-in DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpdacctl](cpdacctl) module"]
pub type CPDACCTL = crate::Reg<u16, _CPDACCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPDACCTL;
#[doc = "`read()` method returns [cpdacctl::R](cpdacctl::R) reader structure"]
impl crate::Readable for CPDACCTL {}
#[doc = "`write(|w| ..)` method takes [cpdacctl::W](cpdacctl::W) writer structure"]
impl crate::Writable for CPDACCTL {}
#[doc = "6-bit Comparator built-in DAC Control Register"]
pub mod cpdacctl;
#[doc = "6-bit Comparator built-in DAC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpdacdata](cpdacdata) module"]
pub type CPDACDATA = crate::Reg<u16, _CPDACDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPDACDATA;
#[doc = "`read()` method returns [cpdacdata::R](cpdacdata::R) reader structure"]
impl crate::Readable for CPDACDATA {}
#[doc = "`write(|w| ..)` method takes [cpdacdata::W](cpdacdata::W) writer structure"]
impl crate::Writable for CPDACDATA {}
#[doc = "6-bit Comparator built-in DAC Data Register"]
pub mod cpdacdata;

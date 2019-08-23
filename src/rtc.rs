#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Counter Modulo Register"]
    pub rtcmod: RTCMOD,
}
#[doc = "RTC Counter Modulo Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtcmod](rtcmod) module"]
pub type RTCMOD = crate::Reg<u16, _RTCMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCMOD;
#[doc = "`read()` method returns [rtcmod::R](rtcmod::R) reader structure"]
impl crate::Readable for RTCMOD {}
#[doc = "`write(|w| ..)` method takes [rtcmod::W](rtcmod::W) writer structure"]
impl crate::Writable for RTCMOD {}
#[doc = "RTC Counter Modulo Register"]
pub mod rtcmod;

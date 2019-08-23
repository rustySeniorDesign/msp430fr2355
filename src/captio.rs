#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Capacitive Touch IO 0 Control Register"]
    pub captioctl: CAPTIOCTL,
}
#[doc = "Capacitive Touch IO 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [captioctl](captioctl) module"]
pub type CAPTIOCTL = crate::Reg<u16, _CAPTIOCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPTIOCTL;
#[doc = "`read()` method returns [captioctl::R](captioctl::R) reader structure"]
impl crate::Readable for CAPTIOCTL {}
#[doc = "`write(|w| ..)` method takes [captioctl::W](captioctl::W) writer structure"]
impl crate::Writable for CAPTIOCTL {}
#[doc = "Capacitive Touch IO 0 Control Register"]
pub mod captioctl;

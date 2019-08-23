#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC DAC Status Register"]
    pub sac3dacsts: SAC3DACSTS,
}
#[doc = "SAC DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sac3dacsts](sac3dacsts) module"]
pub type SAC3DACSTS = crate::Reg<u16, _SAC3DACSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC3DACSTS;
#[doc = "`read()` method returns [sac3dacsts::R](sac3dacsts::R) reader structure"]
impl crate::Readable for SAC3DACSTS {}
#[doc = "`write(|w| ..)` method takes [sac3dacsts::W](sac3dacsts::W) writer structure"]
impl crate::Writable for SAC3DACSTS {}
#[doc = "SAC DAC Status Register"]
pub mod sac3dacsts;

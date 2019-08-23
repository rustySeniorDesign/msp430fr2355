#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SAC DAC Status Register"]
    pub sac0dacsts: SAC0DACSTS,
}
#[doc = "SAC DAC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sac0dacsts](sac0dacsts) module"]
pub type SAC0DACSTS = crate::Reg<u16, _SAC0DACSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC0DACSTS;
#[doc = "`read()` method returns [sac0dacsts::R](sac0dacsts::R) reader structure"]
impl crate::Readable for SAC0DACSTS {}
#[doc = "`write(|w| ..)` method takes [sac0dacsts::W](sac0dacsts::W) writer structure"]
impl crate::Writable for SAC0DACSTS {}
#[doc = "SAC DAC Status Register"]
pub mod sac0dacsts;

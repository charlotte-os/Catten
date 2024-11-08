pub mod interface;
pub mod aarch64;
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub type IoRegister8Impl= aarch64::IoRegister;
#[cfg(target_arch = "x86_64")]
pub type IoRegister8Impl = x86_64::IoRegister8;
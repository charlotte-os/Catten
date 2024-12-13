#[cfg(target_arch = "aarch64")]
pub mod aarch64;
pub mod interface;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::IoReg8;
#[cfg(target_arch = "x86_64")]
pub use x86_64::IoReg8;

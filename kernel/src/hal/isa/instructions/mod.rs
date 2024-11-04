pub mod interface;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "x86_64")]
pub type InstructionWrapper = x86_64::InstructionInterface;
#[cfg(target_arch = "aarch64")]
pub type InstructionWrapper = aarch64::InstructionInterface;
//! Input Controller Drivers

pub mod hid;
#[cfg(target_arch = "x86_64")]
pub mod i8042;

use lazy_static::lazy_static;

use crate::environment::boot_protocol::limine::HHDM_REQUEST;
#[cfg(target_arch = "x86_64")]
use crate::hal::isa::memory::{interface::MemoryInterface, MemoryInterfaceImpl};

type VAddr = <MemoryInterfaceImpl as MemoryInterface>::VAddr;

lazy_static! {
    pub static ref HHDM_BASE: VAddr = if let Some(response) = HHDM_REQUEST.get_response() {
        return VAddr::from(response.offset() as usize);
    } else {
        panic!("Limine failed to provide a higher half direct mapping region.");
    };
}

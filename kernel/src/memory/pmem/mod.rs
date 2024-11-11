use spin::Lazy;

use crate::environment::boot_protocol::limine::HHDM_REQUEST;
use crate::hal::isa::memory::x86_64::address::vaddr::VAddr;

pub static HHDM_BASE: Lazy<VAddr> = Lazy::new(|| {
        if let Some(response) = HHDM_REQUEST.get_response() {
                return VAddr::from(response.offset() as usize);
        } else {
                panic!("Limine failed to provide a higher half direct mapping region.");
        }
});

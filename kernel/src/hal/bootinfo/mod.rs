use core::ptr::NonNull;
use limine::{
        BaseRevision,
        request::{
            MemoryMapRequest,
            HhdmRequest
        },
        response::{
            MemoryMapResponse,
            HhdmResponse
        }
    };
use spin::Lazy;
use crate::hal::isa::memory::interface::{FromPtr};

pub static BASE_REVISION: BaseRevision = BaseRevision::new();

use super::isa::memory::x86_64::address::vaddr::VAddr;

pub static HHDM_BASE: Lazy<VAddr> = Lazy::new(|| {
    if let Some(response) = HHDM_REQUEST.get_response() {
        return VAddr::from(response.offset() as usize);
    }
    else {
        panic!("Limine failed to provide a higher half direct mapping region.");
    }
});

static MEMEORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
use limine::BaseRevision;
use limine::request::{
    ExecutableAddressRequest,
    FramebufferRequest,
    HhdmRequest,
    MemoryMapRequest,
    MpRequest,
    RsdpRequest,
    StackSizeRequest,
};

use crate::isa::memory::MemoryInterfaceImpl;
use crate::memory::MemoryInterface as _;

pub static BASE_REVISION: BaseRevision = BaseRevision::new();
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
pub static EXECUTABLE_ADDRESS_REQUEST: ExecutableAddressRequest = ExecutableAddressRequest::new();
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();
pub static SMP_REQUEST: MpRequest = MpRequest::new();
pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();
pub static STACK_SIZE: StackSizeRequest =
    StackSizeRequest::new().with_size((MemoryInterfaceImpl::PAGE_SIZE * 4) as u64);
pub static MP: MpRequest = MpRequest::new().with_flags(limine::mp::RequestFlags::X2APIC);

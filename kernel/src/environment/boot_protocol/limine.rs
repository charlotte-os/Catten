use limine::request::{
        FramebufferRequest,
        HhdmRequest,
        KernelAddressRequest,
        MemoryMapRequest,
        RsdpRequest,
        SmpRequest,
};
use limine::BaseRevision;

pub static BASE_REVISION: BaseRevision = BaseRevision::new();
pub static MEMEORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
pub static KERNEL_ADDRESS_REQUEST: KernelAddressRequest = KernelAddressRequest::new();
pub static FRAMBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();
pub static SMP_REQUEST: SmpRequest = SmpRequest::new();
pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

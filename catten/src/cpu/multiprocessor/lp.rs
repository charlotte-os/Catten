use crate::cpu::scheduler::threads::ThreadId;
use crate::memory::AddressSpaceId;

const AS_AFFINITY_COUNT: usize = 4095;

pub struct LogicalProcessor {
    as_affinities: [Option<AddressSpaceId>; AS_AFFINITY_COUNT],
}

use core::arch::asm;

use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::cpu::scheduler::system_scheduler::SYSTEM_SCHEDULER;
use crate::memory::{AddressSpaceId, VAddr};

pub fn inval_range_user(asid: AddressSpaceId, base: VAddr, size: usize) {
    // SAFETY: This is safe because we are executing in an interrupt context where
    // preemption is disabled, and we are not modifying any data structures that
    // could be accessed by other threads.
    if let Some(pcid) = SYSTEM_SCHEDULER.get_local_scheduler().lock().asid_to_hwasid(asid) {
        let raw_base = <VAddr as Into<usize>>::into(base);
        for page in (raw_base..raw_base + size * PAGE_SIZE).step_by(PAGE_SIZE) {
            let descriptor: [u64; 2] = [page as u64, pcid.get_inner() as u64];
            unsafe {
                asm!(
                    "invpcid {mode:r}, [{desc_ptr}]",
                    mode = in(reg) 0,
                    desc_ptr = in(reg) &descriptor,
                    options(nostack, preserves_flags),
                );
            }
        }
    }
}

pub fn inval_asid(asid: AddressSpaceId) {
    // SAFETY: This is safe because we are executing in an interrupt context where
    // preemption is disabled, and we are not modifying any data structures that
    // could be accessed by other threads.
    if let Some(pcid) = SYSTEM_SCHEDULER.get_local_scheduler().lock().asid_to_hwasid(asid) {
        let descriptor: [u64; 2] = [0, pcid.get_inner() as u64];
        unsafe {
            asm!(
                "invpcid {mode:r}, [{desc_ptr}]",
                mode = in(reg) 1,
                desc_ptr = in(reg) &descriptor,
                options(nostack, preserves_flags),
            );
        }
    }
}

pub fn inval_range_kernel(base: VAddr, num_pages: usize) {
    let raw_base = <VAddr as Into<usize>>::into(base);
    let len_bytes = num_pages * PAGE_SIZE;
    for page in (raw_base..raw_base + len_bytes).step_by(PAGE_SIZE) {
        unsafe {
            asm!(
                "invlpg [{page}]",
                page = in(reg) page,
                options(nostack, preserves_flags),
            );
        }
    }
}

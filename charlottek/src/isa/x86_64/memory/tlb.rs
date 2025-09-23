use core::arch::asm;

use crate::isa::lp::lp_local::LpLocal;
use crate::isa::memory::paging::PAGE_SIZE;
use crate::memory::{AddressSpaceId, VAddr};

pub fn inval_range_user(asid: AddressSpaceId, base: VAddr, size: usize) {
    // SAFETY: This is safe because we are executing in an interrupt context where
    // preemption is disabled, and we are not modifying any data structures that
    // could be accessed by other threads.
    if let Some(pcid) = unsafe { (*LpLocal::get()).local_scheduler.asid_to_pcid(asid) } {
        let raw_base = <VAddr as Into<usize>>::into(base);
        for page in (raw_base..raw_base + size * PAGE_SIZE).step_by(PAGE_SIZE) {
            let descriptor: [u64; 2] = [page as u64, pcid as u64];
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
    if let Some(pcid) = unsafe { (*LpLocal::get()).local_scheduler.asid_to_pcid(asid) } {
        let descriptor: [u64; 2] = [0, pcid as u64];
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

pub fn inval_range_kernel(base: VAddr, size: usize) {
    let raw_base = <VAddr as Into<usize>>::into(base);
    for page in (raw_base..raw_base + size * PAGE_SIZE).step_by(PAGE_SIZE) {
        unsafe {
            asm!(
                "invlpg [{page}]",
                page = in(reg) page,
                options(nostack, preserves_flags),
            );
        }
    }
}

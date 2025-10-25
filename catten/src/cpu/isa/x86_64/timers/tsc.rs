use alloc::vec::Vec;
use core::arch::asm;

use crate::logln;

pub fn rdtsc() -> u64 {
    let tsc_low: u32;
    let tsc_high: u32;
    unsafe {
        asm! {
            "rdtscp",
            out("eax") tsc_low,
            out("edx") tsc_high,
            out("ecx") _
        }
    }
    ((tsc_high as u64) << 32) | tsc_low as u64
}

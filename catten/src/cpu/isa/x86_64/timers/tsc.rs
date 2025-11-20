use core::arch::asm;
use core::arch::x86_64::__cpuid_count;

use spin::Lazy;

use crate::common::integer::nearest_multiple_of;
use crate::common::time::duration::ExtDuration;
use crate::cpu::isa::interface::system_info::CpuInfoIfce;
use crate::cpu::isa::system_info::{CpuInfo, IsaExtension};
use crate::cpu::isa::timers::i8254;

pub static IS_TSC_INVARIANT: Lazy<bool> =
    Lazy::new(|| CpuInfo::is_extension_supported(IsaExtension::InvariantTsc));
pub static TSC_FREQUENCY_HZ: Lazy<u64> = Lazy::new(get_tsc_freq);
pub static TSC_CYCLE_PERIOD: Lazy<ExtDuration> = Lazy::new(|| {
    let ps = 1_000_000_000_000 / *TSC_FREQUENCY_HZ;
    ExtDuration {
        secs: 0,
        picosecs: ps,
    }
});

pub fn rdtsc() -> u64 {
    //! # Read the timestamp counter with proper serialization
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

fn get_tsc_freq() -> u64 {
    //! # Get the timestamp counter frequency in Hz.
    // Determining the TSC frequency differs by vendor
    match CpuInfo::get_vendor().as_str() {
        "GenuineIntel" => get_tsc_freq_intel(),
        _ => get_tsc_freq_common(),
    }
}

fn get_tsc_freq_common() -> u64 {
    //! # Measure the TSC frequency using the legacy i8254 PIT
    use crate::cpu::isa::x86_64::timers::i8254::*;

    const N_SAMPLES: u64 = 8;
    let mut samples = [0u64; N_SAMPLES as usize];
    crate::cpu::isa::lp::ops::mask_interrupts!();
    for sample in samples.iter_mut() {
        i8254::set_interrupt_on_terminal_count((PIT_FREQUENCY_HZ / 25) as u16); // Set PIT to 40 ms
        while read_channel_2_output() {} // Wait for counting to start
        let start_tsc = rdtsc();
        while !read_channel_2_output() {} // Wait for counting to complete
        let end_tsc = rdtsc();
        *sample = end_tsc - start_tsc;
    }
    crate::cpu::isa::lp::ops::unmask_interrupts!();
    let mean_tsc_cycles = (samples.iter().sum::<u64>() / N_SAMPLES) * 25;
    // Round to the nearest MHz since CPU clocks are near universally multiples of that
    nearest_multiple_of(mean_tsc_cycles, 1_000_000)
}

fn get_tsc_freq_intel() -> u64 {
    //! # Use the CPUID instruction to determine the frequency of the TSC
    //! On Intel processors CPUID leaf 0x15 can be used to determine the frequency of the TSC.
    let cpuid_15 = unsafe { __cpuid_count(0x15, 0) };
    if cpuid_15.ecx != 0 && cpuid_15.eax != 0 && cpuid_15.ebx != 0 {
        cpuid_15.ecx as u64 * (cpuid_15.eax as u64 / cpuid_15.ebx as u64)
    } else {
        get_tsc_freq_common()
    }
}

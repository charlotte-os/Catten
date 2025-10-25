use core::arch::asm;
use core::arch::x86_64::__cpuid_count;

use crate::cpu::isa::interface::system_info::CpuInfoIfce;
use crate::cpu::isa::system_info::{CpuInfo, IsaExtension};

pub struct TscInfo {
    pub invariant: bool,
    pub frequency: u64,
}

pub enum Error {
    MissingValue,
}

impl TscInfo {
    pub fn get() -> Self {
        TscInfo {
            invariant: super::CpuInfo::is_extension_supported(IsaExtension::InvariantTsc),
            frequency: {
                // Determining the TSC frequency differs by vendor
                match CpuInfo::get_vendor().as_str() {
                    "GenuineIntel" => Self::get_tsc_freq_intel(),
                    _ => Self::get_tsc_freq_common(),
                }
            },
        }
    }

    fn get_tsc_freq_common() -> u64 {
        //! Use the PIT to determine the frequency of the TSC
    }

    fn get_tsc_freq_intel() -> u64 {
        let cpuid_15 = unsafe { __cpuid_count(0x15, 0) };
        if cpuid_15.ecx != 0 && cpuid_15.eax != 0 && cpuid_15.ebx != 0 {
            cpuid_15.ecx as u64 * (cpuid_15.eax as u64 / cpuid_15.ebx as u64)
        } else {
            Self::get_tsc_freq_common()
        }
    }
}

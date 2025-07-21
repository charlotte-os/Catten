use alloc::string::String;
use alloc::vec::Vec;
use core::arch::x86_64::__cpuid_count;
use core::mem::transmute;

use crate::isa::interface::system_info::CpuInfoIfce;

pub enum IsaExtension {
    Avx2,
    Avx512,
    Pml5,
}

pub struct CpuInfo;

impl CpuInfoIfce for CpuInfo {
    type IsaExtension = IsaExtension;
    type Model = String;
    type Vendor = String;

    fn get_vendor() -> Self::Vendor {
        unsafe {
            let vendor_string_raw = __cpuid_count(0, 0);
            let utf8 = Vec::from(transmute::<[u32; 3], [u8; 12]>([
                vendor_string_raw.ebx,
                vendor_string_raw.edx,
                vendor_string_raw.ecx,
            ]));
            String::from_utf8(utf8).unwrap()
        }
    }

    fn get_model() -> Self::Model {
        unsafe {
            let cpuid_results = [
                __cpuid_count(0x80000002, 0),
                __cpuid_count(0x80000003, 0),
                __cpuid_count(0x80000004, 0),
            ];
            let utf8 = Vec::from(transmute::<[u32; 12], [u8; 48]>([
                cpuid_results[0].eax,
                cpuid_results[0].ebx,
                cpuid_results[0].ecx,
                cpuid_results[0].edx,
                cpuid_results[1].eax,
                cpuid_results[1].ebx,
                cpuid_results[1].ecx,
                cpuid_results[1].edx,
                cpuid_results[2].eax,
                cpuid_results[2].ebx,
                cpuid_results[2].ecx,
                cpuid_results[2].edx,
            ]));
            // Convert the byte vector to a String, assuming it is valid UTF-8
            // Note: This is safe because the cpuid results are guaranteed to be valid UTF-8
            // as per the AMD64 Architecture Programmer's Manual.
            String::from_utf8(utf8)
                .unwrap()
                .trim_end_matches("\0")
                .into()
        }
    }

    fn get_paddr_sig_bits() -> u8 {
        unsafe {
            let cpuid_result = __cpuid_count(0x80000008, 0);
            cpuid_result.eax as u8
        }
    }

    fn get_vaddr_sig_bits() -> u8 {
        unsafe {
            let cpuid_result = __cpuid_count(0x80000008, 0);
            ((cpuid_result.eax >> 8) & 0xff) as u8
        }
    }

    fn is_extension_supported(extension: Self::IsaExtension) -> bool {
        match extension {
            IsaExtension::Avx2 => unsafe {
                let cpuid_result = __cpuid_count(7, 0);
                (cpuid_result.ebx & 0x20) != 0
            },
            IsaExtension::Avx512 => unsafe {
                let cpuid_result = __cpuid_count(7, 0);
                (cpuid_result.ebx & 0x40000000) != 0
            },
            IsaExtension::Pml5 => unsafe {
                let cpuid_result = __cpuid_count(0x80000008, 0);
                (cpuid_result.ecx & 0x100) != 0
            },
        }
    }
}

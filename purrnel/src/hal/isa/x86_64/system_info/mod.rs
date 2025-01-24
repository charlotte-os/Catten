use core::arch::x86_64::__cpuid_count;
use core::fmt::{self, Debug, Display, Formatter};
use core::mem::transmute_copy;

use crate::hal::isa::interface::system_info::CpuInfoIfce;

pub enum IsaExtension {
    avx2,
    avx512,
    pml5,
}

#[repr(transparent)]
pub struct VendorString([u8; 12]);

impl Display for VendorString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            core::str::from_utf8(&self.0).unwrap_or("<Invalid Vendor String>")
        )
    }
}

impl Debug for VendorString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            core::str::from_utf8(&self.0).unwrap_or("<Invalid Vendor String>")
        )
    }
}

#[repr(transparent)]
pub struct ModelString([u8; 48]);

impl Display for ModelString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", core::str::from_utf8(&self.0).expect("<Invalid Model String>"))
    }
}

pub struct CpuInfo;

impl CpuInfoIfce for CpuInfo {
    type IsaExtension = IsaExtension;
    type Model = ModelString;
    type Vendor = VendorString;

    fn get_vendor() -> Self::Vendor {
        unsafe {
            let vendor_string_raw = __cpuid_count(0, 0);
            core::mem::transmute::<[u32; 3], VendorString>([
                vendor_string_raw.ebx,
                vendor_string_raw.edx,
                vendor_string_raw.ecx,
            ])
        }
    }

    fn get_model() -> Self::Model {
        unsafe {
            let mut brand_string = ModelString { 0: [0u8; 48] };

            let mut cpuid_result = __cpuid_count(0x80000002, 0);
            brand_string.0[0..4].copy_from_slice(transmute_copy(&cpuid_result.eax));
            brand_string.0[4..8].copy_from_slice(transmute_copy(&cpuid_result.ebx));
            brand_string.0[8..12].copy_from_slice(transmute_copy(&cpuid_result.ecx));
            brand_string.0[12..16].copy_from_slice(transmute_copy(&cpuid_result.edx));

            cpuid_result = __cpuid_count(0x80000003, 0);
            brand_string.0[16..20].copy_from_slice(transmute_copy(&cpuid_result.eax));
            brand_string.0[20..24].copy_from_slice(transmute_copy(&cpuid_result.ebx));
            brand_string.0[24..28].copy_from_slice(transmute_copy(&cpuid_result.ecx));
            brand_string.0[28..32].copy_from_slice(transmute_copy(&cpuid_result.edx));

            cpuid_result = __cpuid_count(0x80000004, 0);
            brand_string.0[32..36].copy_from_slice(transmute_copy(&cpuid_result.eax));
            brand_string.0[36..40].copy_from_slice(transmute_copy(&cpuid_result.ebx));
            brand_string.0[40..44].copy_from_slice(transmute_copy(&cpuid_result.ecx));
            brand_string.0[44..48].copy_from_slice(transmute_copy(&cpuid_result.edx));

            brand_string
        }
    }

    fn get_vaddr_sig_bits() -> u8 {
        unsafe {
            let cpuid_result = __cpuid_count(0x80000008, 0);
            cpuid_result.eax as u8
        }
    }

    fn get_paddr_sig_bits() -> u8 {
        unsafe {
            let cpuid_result = __cpuid_count(0x80000008, 0);
            (cpuid_result.eax >> 8) as u8
        }
    }

    fn is_extension_supported(extension: Self::IsaExtension) -> bool {
        match extension {
            IsaExtension::avx2 => unsafe {
                let cpuid_result = __cpuid_count(7, 0);
                (cpuid_result.ebx & 0x20) != 0
            },
            IsaExtension::avx512 => unsafe {
                let cpuid_result = __cpuid_count(7, 0);
                (cpuid_result.ebx & 0x40000000) != 0
            },
            IsaExtension::pml5 => unsafe {
                let cpuid_result = __cpuid_count(0x80000008, 0);
                (cpuid_result.ecx & 0x100) != 0
            },
        }
    }
}

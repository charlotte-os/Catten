#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "aarch64")]
mod aarch64;

pub enum Error {
    UnsupportedByIsa,
}

pub struct SimdExtensions {
    
    // x86_64
    avx2: bool,
    avx_512: bool,
    // aarch64
    neon: bool,
    sve: bool,
    sve2: bool,
    sme: bool,
}

trait CpuInfoIfce {
    fn get_vendor_string(&self) -> [u8; 12];
    fn get_brand_string(&self) -> [u8; 48];
    fn get_vaddr_sig_bits(&self) -> u8;
    fn get_paddr_sig_bits(&self) -> u8;
    fn get_simd_extensions(&self) -> SimdExtensions;
    fn supports_five_level_paging(&self) -> bool;
}
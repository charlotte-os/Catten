struct CpuInfo {
    vendor_string: [u8; 12],
    brand_string: [u8; 48],
    vaddr_sig_bits: u8,
    paddr_sig_bits: u8,
    supports_avx_512: bool,
    supports_pml5: bool,
}
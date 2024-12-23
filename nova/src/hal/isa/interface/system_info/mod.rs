pub enum Error {
    UnsupportedByIsa,
    UnableToDetermine,
}

pub trait CpuInfoIfce {
    type IsaExtension;
    type Vendor;
    type Model;

    fn get_vendor() -> Self::Vendor;
    fn get_brand() -> Self::Model;
    fn get_vaddr_sig_bits() -> u8;
    fn get_paddr_sig_bits() -> u8;
    fn is_extension_supported(extension: Self::IsaExtension) -> bool;
}
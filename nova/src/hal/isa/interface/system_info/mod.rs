pub enum Error {
    UnsupportedByIsa,
    UnableToDetermine,
}

trait CpuInfoIfce {
    type IsaExtension;
    type Vendor;
    type Model;

    fn get_vendor(&self) -> Self::Vendor;
    fn get_brand(&self) -> Self::Model;
    fn get_vaddr_sig_bits(&self) -> u8;
    fn get_paddr_sig_bits(&self) -> u8;
    fn is_extension_supported(&self, extension: Self::IsaExtension) -> bool;
}
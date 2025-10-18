use alloc::string::String;
use crate::cpu::isa::interface::system_info::CpuInfoIfce;

pub enum IsaExtension {
    // TODO: Define RISC-V ISA extensions (e.g., M, A, F, D, C, V, etc.)
    // Common extensions include:
    // - M: Integer Multiplication and Division
    // - A: Atomic Instructions
    // - F: Single-Precision Floating-Point
    // - D: Double-Precision Floating-Point
    // - C: Compressed Instructions
    // - V: Vector Operations
}

pub struct CpuInfo;

impl CpuInfoIfce for CpuInfo {
    type IsaExtension = IsaExtension;
    type Model = String;
    type Vendor = String;

    fn get_vendor() -> Self::Vendor {
        // TODO: Read vendor information from device tree or SBI
        String::from("Unknown RISC-V Vendor")
    }

    fn get_model() -> Self::Model {
        // TODO: Read model information from device tree or SBI
        String::from("Unknown RISC-V Model")
    }

    fn get_paddr_sig_bits() -> u8 {
        // TODO: Determine from hardware capabilities
        // RISC-V typically supports 56-bit physical addresses (Sv39/48/57)
        56
    }

    fn get_vaddr_sig_bits() -> u8 {
        // TODO: Determine based on paging mode (Sv39=39, Sv48=48, Sv57=57)
        // Default to Sv39 for now
        39
    }

    fn is_extension_supported(extension: Self::IsaExtension) -> bool {
        // TODO: Read misa CSR to determine supported extensions
        false
    }
}

use crate::isa::interface::system_info::CpuInfoIfce;
#[derive(Debug)]
pub enum IsaExtension {}
#[derive(Debug)]
pub enum Vendor {
    SwReserved = 0x00,
    Arm = 0x41,
    Broadcom = 0x42,
    Cavium = 0x43,
    Dec = 0x44,
    Fujitsu = 0x46,
    Infineon = 0x49,
    MotorolaFreescale = 0x4d,
    Nvidia = 0x4e,
    Amcc = 0x50,
    Qualcomm = 0x51,
    Marvell = 0x56,
    Intel = 0x69,
    Ampere = 0xc0,
}

impl From<u8> for Vendor {
    fn from(vendor_id: u8) -> Self {
        match vendor_id {
            0x00 => Vendor::SwReserved,
            0x41 => Vendor::Arm,
            0x42 => Vendor::Broadcom,
            0x43 => Vendor::Cavium,
            0x44 => Vendor::Dec,
            0x46 => Vendor::Fujitsu,
            0x49 => Vendor::Infineon,
            0x4d => Vendor::MotorolaFreescale,
            0x4e => Vendor::Nvidia,
            0x50 => Vendor::Amcc,
            0x51 => Vendor::Qualcomm,
            0x56 => Vendor::Marvell,
            0x69 => Vendor::Intel,
            0xc0 => Vendor::Ampere,
            _ => panic!("aarch64 systeminfo: Unrecognized vendor ID!"),
        }
    }
}

#[derive(Debug)]
pub struct Model {
    part_num: u16,
    revision: u8,
    variant: u8,
    architecture: u8,
}

#[derive(Debug)]
pub struct CpuInfo;

impl CpuInfoIfce for CpuInfo {
    type IsaExtension = IsaExtension;
    type Model = Model;
    type Vendor = Vendor;

    fn get_vendor() -> Self::Vendor {
        let mut midr: u64;
        unsafe {
            core::arch::asm!("mrs {}, midr_el1", out(reg) midr);
        }
        let vendor_id = (midr >> 24) as u8;

        vendor_id.into()
    }

    fn get_model() -> Self::Model {
        let mut midr: u64;
        unsafe {
            core::arch::asm!("mrs {}, midr_el1", out(reg) midr);
        }
        let part_num = (midr & 0xfff) as u16;
        let revision = ((midr >> 16) & 0xf) as u8;
        let variant = ((midr >> 20) & 0xf) as u8;
        let architecture = ((midr >> 4) & 0xf) as u8;

        Model {
            part_num,
            revision,
            variant,
            architecture,
        }
    }

    fn get_vaddr_sig_bits() -> u8 {
        let mut id_aa64mmfr2_el1: u64;
        unsafe {
            core::arch::asm!("mrs {}, id_aa64mmfr2_el1", out(reg) id_aa64mmfr2_el1);
        }
        let vaddr_range = ((id_aa64mmfr2_el1 >> 16) & 0xf) as u8;

        match vaddr_range {
            0b0000 => 48, // 48-bit VA
            0b0001 => 52, // 52-bit VA when using the 64KB granule
            0b0010 => 56, // 52-bit VA only when FEAT_D128 is implemented
            _ => panic!("aarch64 systeminfo: Unrecognized virtual address range value!"),
        }
    }

    fn get_paddr_sig_bits() -> u8 {
        let mut id_aa64mmfr0_el1: u64;
        unsafe {
            core::arch::asm!("mrs {}, id_aa64mmfr0_el1", out(reg) id_aa64mmfr0_el1);
        }
        let paddr_range = (id_aa64mmfr0_el1 & 0xf) as u8;

        match paddr_range {
            0b0000 => 32, // 32-bit PA, 4GB
            0b0001 => 36, // 36-bit PA, 64GB
            0b0010 => 40, // 40-bit PA, 1TB
            0b0011 => 42, // 42-bit PA, 4TB
            0b0100 => 44, // 44-bit PA, 16TB
            0b0101 => 48, // 48-bit PA, 256TB
            0b0110 => 52, // 52-bit PA, 4PB when FEAT_LPA is implemented
            0b0111 => 56, // 56-bit PA, 64PB when FEAT_D128 is implemented
            _ => panic!("aarch64 systeminfo: Unrecognized physical address range value!"),
        }
    }

    fn is_extension_supported(_extension: Self::IsaExtension) -> bool {
        false
    }
}

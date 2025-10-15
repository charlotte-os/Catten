#[derive(Debug)]
pub enum IsaExtension {
    // 128-bit page table entries for stage 1 translations
    FeatD128,
    // Large Physical Address Extension for the 64KB granule
    FeatLPA,
    // Large Physical Address Extension for 4KB and 16KB granules
    FeatLPA2,
    // Large Virtual Address Extension 52-bit
    FeatLVA,
    // Large Virtual Address Extension 56-bit
    FeatLVA3,
}

mod check_feat {
    // ARM ARM D24.2.85
    const ID_AA64MMFR1_EL1_D128_SHIFT: u64 = 32;
    const ID_AA64MMFR1_EL1_D128_MASK: u64 = 0b1111 << ID_AA64MMFR1_EL1_D128_SHIFT;
    const ID_AA64MMFR1_EL1_D128_VAL: u64 = 0b0001;

    pub fn d128() -> bool {
        let mut id_aa64mmfr1_el1: u64;
        unsafe {
            core::arch::asm!("mrs {}, id_aa64mmfr1_el1", out(reg) id_aa64mmfr1_el1);
        }
        (id_aa64mmfr1_el1 & ID_AA64MMFR1_EL1_D128_MASK) >> ID_AA64MMFR1_EL1_D128_SHIFT
            == ID_AA64MMFR1_EL1_D128_VAL
    }
    // ARM ARM D24.2.82
    const ID_AA64MMFR0_EL1_LPA_SHIFT: u64 = 0;
    const ID_AA64MMFR0_EL1_LPA_MASK: u64 = 0b1111 << ID_AA64MMFR0_EL1_LPA_SHIFT;
    const ID_AA64MMFR0_EL1_LPA_VAL: u64 = 0b0110;

    pub fn lpa() -> bool {
        let mut id_aa64mmfr0_el1 = 0u64;
        unsafe {
            core::arch::asm!("mrs {}, id_aa64mmfr0_el1", out(reg) id_aa64mmfr0_el1);
        }
        (id_aa64mmfr0_el1 & ID_AA64MMFR0_EL1_LPA_MASK) >> ID_AA64MMFR0_EL1_LPA_SHIFT
            == ID_AA64MMFR0_EL1_LPA_VAL
    }
    // ARM ARM D24.2.82
    const ID_AA64MMFR0_EL1_LPA2_SHIFT: u64 = 28;
    const ID_AA64MMFR0_EL1_LPA2_MASK: u64 = 0b1111 << ID_AA64MMFR0_EL1_LPA2;
    const ID_AA64MMFR0_EL1_LPA2_VAL: u64 = 0b0001;
    pub fn lpa2() -> bool {
        let mut id_aa64mmfr0_el1 = 0u64;
        unsafe {
            core::arch::asm!("mrs {}, id_aa64mmfr0_el1", out(reg) id_aa64mmfr0_el1);
        }
        (id_aa64mmfr0_el1 & ID_AA64MMFR0_EL1_LPA2_MASK) >> ID_AA64MMFR0_EL1_LPA2_SHIFT
            == ID_AA64MMFR0_EL1_LPA2_VAL
    }
}

#[derive(Debug)]
pub enum IsaExtension {
    // 128-bit page table entries for stage 1 translations
    FeatD128,
}

mod check_feat {
    const ID_AA64MMFR1_EL1_D128_MASK: u64 = 0b1111 << 32;

    pub fn d128() -> bool {
        let mut id_aa64mmfr1_el1: u64;
        unsafe {
            core::arch::asm!("mrs {}, id_aa64mmfr1_el1", out(reg) id_aa64mmfr1_el1);
        }
        id_aa64mmfr1_el1 & ID_AA64MMFR1_EL1_D128_MASK << 32 != 0
    }
}

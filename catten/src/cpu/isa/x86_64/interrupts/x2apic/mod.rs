//! # x2APIC Local Advanced Programmable Interrupt Controller

use alloc::vec::Vec;
use core::arch::asm;

pub static mut X2APIC_ID_TABLE: Vec<LapicId> = Vec::new();

pub struct LapicId {
    pub physical: PhysicalLapicId,
    pub logical:  LogicalLapicId,
}

impl LapicId {
    pub fn get_local() -> Self {
        let physical: PhysicalLapicId;
        let logical: u32;
        unsafe {
            asm! (
                "mov ecx, 0x802", // x2APIC ID Register
                "rdmsr",
                "mov [{phys:e}], eax",
                "mov ecx, 0x80d", // x2APIC Logical Destination Register
                "rdmsr",
                "mov [{log:e}], eax",
                phys = out(reg) physical,
                log = out(reg) logical,
            )
        }
        LapicId {
            physical,
            logical: LogicalLapicId {
                cluster_id: ((logical >> 16) & (1 << 16 - 1)) as u16,
                apic_bitmask: (logical & (1 << 16 - 1)) as u16,
            },
        }
    }
}
pub type PhysicalLapicId = u32;
#[repr(C, packed)]
pub struct LogicalLapicId {
    cluster_id: u16,
    apic_bitmask: u16,
}

//! # Local Advanced Programmable Interrupt Controller Driver for x2APIC mode

use alloc::collections::btree_map::BTreeMap;
use core::arch::asm;

use spin::rwlock::RwLock;

use crate::cpu::isa::lp::LpId;

pub static mut X2APIC_ID_TABLE: RwLock<BTreeMap<LpId, LogicalLapicId>> =
    RwLock::new(BTreeMap::new());

/// x2APIC MSR space docs: AAPM 16.11.1 and ISDM 12.12.1.2
pub static X2APIC_ID_REG: u32 = 0x802;
pub static X2APIC_LOGICAL_DEST_REG: u32 = 0x80d;

pub type PhysicalLapicId = u32;
#[repr(C, packed)]
pub struct LogicalLapicId {
    cluster_id: u16,
    apic_bitmask: u16,
}

pub struct X2Apic {
    timer_res: u64,
}

impl X2Apic {
    fn new() -> Self {
        X2Apic {
            timer_res: Self::measure_res(),
        }
    }

    fn measure_res() -> u64 {
        // Placeholder for actual timer resolution measurement logic
        0
    }

    pub fn get_physical_id(&self) -> PhysicalLapicId {
        let apic_id: u32;
        unsafe {
            asm!(
                "rdmsr",
                in("ecx") X2APIC_ID_REG,
                out("eax") apic_id,
                out("edx") _,
            );
        }
        apic_id
    }

    pub fn get_logical_id(&self) -> LogicalLapicId {
        let logical_id: u32;
        unsafe {
            asm!(
                "rdmsr",
                in("ecx") X2APIC_LOGICAL_DEST_REG,
                out("eax") logical_id,
                out("edx") _,
            );
        }
        LogicalLapicId {
            cluster_id: ((logical_id >> 16) & 0xffff) as u16,
            apic_bitmask: (logical_id & 0xffff) as u16,
        }
    }
}

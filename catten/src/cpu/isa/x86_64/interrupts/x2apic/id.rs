use alloc::collections::btree_map::BTreeMap;

use crate::cpu::isa::lp::LpId;

pub(super) static mut X2APIC_ID_TABLE: BTreeMap<LpId, LapicId> = BTreeMap::new();

/// x2APIC MSR space docs: AAPM 16.11.1 and ISDM 12.12.1.2
pub static X2APIC_ID_REG: u32 = 0x802;
pub static X2APIC_LOGICAL_DEST_REG: u32 = 0x80d;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct LapicId {
    pub physical: PhysicalLapicId,
    pub logical:  LogicalLapicId,
}

impl LapicId {
    pub fn get_local() -> Self {
        let physical: PhysicalLapicId;
        let logical: u32;
        unsafe {
            core::arch::asm! {
                "mov ecx, X2APIC_ID_REG", // x2APIC ID Register
                "rdmsr",
                "mov [{phys:e}], eax",
                "mov ecx, X2APIC_LOGICAL_DEST_REG", // x2APIC Logical Destination Register
                "rdmsr",
                "mov [{log:e}], eax",
                phys = out(reg) physical,
                log = out(reg) logical,
            }
        }
        LapicId {
            physical,
            logical: unsafe { core::mem::transmute(logical) },
        }
    }
}
pub(super) type PhysicalLapicId = u32;
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(super) struct LogicalLapicId {
    cluster_id: u16,
    apic_bitmask: u16,
}

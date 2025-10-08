//! # x2APIC Local Advanced Programmable Interrupt Controller

use core::arch::asm;
use core::mem::MaybeUninit;

use hashbrown::HashMap;
use spin::rwlock::RwLock;

use crate::cpu::isa::lp::{LogicalProcessor, LpIfce};

pub static X2APIC_ID_TABLE: RwLock<
    MaybeUninit<HashMap<<LogicalProcessor as LpIfce>::LpId, X2ApicId>>,
> = RwLock::new(MaybeUninit::uninit());

pub fn write_id_mapping() {
    let lp_id = LogicalProcessor::read_lp_id();
    let x2apic_id = X2ApicId::get_local();
    let mut guard = X2APIC_ID_TABLE.write();
    if lp_id == 0 {
        // The BSP initializes the table and adds its entry
        guard.write(HashMap::new()).insert(lp_id, x2apic_id);
    } else {
        unsafe {
            // Safety: By the time APs are started, the BSP has already initialized the table
            guard.assume_init_mut().insert(lp_id, x2apic_id);
        }
    }
}

#[derive(Debug)]
pub struct X2ApicId {
    pub physical: PhysicalX2ApicId,
    pub logical:  LogicalX2ApicId,
}

impl X2ApicId {
    pub fn get_local() -> Self {
        let physical: PhysicalX2ApicId;
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
        X2ApicId {
            physical,
            logical: LogicalX2ApicId {
                cluster_id: ((logical >> 16) & (1 << 16 - 1)) as u16,
                apic_bitmask: (logical & (1 << 16 - 1)) as u16,
            },
        }
    }
}
pub type PhysicalX2ApicId = u32;

#[derive(Debug)]
#[repr(C, packed)]
pub struct LogicalX2ApicId {
    cluster_id: u16,
    apic_bitmask: u16,
}

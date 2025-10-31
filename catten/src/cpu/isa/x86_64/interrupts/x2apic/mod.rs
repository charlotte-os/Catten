//! # Local Advanced Programmable Interrupt Controller Driver for x2APIC mode

use alloc::collections::btree_map::BTreeMap;
use core::arch::asm;

use spin::rwlock::RwLock;

use crate::common::bitwise::DBYTE_MASK;
use crate::common::constants::PS_PER_SEC;
use crate::common::time::ExtDuration;
use crate::cpu::isa::constants::msrs;
use crate::cpu::isa::lp::LpId;
use crate::cpu::isa::timers::tsc::rdtsc;

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
    timer_res: ExtDuration,
}

impl X2Apic {
    fn new() -> Self {
        X2Apic {
            timer_res: Self::measure_timer_res(),
        }
    }

    fn measure_timer_res() -> ExtDuration {
        const SAMPLE_CYCLES: u64 = 100;
        unsafe { msrs::write(msrs::x2apic::TIMER_INITIAL_COUNT_REGISTER, SAMPLE_CYCLES) };
        let start_tsc = rdtsc();
        while unsafe { msrs::read(msrs::x2apic::TIMER_CURRENT_COUNT_REGISTER) } > 0 {}
        let end_tsc = rdtsc();
        let delta_tsc = end_tsc - start_tsc;
        let timer_freq = delta_tsc / SAMPLE_CYCLES;
        let res_ps = PS_PER_SEC / timer_freq;
        ExtDuration::from_ps(res_ps)
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
            cluster_id: ((logical_id >> 16) & DBYTE_MASK as u32) as u16,
            apic_bitmask: (logical_id & DBYTE_MASK as u32) as u16,
        }
    }
}

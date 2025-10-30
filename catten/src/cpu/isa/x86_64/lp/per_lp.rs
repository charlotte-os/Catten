use crate::cpu::scheduler::local_scheduler::LocalScheduler;
use crate::cpu::scheduler::threads::Thread;

pub static LP_DATA_TABLE: Lazy<IdTable<LpId, PerLpDataSegment>> = Lazy::new(IdTable::new);

pub struct PerLpDataSegment {
    pub curr_tcb: Arc<Mutex<Thread>>,
    pub ipi_mailbox: *const ipis::IpiRpc,
    idt: Idt,
    local_scheduler: Mutex<LocalScheduler>,
}

pub extern "C" fn get_per_lp_data_segment() -> &'static PerLpDataSegment {
    unsafe {
        let gs_base: u64;
        asm!(
            "rdgsbase {}", 
            out(reg) gs_base
        );
        &*(gs_base as *const PerLpDataSegment)
    }
}

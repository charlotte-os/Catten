pub use crate::isa::interface::lp_control::core_state::CoreStateIfce;
use crate::memory::pmem::PAddr;

pub struct CoreState {
    cr3: u64,
    rsp: PAddr,
}

impl CoreStateIfce for CoreState {
    extern "C" fn save(&mut self) {
        let rsp_raw: u64;
        unsafe {
            core::arch::asm!(
                // Save the current CR3 value
                "lea {rsp}, [rsp + 8]", // +8 to account for the return address pushed by call
                "mov {cr3}, cr3",
                rsp = out(reg) rsp_raw,
                cr3 = out(reg) self.cr3,
            );
        }
        self.rsp = PAddr::from(rsp_raw);
    }

    extern "C" fn load(&self) {
        unsafe {
            core::arch::asm!(
                // Load the saved CR3 value
                "mov cr3, {cr3}",
                "mov rsp, {rsp}",
                rsp = in(reg) <PAddr as Into<u64>>::into(self.rsp),
                cr3 = in(reg) self.cr3,
            );
        }
    }
}

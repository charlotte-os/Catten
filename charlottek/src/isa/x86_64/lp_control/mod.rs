use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::vec_deque::VecDeque;
use core::arch::{asm, naked_asm};

use crate::isa::interface::lp_control::LpControlIfce;

static mut TEMP_STATE: BTreeMap<<LpControl as LpControlIfce>::LpId, LpState> = BTreeMap::new();

pub enum Error {}

pub struct LpControl;

impl LpControlIfce for LpControl {
    type Error = Error;
    // The logical processor ID is a 32-bit value on x86_64, representing the xAPIC ID in x2APIC
    // mode.
    type LpId = u32;
    type LpState = LpState;

    #[inline(always)]
    fn halt() -> ! {
        unsafe {
            asm!("hlt");
        }
        loop {}
    }

    #[inline(always)]
    fn mask_interrupts() {
        unsafe {
            asm!("cli");
        }
    }

    #[inline(always)]
    fn unmask_interrupts() {
        unsafe {
            asm!("sti");
        }
    }

    #[inline(always)]
    fn get_lp_id() -> Self::LpId {
        let lp_id: Self::LpId;
        // Read the LAPIC ID using the x2APIC MSR interface.
        unsafe {
            asm!(
                "mov ecx, 0x802",
                "rdmsr",
                out("eax") lp_id
            );
        }
        lp_id
    }
    #[inline(always)]
    fn save_lp_state() -> Box<LpState> {
        unsafe {
            asm_save_lp_state();
            let lp_state = Box::new(temp_lp_state);
            temp_lp_state_me = 1; // Indicate that the state has been saved.
            lp_state
        }
    }

    #[inline(always)]
    fn load_lp_state(state: Box<LpState>) {
        unsafe {
            // acquire the temporary state mutex
            asm!("spin:",
                "cmpxchg  temp_lp_state_me, 0, 1",
                "jne spin"
            );
            temp_lp_state = *state;
            asm_load_lp_state();
            // release the temporary state mutex
            temp_lp_state_me = 0;
        }
}

unsafe extern "C" {
    static temp_lp_state: LpState;
    static mut temp_lp_state_me: u8;
    fn asm_save_lp_state();
}

const GPR_COUNT: usize = 16; // Number of general-purpose registers on x86_64.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LpState {
    gprs: [u64; GPR_COUNT],
    rip: u64,
    rflags: u64,
}

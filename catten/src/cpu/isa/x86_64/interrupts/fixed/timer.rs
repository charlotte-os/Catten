use crate::cpu::isa::interface::lp::LpIsaDataIfce;
use crate::cpu::isa::interface::timers::LpTimerIfce;
use crate::cpu::multiprocessor::lp_local::LpLocal;
use crate::{get_lp_id, logln};

unsafe extern "C" {
    pub unsafe fn isr_lp_timer();
}
#[rustfmt::skip]
core::arch::global_asm!(
    include_str!("../../asm_macros/context_switch.asm"),
    ".global isr_lp_timer",
    "isr_lp_timer:",
    "m_save_gprs",
    "call ih_lp_timer",
    "m_restore_gprs",
    "iretq"
);

#[unsafe(no_mangle)]
pub extern "C" fn ih_lp_timer() {
    logln!("LP{}: Timer interrupt has occurred.\nRearming timer.", (get_lp_id!()));
    let timer = LpLocal::get_mut().isa_data.get_lp_timer();
    timer.signal_eoi();
    let _ = timer.reset();
    logln!("LP{}: Timer rearmed.", (get_lp_id!()));
}

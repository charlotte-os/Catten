pub mod context_switch;
pub mod exceptions;
pub mod idt;
pub mod ipis;
pub mod x2apic;

use idt::*;
use spin::{Lazy, Mutex};

use crate::cpu::isa::init::gdt;
use crate::cpu::isa::interface::interrupts::InterruptManagerIfce;
use crate::cpu::isa::lp::LpId;
use crate::memory::IdTable;

pub static BSP_IDT: Mutex<Idt> = Mutex::new(Idt::new());
pub static IDT_TABLE: Lazy<IdTable<LpId, Mutex<Idt>>> = Lazy::new(IdTable::new);

pub fn register_fixed_isr_gates(idt: &mut Idt) {
    exceptions::load_exceptions(idt);
}

pub struct IsrDesc {
    pub target_lp: LpId,
    pub vector: u8,
    pub handler: extern "C" fn(),
}

pub struct InterruptManager;

pub enum Error {}

impl InterruptManagerIfce for InterruptManager {
    type Error = Error;
    type IsrDesc = IsrDesc;
    type LocalIntCtlr = x2apic::X2Apic;

    fn init_interrupt_structures() -> Result<(), Self::Error> {
        todo!()
    }

    fn register_interrupt_handler(isrd: &Self::IsrDesc) -> Result<(), Self::Error> {
        todo!()
    }
}

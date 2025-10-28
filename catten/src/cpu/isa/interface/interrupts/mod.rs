use crate::cpu::isa::lp::LpId;
pub trait InterruptManagerIfce {
    type Error;
    type Ipi;
    type IsrDesc;
    /// Initialize interrupt structures (IDT, IVT, etc.)
    fn init_interrupt_structures() -> Result<(), Self::Error>;
    /// Initialize local interrupt controller (e.g., LAPIC, GIC, CLINT, IMSIC, etc.)
    fn init_local_interrupt_controller() -> Result<(), Self::Error>;
    /// Initialize system interrupt controller (e.g., IOAPIC, GIC Distributor, APLIC, etc.)
    fn init_system_interrupt_controller() -> Result<(), Self::Error>;
    /// Send the specified inter-processor interrupt (IPI) to the specified logical processors
    fn send_ipi(lp_list: &[LpId], ipi: Self::Ipi);
    /// Register an interrupt handler using an ISA specific descriptor for where to install it and
    /// with what attributes
    fn register_interrupt_handler(isrd: &Self::IsrDesc) -> Result<(), Self::Error>;
}

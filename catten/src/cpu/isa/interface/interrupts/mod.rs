use crate::cpu::isa::lp::LpId;
pub trait InterruptManagerIfce {
    type Error;
    type IsrDesc;
    type LocalIntCtlr: LocalIntCtlr;
    /// Initialize interrupt structures (IDT, IVT, etc.)
    fn init_interrupt_structures() -> Result<(), Self::Error>;
    /// Register an interrupt handler using an ISA specific descriptor for where to install it and
    /// with what attributes
    fn register_interrupt_handler(isrd: &Self::IsrDesc) -> Result<(), Self::Error>;
}

/// # Local Interrupt Controller Interface
pub trait LocalIntCtlr {
    type Error;
    /// Initialize the local interrupt controller
    fn init() -> Result<(), Self::Error>;
    /// Send an inter-processor interrupt to the specified logical processor
    fn send_unicast_ipi(target_lp: LpId) -> Result<(), Self::Error>;
}

//! # Inter-Processor Interrupts (IPIs) on the x86_64 Architecture
//!
//! The charlottek IPI protocol is designed to work using remote procedure calls (RPCs).
//! This allows for a flexible and extensible way to send IPIs between processors.
//! The protocol supports both unicast (single target) and multicast (multiple targets) IPIs.
//! The implementation is kept as similar as possible across different architectures within reason.
//!
//! Each logical processor (LP) has it's own IPI mailbox, which is contains an enum with the IPI
//! type and arguments. Sending an IPI involves writing to the target LP's mailbox and then
//! triggering the IPI via the architecture-specific mechanism. Receiving an IPI involves checking
//! the mailbox and executing the corresponding handler. If the IPI is multicast, the first argument
//! passed is a pointer to the completion barrier, which is used to signal when all target LPs have
//! completed handling the IPI. This is important for ensuring that all target LPs have completed
//! the requested operation before any of them return from the ISR.

enum UnicastIpi {
    // Halt the target processor in a loop such that it handles interrupts but does not execute
    // any threads until it is woken up by a Wake IPI.
    SoftSleep,
    // Wake up and call the scheduler to start executing threads.
    Wake,
}

#[unsafe_(no_mangle)]
pub extern "C" fn ih_unicast_ipi() {}

enum MulticastIpi {
    VMemInval(usize, usize, usize),
    AsidInval(usize),
}

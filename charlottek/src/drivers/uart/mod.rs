//! # Universal Asynchronous Receiver/Transmitter (UART) Subsystem
//!
//! This subsystem provides a common interface for UART devices and contains drivers for specific
//! UART devices that implement this interface.
//! # UART Drivers
pub mod ns16550;

use core::fmt::Write;
use core::marker::Sized;

use crate::isa::io::IoReg8;

pub trait Uart: Write + Sized {
    type Error: Sized;
    fn try_new(base: IoReg8) -> Result<Self, Self::Error>;
}

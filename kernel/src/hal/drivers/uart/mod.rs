//! # Universal Asynchronous Receiver/Transmitter (UART) Subsystem
//!
//! This subsystem provides a common interface for UART devices and contains drivers for specific UART devices that
//! implement this interface.

// 16550 UART
pub mod uart_16550;

use core::fmt::Write;

use crate::hal::isa::io::IoReg8;

pub trait Uart: Write + Sized {
        type Error: Sized;
        fn try_new(base: IoReg8) -> Result<Self, Self::Error>;
}

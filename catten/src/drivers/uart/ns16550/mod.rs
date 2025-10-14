//! # National Semiconductor 16550 Compatible UART Driver
mod legacy_ports;

use core::fmt::{self, Write};
use core::result::Result;

use spin::{Lazy, Mutex};

use crate::common::io::Read;
use crate::cpu::isa::interface::io::{IReg8Ifce, OReg8Ifce};
use crate::cpu::isa::io::{self, IoReg8};
use crate::drivers::uart::Uart;

#[cfg(target_arch = "x86_64")]
pub static LOG_PORT: Lazy<Mutex<Uart16550>> =
    Lazy::new(|| Mutex::new(Uart16550::try_new(io::IoReg8::IoPort(legacy_ports::COM1)).unwrap()));

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Uart16550 {
    base: IoReg8,
}
#[derive(Debug, Clone, Copy)]
pub enum Error {
    FailedSelfTest,
}

impl Uart16550 {
    fn is_transmit_empty(&self) -> i32 {
        ((self.base + 5).read() & 0x20).into()
    }

    fn received(&self) -> bool {
        ((self.base + 5).read() & 1) != 0
    }

    fn read_char(&self) -> char {
        while !self.received() {}
        (self.base).read() as char
    }
}

impl Uart for Uart16550 {
    type Error = Error;

    fn try_new(base: IoReg8) -> Result<Self, Error> {
        let port = Uart16550 {
            base: base,
        };
        (port.base + 1).write(0x00); // Disable all interrupts
        (port.base + 3).write(0x80); // Enable DLAB (set baud rate divisor)
        (port.base + 0).write(0x01); // Set divisor to 1 (lo byte) 115200 baud
        (port.base + 1).write(0x00); //                  (hi byte)
        (port.base + 3).write(0x03); // 8 bits, no parity, one stop bit
        (port.base + 2).write(0xc7); // Enable FIFO, clear them, with 14-byte threshold
        (port.base + 4).write(0x0b); // IRQs enabled, RTS/DSR set
        (port.base + 4).write(0x1e); // Set in loopback mode, test the serial chip
        (port.base + 0).write(0xae); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        if port.base.read() != 0xae {
            Err(Error::FailedSelfTest)
        } else {
            (port.base + 4).write(0x0f);
            Ok(port)
        }
    }
}

impl Write for Uart16550 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c)?
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        while self.is_transmit_empty() == 0 {}
        if c.is_ascii() {
            if c == '\n' {
                (self.base).write('\r' as u8);
                (self.base).write('\n' as u8);
            } else {
                (self.base).write(c as u8);
            }
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

impl Read for Uart16550 {
    fn read(&mut self, buf: &mut [u8]) -> usize {
        for i in 0..buf.len() {
            buf[i] = self.read_char() as u8;
        }
        buf.len()
    }
}

unsafe impl Send for Uart16550 {}

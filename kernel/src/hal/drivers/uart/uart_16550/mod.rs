use core::fmt::{self, Write};

use spin::{Lazy, Mutex};

use crate::hal::isa::io::interface::{IReg8Ifce, OReg8Ifce};
use crate::hal::isa::io::{self, IoReg8};

#[cfg(target_arch = "x86_64")]
pub static LOG_PORT: Lazy<Mutex<SerialPort>> =
        Lazy::new(|| Mutex::new(SerialPort::try_new(io::IoReg8::IoPort(COM1)).unwrap()));

// Serial port base addresses
static COM1: u16 = 0x3f8;
static COM2: u16 = 0x2f8;
static COM3: u16 = 0x3e8;
static COM4: u16 = 0x2e8;
static COM5: u16 = 0x5f8;
static COM6: u16 = 0x4f8;
static COM7: u16 = 0x5e8;
static COM8: u16 = 0x4e8;
/* // Serial port register offsets
static read_write: u16 = 0; // Data register
static interrupt_enable: u16 = 1; // Interrupt enable register
static divisor_lsb: u16 = 0; // Divisor latch LSB
static divisor_msb: u16 = 1; // Divisor latch MSB
static interrupt_identification: u16 = 2; // Interrupt identification register
static fifo_control: u16 = 2; // FIFO control register
static line_control: u16 = 3; // Line control register, MSB is the DLAB bit
static modem_control: u16 = 4; // Modem control register
static line_status: u16 = 5; // Line status register
static modem_status: u16 = 6; // Modem status register
static scratch: u16 = 7; // Scratch register */

#[derive(Copy, Clone, Debug)]
pub struct SerialPort {
        base: IoReg8,
}

impl SerialPort {
        pub fn try_new(base: IoReg8) -> Option<Self> {
                let port = SerialPort { base: base };
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
                        None
                } else {
                        (port.base + 4).write(0x0f);
                        Some(port)
                }
        }

        fn is_transmit_empty(&self) -> i32 {
                ((self.base + 5).read() & 0x20).into()
        }

        fn received(&self) -> bool {
                ((self.base + 5).read() & 1) != 0
        }
}

impl Write for SerialPort {
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

unsafe impl Send for SerialPort {}

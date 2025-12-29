//! Legacy PC COM Port I/O port bases
/*
 * These won't work on modern hardware as it no longer uses the ISA bus however they are useful
 * for logging when running under hypervisors like QEMU or Bochs which emulate ISA hardware.
 * Typically modern hardware exposes UARTs via MMIO, PCIe, or USB.
 */
#[allow(unused)]
pub static COM1: u16 = 0x3f8;
#[allow(unused)]
pub static COM2: u16 = 0x2f8;
#[allow(unused)]
pub static COM3: u16 = 0x3e8;
#[allow(unused)]
pub static COM4: u16 = 0x2e8;
#[allow(unused)]
pub static COM5: u16 = 0x5f8;
#[allow(unused)]
pub static COM6: u16 = 0x4f8;
#[allow(unused)]
pub static COM7: u16 = 0x5e8;
#[allow(unused)]
pub static COM8: u16 = 0x4e8;

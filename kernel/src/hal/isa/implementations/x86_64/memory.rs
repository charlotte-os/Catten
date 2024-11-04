use super::instructions::{inb, outb};

pub struct Memory;

impl crate::hal::isa::interfaces::memory::Memory for Memory {
    type VAddr = VAddr;
    type PAddr = PAddr;
    type IoAddr = IoAddr;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct VAddr {
    raw: usize
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum IoAddr {
    IoPort(u16),
    Mmio(u64),
}

impl IoAddr {
    pub fn new_port(port: u16) -> Self {
        IoAddr::IoPort(port)
    }
    pub fn new_mmio(addr: u64) -> Self {
        IoAddr::Mmio(addr)
    }
    pub fn writeb(&self, value: u8) {
        match self {
            IoAddr::IoPort(port) => {
                unsafe {
                    outb(*port, value as u8);
                }
            }
            IoAddr::Mmio(addr) => {
                unsafe {
                    core::ptr::write_volatile(*addr as *mut u8, value);
                }
            }
        }
    }
    pub fn readb(&self) -> u8 {
        match self {
            IoAddr::IoPort(port) => {
                unsafe { inb(*port) }
            }
            IoAddr::Mmio(addr) => {
                unsafe { core::ptr::read_volatile(*addr as *const u8) }
            }
        }
    }
}
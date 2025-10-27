use core::arch::asm;
use core::ops::Add;

pub use crate::cpu::isa::interface::io::{IReg8Ifce, OReg8Ifce};
use crate::memory::PAddr;
use crate::memory::physical::PhysicalAddress;

#[derive(Copy, Clone, Debug)]
pub enum IoReg8 {
    IoPort(u16),
    Mmio(PAddr),
}

impl IReg8Ifce for IoReg8 {
    unsafe fn read(&self) -> u8 {
        match self {
            IoReg8::IoPort(port) => {
                let value: u8;
                unsafe {
                    asm!(
                        "in al, dx",
                        in("dx") *port,
                        out("al") value,
                    );
                }
                value
            }
            IoReg8::Mmio(address) => unsafe { core::ptr::read_volatile(address.into_hhdm_ptr()) },
        }
    }
}

impl OReg8Ifce for IoReg8 {
    unsafe fn write(&self, value: u8) {
        match self {
            IoReg8::IoPort(port) => unsafe {
                asm!(
                    "out dx, al",
                    in("dx") *port,
                    in("al") value,
                );
            },
            IoReg8::Mmio(address) => unsafe {
                core::ptr::write_volatile(address.into_hhdm_mut(), value)
            },
        }
    }
}

impl Add<u16> for IoReg8 {
    type Output = IoReg8;

    fn add(self, rhs: u16) -> Self::Output {
        match self {
            IoReg8::IoPort(port) => IoReg8::IoPort(port + rhs),
            IoReg8::Mmio(address) => IoReg8::Mmio(address + rhs as usize),
        }
    }
}

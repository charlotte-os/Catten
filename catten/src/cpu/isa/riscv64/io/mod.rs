use crate::cpu::isa::interface::io::{IReg8Ifce, OReg8Ifce};
use core::ops::Add;

#[derive(Copy, Clone, Debug)]
pub enum IoReg8 {
    // RISC-V uses memory-mapped I/O exclusively (no I/O ports like x86)
    Mmio(*mut u8),
}

impl IReg8Ifce for IoReg8 {
    fn read(&self) -> u8 {
        match self {
            IoReg8::Mmio(address) => unsafe { core::ptr::read_volatile(*address) },
        }
    }
}

impl OReg8Ifce for IoReg8 {
    fn write(&self, value: u8) {
        match self {
            IoReg8::Mmio(address) => unsafe { core::ptr::write_volatile(*address, value) },
        }
    }
}

impl Add<u16> for IoReg8 {
    type Output = IoReg8;

    fn add(self, rhs: u16) -> Self::Output {
        match self {
            IoReg8::Mmio(address) => {
                IoReg8::Mmio(unsafe { (address as *mut u8).add(rhs as usize) as *mut u8 })
            }
        }
    }
}

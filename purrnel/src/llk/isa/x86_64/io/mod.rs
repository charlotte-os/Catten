use core::arch::asm;
use core::ops::Add;

use crate::llk::isa::interface::io::{IReg8Ifce, OReg8Ifce};

#[derive(Copy, Clone, Debug)]
pub enum IoReg8 {
    IoPort(u16),
    Mmio(*mut u8),
}

impl IReg8Ifce for IoReg8 {
    fn read(&self) -> u8 {
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
            IoReg8::Mmio(address) => unsafe { core::ptr::read_volatile(*address) },
        }
    }
}

impl OReg8Ifce for IoReg8 {
    fn write(&self, value: u8) {
        match self {
            IoReg8::IoPort(port) => unsafe {
                asm!(
                    "out dx, al",
                    in("dx") *port,
                    in("al") value,
                );
            },
            IoReg8::Mmio(address) => unsafe { core::ptr::write_volatile(*address, value) },
        }
    }
}

impl Add<u16> for IoReg8 {
    type Output = IoReg8;

    fn add(self, rhs: u16) -> Self::Output {
        match self {
            IoReg8::IoPort(port) => IoReg8::IoPort(port + rhs),
            IoReg8::Mmio(address) => IoReg8::Mmio(unsafe { (address as *mut u8).add(rhs as usize) as *mut u8 }),
        }
    }
}

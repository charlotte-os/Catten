use core::arch::asm;
use core::ops::Add;

use super::interface;

#[derive(Copy, Clone, Debug)]
pub enum IoRegister8 {
    IoPort(u16),
    Mmio(*mut u8),
}

impl interface::IRegister8 for IoRegister8 {
    fn read(&self) -> u8 {
        match self {
            IoRegister8::IoPort(port) => {
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
            IoRegister8::Mmio(address) => unsafe { core::ptr::read_volatile(*address) },
        }
    }
}

impl interface::ORegister8 for IoRegister8 {
    fn write(&self, value: u8) {
        match self {
            IoRegister8::IoPort(port) => {
                unsafe {
                    asm!(
                        "out dx, al",
                        in("dx") *port,
                        in("al") value,
                    );
                }
            }
            IoRegister8::Mmio(address) => unsafe { core::ptr::write_volatile(*address, value) },
        }
    }
}

impl Add<u16> for IoRegister8 {
    type Output = IoRegister8;

    fn add(self, rhs: u16) -> Self::Output {
        match self {
            IoRegister8::IoPort(port) => IoRegister8::IoPort(port + rhs),
            IoRegister8::Mmio(address) => IoRegister8::Mmio(unsafe {
                (address as *mut u8).add(rhs as usize) as *mut u8
            }),
        }
    }
}
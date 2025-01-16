use core::ops::Add;

use crate::hal::isa::interface::io::{IReg8Ifce, OReg8Ifce};

#[derive(Copy, Clone, Debug)]
pub struct IoReg8 {
    address: *mut u8,
}

impl IReg8Ifce for IoReg8 {
    fn read(&self) -> u8 {
        unsafe { core::ptr::read_volatile(self.address) }
    }
}

impl OReg8Ifce for IoReg8 {
    fn write(&self, value: u8) {
        unsafe { core::ptr::write_volatile(self.address, value) }
    }
}

impl Add<u16> for IoReg8 {
    type Output = IoReg8;

    fn add(self, rhs: u16) -> Self::Output {
        IoReg8 {
            address: unsafe { (self.address as *mut u8).add(rhs as usize) as *mut u8 },
        }
    }
}

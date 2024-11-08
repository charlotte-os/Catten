use core::ops::Add;

use super::interface;

#[derive(Copy, Clone, Debug)]
pub struct IoRegister8 {
    address: *mut u8
}

impl interface::IRegister8 for IoRegister8 {
    fn read(&self) -> u8 {
        unsafe { core::ptr::read_volatile(self.address) }
    }
}

impl interface::ORegister8 for IoRegister8 {
    fn write(&self, value: u8) {
        unsafe { core::ptr::write_volatile(self.address, value) }
    }
}

impl Add<u16> for IoRegister8 {
    type Output = IoRegister8;

    fn add(self, rhs: u16) -> Self::Output {
        IoRegister8 {
            address: unsafe { (self.address as *mut u8).add(rhs as usize) as *mut u8 }
        }
    }
}
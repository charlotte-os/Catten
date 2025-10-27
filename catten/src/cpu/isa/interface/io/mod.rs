pub trait IReg8Ifce {
    unsafe fn read(&self) -> u8;
}

pub trait OReg8Ifce {
    unsafe fn write(&self, value: u8);
}

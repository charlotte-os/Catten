pub trait IReg8Ifce {
    fn read(&self) -> u8;
}

pub trait OReg8Ifce {
    fn write(&self, value: u8);
}

pub trait IRegister8 {
    fn read(&self) -> u8;
}

pub trait ORegister8 {
    fn write(&self, value: u8);
}
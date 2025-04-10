pub trait TryClone {
    type Error;
    fn try_clone(&self) -> Result<Self, Self::Error>;
}

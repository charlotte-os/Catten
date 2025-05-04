pub trait TryClone: Sized {
    type Error;
    fn try_clone(&self) -> Result<Self, Self::Error>;
}

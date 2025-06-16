pub trait InterruptManagerIfce {
    type Error;

    fn init_interrupt_strucutures() -> Result<(), Self::Error>;
}

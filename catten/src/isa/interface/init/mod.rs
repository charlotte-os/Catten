///! # Interface for ISA specific initialization

pub trait InitInterface {
    type Error: core::fmt::Debug;
    /// Perform ISA specific processor and system initialization
    fn init_bsp() -> Result<(), Self::Error>;
    /// Perform ISA specific application processor initialization
    fn init_ap() -> Result<(), Self::Error>;
    /// Perform ISA specific deinitialization
    fn deinit() -> Result<(), Self::Error>;
}

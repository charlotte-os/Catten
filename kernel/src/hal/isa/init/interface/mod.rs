///! # Interface for ISA specific initialization

pub trait InitInterface {
    type Error: core::fmt::Debug;
    /// Perform ISA specific initialization
    fn init()->Result<(), Self::Error>;
    /// Perform ISA specific deinitialization
    fn deinit()->Result<(), Self::Error>;
}
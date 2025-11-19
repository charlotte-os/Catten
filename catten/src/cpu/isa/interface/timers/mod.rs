pub use crate::common::time::duration::ExtDuration;

pub enum LicTimerError {
    TimerNotPresent,
    DivisorNotSupported,
    DurationOutOfRange,
    DurationNotSet,
    TimerNotStarted,
    TimerAlreadyStarted,
}

pub trait LicTimer {
    //! # Local Interrupt Controller Timer Interface

    type Divisor;
    type Duration: Into<ExtDuration>;
    type IntDispatchNum;

    fn get_resolution(&self) -> Result<ExtDuration, LicTimerError>;
    fn set_divisor(&mut self, divisor: Self::Divisor) -> Result<(), LicTimerError>;
    fn set_duration(&mut self, duration: Self::Duration) -> Result<(), LicTimerError>;
    fn get_duration(&self) -> Result<Self::Duration, LicTimerError>;
    fn start(&mut self) -> Result<(), LicTimerError>;
    fn stop(&mut self) -> Result<(), LicTimerError>;
    fn reset(&mut self) -> Result<(), LicTimerError>;
    fn get_interrupt_mask(&mut self) -> Result<bool, LicTimerError>;
    fn toggle_interrupt_mask(&mut self, mask: bool) -> Result<(), LicTimerError>;
    extern "C" fn signal_eoi(&mut self) -> i32;
    fn set_isr_dispatch_number(&mut self, num: Self::IntDispatchNum) -> Result<(), LicTimerError>;
}

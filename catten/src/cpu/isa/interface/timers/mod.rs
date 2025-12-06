pub use crate::common::time::duration::ExtDuration;

pub enum LpTimerError {
    TimerNotPresent,
    DivisorNotSupported,
    DurationOutOfRange,
    DurationNotSet,
    TimerNotStarted,
    TimerAlreadyStarted,
    TimerStartsAutomatically,
}

pub trait LpTimerIfce {
    //! # Local Interrupt Controller Timer Interface

    const NAME: &'static str;

    type Divisor;
    type TickCount;
    type IntDispatchNum;

    fn get_resolution(&self) -> Result<ExtDuration, LpTimerError>;
    fn set_divisor(&mut self, divisor: Self::Divisor) -> Result<(), LpTimerError>;
    fn set_duration(&mut self, duration: ExtDuration) -> Result<(), LpTimerError>;
    fn get_duration(&self) -> Result<ExtDuration, LpTimerError>;
    fn start(&mut self) -> Result<(), LpTimerError>;
    fn stop(&mut self) -> Result<(), LpTimerError>;
    fn reset(&mut self) -> Result<(), LpTimerError>;
    fn get_interrupt_mask(&mut self) -> Result<bool, LpTimerError>;
    fn set_interrupt_mask(&mut self, mask: bool) -> Result<(), LpTimerError>;
    extern "C" fn signal_eoi(&mut self) -> i32;
    fn set_isr_dispatch_number(&mut self, num: Self::IntDispatchNum) -> Result<(), LpTimerError>;
}

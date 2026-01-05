use crate::cpu::isa::interface::timers::LpTimerIfce;

pub type LpTimer = GenericTimer;

pub struct GenericTimer;

impl LpTimerIfce for GenericTimer {
    type Divisor = u32;
    type IntDispatchNum = u32;
    type TickCount = u64;

    const NAME: &'static str = "AArch64 Generic Timer";

    fn get_resolution(
        &self,
    ) -> Result<
        crate::common::time::duration::ExtDuration,
        crate::cpu::isa::interface::timers::LpTimerError,
    > {
        // Implementation here
        todo!()
    }

    fn set_divisor(
        &mut self,
        divisor: Self::Divisor,
    ) -> Result<(), crate::cpu::isa::interface::timers::LpTimerError> {
        // Implementation here
        todo!()
    }

    fn set_duration(
        &mut self,
        duration: crate::common::time::duration::ExtDuration,
    ) -> Result<(), crate::cpu::isa::interface::timers::LpTimerError> {
        // Implementation here
        todo!()
    }

    fn get_duration(
        &self,
    ) -> Result<
        crate::common::time::duration::ExtDuration,
        crate::cpu::isa::interface::timers::LpTimerError,
    > {
        // Implementation here
        todo!()
    }

    fn start(&mut self) -> Result<(), crate::cpu::isa::interface::timers::LpTimerError> {
        // Implementation here
        todo!()
    }

    fn stop(&mut self) -> Result<(), crate::cpu::isa::interface::timers::LpTimerError> {
        // Implementation here
        todo!()
    }

    fn reset(&mut self) -> Result<(), crate::cpu::isa::interface::timers::LpTimerError> {
        // Implementation here
        todo!()
    }

    fn get_interrupt_mask(
        &mut self,
    ) -> Result<bool, crate::cpu::isa::interface::timers::LpTimerError> {
        // Implementation here
        todo!()
    }

    fn set_interrupt_mask(
        &mut self,
        mask: bool,
    ) -> Result<(), crate::cpu::isa::interface::timers::LpTimerError> {
        // Implementation here
        todo!()
    }

    extern "C" fn signal_eoi(&mut self) -> i32 {
        // Implementation here
        todo!()
    }

    fn set_isr_dispatch_number(
        &mut self,
        num: Self::IntDispatchNum,
    ) -> Result<(), crate::cpu::isa::interface::timers::LpTimerError> {
        // Implementation here
        todo!()
    }
}

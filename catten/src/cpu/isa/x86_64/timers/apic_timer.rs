use alloc::vec::Vec;

use super::tsc::TSC_CYCLE_PERIOD;
use crate::common::time::duration::ExtDuration;
use crate::cpu::isa::interface::interrupts::LocalIntCtlrIfce;
use crate::cpu::isa::interface::timers::{LpTimerError, LpTimerIfce};
use crate::cpu::isa::interrupts::x2apic::X2Apic;
//use crate::cpu::isa::interrupts::x2apic::X2Apic;
use crate::cpu::isa::timers::tsc::rdtsc;
use crate::cpu::isa::x86_64::lp::msrs;

pub type LpTimer = ApicTimer;

/// # Timer Divisors for the Local APIC Timer
#[repr(u64)]
pub enum ApicTimerDivisors {
    DivBy2 = 0b0000,
    DivBy4 = 0b0001,
    DivBy8 = 0b0010,
    DivBy16 = 0b0011,
    DivBy32 = 0b1000,
    DivBy64 = 0b1001,
    DivBy128 = 0b1010,
    DivBy1 = 0b1011,
}

pub struct ApicTimer {
    resolution:  ExtDuration,
    reset_value: <Self as LpTimerIfce>::TickCount,
}

impl ApicTimer {
    pub fn set_timer_initial_count(count: u32) {
        unsafe {
            msrs::write(msrs::APIC_TIMER_INITIAL_COUNT, count as u64);
        }
    }

    pub fn read_timer_current_count() -> u32 {
        unsafe { msrs::read(msrs::APIC_TIMER_CURRENT_COUNT) as u32 }
    }

    fn determine_timer_resolution(&mut self) {
        const SAMPLE_TICKS: u32 = 10_000_000;
        const NUM_SAMPLES: usize = 100;

        let mut samples = Vec::<u128>::new();
        let _ = self.set_interrupt_mask(true);
        let _ = Self::set_divisor(self, ApicTimerDivisors::DivBy1);
        for _ in 0..NUM_SAMPLES {
            Self::set_timer_initial_count(SAMPLE_TICKS);
            let tsc_start = rdtsc();
            while Self::read_timer_current_count() > 0 {}
            let tsc_end = rdtsc();
            let duration = (tsc_end - tsc_start) as u128 * (*TSC_CYCLE_PERIOD).as_picos();
            let apic_timer_duration = duration / SAMPLE_TICKS as u128;
            samples.push(apic_timer_duration);
        }
        let mut sum = 0u128;
        for sample in samples.iter() {
            sum += *sample;
        }
        let ps = sum / NUM_SAMPLES as u128;
        self.resolution = ExtDuration::from_picos(ps);
    }

    pub fn new(interrupt_vector: <ApicTimer as LpTimerIfce>::IntDispatchNum) -> Self {
        let mut t = ApicTimer {
            resolution:  ExtDuration::from_secs(0),
            reset_value: 0,
        };
        t.determine_timer_resolution();
        t.set_divisor(ApicTimerDivisors::DivBy1);
        t.set_isr_dispatch_number(interrupt_vector);
        t
    }
}

impl LpTimerIfce for ApicTimer {
    type Divisor = ApicTimerDivisors;
    type IntDispatchNum = u8;
    type TickCount = u32;

    const NAME: &'static str = "x86-64 x2APIC Timer";

    fn get_resolution(&self) -> Result<ExtDuration, LpTimerError> {
        Ok(self.resolution)
    }

    fn set_divisor(&mut self, divisor: Self::Divisor) -> Result<(), LpTimerError> {
        unsafe {
            msrs::write(msrs::APIC_TIMER_DIVIDE_CONFIGURATION, divisor as u64);
        }
        Ok(())
    }

    fn set_duration(&mut self, duration: ExtDuration) -> Result<(), LpTimerError> {
        self.reset_value = (duration.as_picos() / (self.resolution.as_picos())
            + if duration.as_picos() % self.resolution.as_picos() > 0 {
                1
            } else {
                0
            })
        .try_into()
        .map_err(|_| LpTimerError::DurationOutOfRange)?;
        Ok(())
    }

    fn get_duration(&self) -> Result<ExtDuration, LpTimerError> {
        let current_count = Self::read_timer_current_count();
        let duration = ExtDuration::from_picos(current_count as u128 * self.resolution.as_picos());
        Ok(duration)
    }

    fn start(&mut self) -> Result<(), LpTimerError> {
        if self.reset_value == 0 {
            return Err(LpTimerError::DurationNotSet);
        } else if Self::read_timer_current_count() != 0 {
            return Err(LpTimerError::TimerAlreadyStarted);
        } else {
            self.reset()
        }
    }

    fn stop(&mut self) -> Result<(), LpTimerError> {
        if Self::read_timer_current_count() == 0 {
            return Err(LpTimerError::TimerNotStarted);
        } else {
            Self::set_timer_initial_count(0);
            Ok(())
        }
    }

    fn reset(&mut self) -> Result<(), LpTimerError> {
        Self::set_timer_initial_count(self.reset_value);
        Ok(())
    }

    fn get_interrupt_mask(&mut self) -> Result<bool, LpTimerError> {
        const MASK_BIT_SHIFT: u64 = 16;
        let apic_timer_lvt_entry = unsafe { msrs::read(msrs::APIC_TIMER_LVTR) };
        let is_masked = (apic_timer_lvt_entry >> MASK_BIT_SHIFT) & 0b1 == 1;
        Ok(is_masked)
    }

    fn set_interrupt_mask(&mut self, mask: bool) -> Result<(), LpTimerError> {
        const MASK_BIT_SHIFT: u64 = 16;
        let mut apic_timer_lvt_entry = unsafe { msrs::read(msrs::APIC_TIMER_LVTR) };
        if mask {
            apic_timer_lvt_entry |= 1u64 << MASK_BIT_SHIFT;
        } else {
            apic_timer_lvt_entry &= !(1u64 << MASK_BIT_SHIFT);
        }
        unsafe { msrs::write(msrs::APIC_TIMER_LVTR, apic_timer_lvt_entry) };
        Ok(())
    }

    extern "C" fn signal_eoi(&mut self) -> i32 {
        // We use level triggered interrupts for the APIC timer to ensure that we don't miss any
        // timer interrupts due to e.g. SMIs. Thus we must signal an EOI to the local APIC otherwise
        // the timer interrupt will immediately trigger again repeatedly.
        X2Apic::signal_eoi();
        0
    }

    fn set_isr_dispatch_number(&mut self, num: Self::IntDispatchNum) -> Result<(), LpTimerError> {
        let mut apic_timer_lvt_entry = unsafe { msrs::read(msrs::APIC_TIMER_LVTR) };
        apic_timer_lvt_entry &= !0xffu64;
        apic_timer_lvt_entry |= num as u64;
        unsafe { msrs::write(msrs::APIC_TIMER_LVTR, apic_timer_lvt_entry) };
        Ok(())
    }
}

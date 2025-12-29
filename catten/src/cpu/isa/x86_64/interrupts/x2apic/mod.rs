//! # x2APIC Local Advanced Programmable Interrupt Controller
mod id;

use core::arch::asm;

use super::super::constants::interrupt_vectors::*;
use crate::cpu::isa::constants::msrs::APIC_EOI_REGISTER;
use crate::cpu::isa::interface::interrupts::LocalIntCtlrIfce;
use crate::cpu::isa::interface::timers::LpTimerIfce;
use crate::cpu::isa::lp::LpId;
use crate::cpu::isa::timers::apic_timer::ApicTimer;
use crate::cpu::isa::x86_64::constants::msrs::{self, INTERRUPT_COMMAND_REGISTER};
use crate::get_lp_id;

pub enum Error {
    InvalidLpId,
}

/// # Interrupt Command Register Delivery Mode
#[repr(u32)]
enum IcrDeliveryMode {
    Fixed = 0b000,
    Smi = 0b010,
    Nmi = 0b100,
    Init = 0b101,
    Startup = 0b110,
}

/// # Interrupt Command Register Destination Shorthand
#[repr(u32)]
enum IcrDestShorthand {
    NoShorthand = 0b00,
    OnlySelf = 0b01,
    AllIncludingSelf = 0b10,
    AllExcludingSelf = 0b11,
}

pub struct X2Apic {
    pub timer: ApicTimer,
}

impl X2Apic {
    /// # Initialize the local APIC in x2APIC mode
    /// Ref: AMD APM 16.4.7
    fn new(timer_int_vec: <ApicTimer as LpTimerIfce>::IntDispatchNum) -> Self {
        // Set the Spurious Interrupt Vector Register (SIVR) to enable the APIC with Focused CPU
        // Core Checking and set the spurious interrupt vector to 32
        const FCC_BIT_SHIFT: u64 = 9;
        const ASE_BIT_SHIFT: u64 = 8;
        const VEC_MASK: u64 = 0xff;
        let sivr_val = SPURIOUS_INTERRUPT_VECTOR as u64 & VEC_MASK
            | (1 << ASE_BIT_SHIFT) // APIC Software Enable
            | (1 << FCC_BIT_SHIFT); // Focused CPU Core Checking Enable
        unsafe {
            msrs::write(msrs::APIC_SPURIOUS_INTERRUPT_VECTOR, sivr_val);
        }
        X2Apic {
            timer: ApicTimer::new(timer_int_vec),
        }
    }

    pub fn record_id() {
        unsafe {
            id::X2APIC_ID_TABLE.insert(get_lp_id(), id::LapicId::get_local());
        }
    }

    fn translate_lp_id(lp_id: LpId) -> Option<id::LapicId> {
        unsafe { id::X2APIC_ID_TABLE.get(&lp_id).cloned() }
    }

    fn make_icr_low(
        vector: u8,
        delivery_mode: IcrDeliveryMode,
        is_dest_logical: bool,
        level: bool,
        is_level_triggered: bool,
        dest_shorthand: IcrDestShorthand,
    ) -> u32 {
        const DELIVERY_MODE_SHIFT: u32 = 8;
        const IS_DEST_LOGICAL_SHIFT: u32 = 11;
        const LEVEL_SHIFT: u32 = 14;
        const IS_LEVEL_TRIGGERED_SHIFT: u32 = 15;
        const DEST_SHORTHAND_SHIFT: u32 = 18;

        vector as u32
            | ((delivery_mode as u32) << DELIVERY_MODE_SHIFT)
            | ((is_dest_logical as u32) << IS_DEST_LOGICAL_SHIFT)
            | ((level as u32) << LEVEL_SHIFT)
            | ((is_level_triggered as u32) << IS_LEVEL_TRIGGERED_SHIFT)
            | ((dest_shorthand as u32) << DEST_SHORTHAND_SHIFT)
    }

    pub fn set_timer_lvt_entry(
        interrupt_vector: <ApicTimer as LpTimerIfce>::IntDispatchNum,
        periodic: bool,
    ) {
        const TIMER_MODE_SHIFT: u64 = 17;
        const TIMER_MODE_PERIODIC: u64 = 0b1;
        const TIMER_MODE_ONE_SHOT: u64 = 0b0;
        const MASK_BIT_SHIFT: u64 = 16;
        const TIMER_VECTOR_MASK: u64 = 0xff;

        let timer_lvt_entry = (interrupt_vector as u64 & TIMER_VECTOR_MASK)
            | (if periodic {
                TIMER_MODE_PERIODIC
            } else {
                TIMER_MODE_ONE_SHOT
            } << TIMER_MODE_SHIFT)
                & !(1u64 << MASK_BIT_SHIFT); // Unmask the timer interrupt
        unsafe {
            msrs::write(msrs::APIC_TIMER_LVTR, timer_lvt_entry);
        }
    }
}

impl LocalIntCtlrIfce for X2Apic {
    type Error = Error;

    /// # Send a unicast IPI to the target logical processor
    ///
    /// Ref: Intel SDM Vol.3 12.12.10.1
    fn send_unicast_ipi(target_lp: LpId) -> Result<(), Error> {
        if let Some(apic_id) = Self::translate_lp_id(target_lp) {
            // Get the physical APIC ID for the target LP
            let dest = apic_id.physical;
            // Construct the ICR low dword
            let icr_low = Self::make_icr_low(
                UNICAST_IPI_VECTOR,
                IcrDeliveryMode::Fixed,
                false,
                true,
                false,
                IcrDestShorthand::NoShorthand,
            );
            // Write to the Interrupt Command Register MSR to send the IPI
            unsafe {
                asm!{
                    "wrmsr",
                    in("ecx") INTERRUPT_COMMAND_REGISTER,
                    in("eax") icr_low,
                    in("edx") dest,
                    options(nomem, nostack, preserves_flags),
                }
            }
            // Success
            Ok(())
        } else {
            Err(Error::InvalidLpId)
        }
    }

    fn signal_eoi() {
        unsafe {
            msrs::write(APIC_EOI_REGISTER, 0);
        }
    }
}

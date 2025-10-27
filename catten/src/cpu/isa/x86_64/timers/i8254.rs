//! # Intel 8254 Compatible Programmable Interval Timer
//!
//! At present this timer is only used to measure the TSC frequency.
//! It is a legacy timer present in all x86-64 systems however it should not be used
//! for any purpose other than TSC calibration as it is obsolete and this kernel
//! uses the TSC and APIC timers exclusively on the x86-64 architecture.
//!
//! Ref: https://osdev.wiki/wiki/Programmable_Interval_Timer

use crate::cpu::isa::io::{IReg8Ifce, IoReg8, OReg8Ifce};

static PIT_CH0_DATA_REG: IoReg8 = IoReg8::IoPort(0x40);
static PIT_CH1_DATA_REG: IoReg8 = IoReg8::IoPort(0x41);
static PIT_CH2_DATA_REG: IoReg8 = IoReg8::IoPort(0x42);
static PIT_MODE_COMMAND_REG: IoReg8 = IoReg8::IoPort(0x43);
static PIT_CH2_GATE_AND_OUTPUT_REG: IoReg8 = IoReg8::IoPort(0x61);
const CH2_OUTPUT_BIT: u8 = 5;
const CH2_GATE_INPUT_BIT: u8 = 0;
const CH2_SPEAKER_BIT: u8 = 1;

pub const PIT_FREQUENCY_HZ: u64 = 1_193_182;

fn make_command_byte(channel: u8, access_mode: u8, operating_mode: u8) -> u8 {
    ((channel & 0b11) << 6) | ((access_mode & 0b11) << 4) | ((operating_mode & 0b111) << 1)
}

pub fn set_interrupt_on_terminal_count(count: u16) {
    let cb = make_command_byte(2, 0b11, 0b000);
    let low_byte = (count & 0xff) as u8;
    let high_byte = ((count >> 8) & 0xff) as u8;
    unsafe {
        let mut pgo = PIT_CH2_GATE_AND_OUTPUT_REG.read();
        pgo |= 1 << CH2_GATE_INPUT_BIT;
        pgo &= !(1 << CH2_SPEAKER_BIT);
        PIT_CH2_GATE_AND_OUTPUT_REG.write(pgo); // Enable gate input
        PIT_MODE_COMMAND_REG.write(cb);
        PIT_CH2_DATA_REG.write(low_byte);
        PIT_CH2_DATA_REG.write(high_byte);
    }
}

pub fn read_channel_2_output() -> bool {
    unsafe { PIT_CH2_GATE_AND_OUTPUT_REG.read() & (1 << CH2_OUTPUT_BIT) != 0 }
}

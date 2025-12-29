//! # Intel 8042 Compatible Input Controller Driver
//!
//! This is the driver for the I8042 PS/2 Controller.
//! This uses interrupts for I/O so make sure these are handled.

use crate::cpu::isa::interface::io::{IReg8Ifce, OReg8Ifce};
use crate::cpu::isa::io::IoReg8;

// I/O Ports.
const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

// Status bits.
const STATUS_OUTPUT_BUFFER: u8 = 1 << 0;
const STATUS_INPUT_BUFFER: u8 = 1 << 1;

// Commands.
const CMD_DISABLE_KBD: u8 = 0xad;
const CMD_ENABLE_KBD: u8 = 0xae;
const CMD_DISABLE_MOUSE: u8 = 0xa7;
const CMD_ENABLE_MOUSE: u8 = 0xa8;

// Tests.
const CMD_CONTROLLER_TEST: u8 = 0xaa;
const CONTROLLER_TEST_OK: u8 = 0x55;
const CMD_KBD_INTERFACE_TEST: u8 = 0xab;
const CMD_MOUSE_INTERFACE_TEST: u8 = 0xa9;
const INTERFACE_TEST_OK: u8 = 0x00;

const CMD_ACK: u8 = 0xfa;

// Device commands.
const KBD_RESET: u8 = 0xff;

const KBD_ENABLE_SCANNING: u8 = 0xf4;
const KBD_DISABLE_SCANNING: u8 = 0xf5;

const MOUSE_ENABLE_REPORTING: u8 = 0xf4;
const MOUSE_DISABLE_REPORTING: u8 = 0xf5;

pub struct I8042 {
    data: IoReg8,
    status: IoReg8,
}

pub struct Ps2Status {
    pub keyboard_ok: bool,
    pub mouse_ok: bool,
}

impl I8042 {
    fn wait_input_empty(&self) {
        unsafe { while self.status.read() & STATUS_INPUT_BUFFER != 0 {} }
    }

    fn wait_output_full(&self) {
        unsafe { while self.status.read() & STATUS_OUTPUT_BUFFER == 0 {} }
    }

    unsafe fn send_keyboard_command(&self, cmd: u8) -> bool {
        unsafe {
            self.wait_input_empty();
            self.data.write(cmd);
            self.wait_output_full();
            return self.data.read() == CMD_ACK;
        }
    }

    unsafe fn send_mouse_command(&self, cmd: u8) -> bool {
        unsafe {
            self.wait_input_empty();
            self.status.write(0xd4);

            self.wait_input_empty();
            self.data.write(cmd);

            self.wait_output_full();
            return self.data.read() == CMD_ACK;
        }
    }

    ///# Main initialize function for the `I8042` driver.
    /// Initialize the PS/2 controller and device.
    /// Returns `Ok(I8042)` if both keyboard and mouse are successfully initialized.
    /// Else returns `Err(Ps2Status)` and states which devices failed.
    pub fn try_new() -> Result<Self, Ps2Status> {
        // The status.
        let mut status = Ps2Status {
            keyboard_ok: false,
            mouse_ok: false,
        };

        let driver = I8042 {
            data: IoReg8::IoPort(DATA_PORT),
            status: IoReg8::IoPort(STATUS_PORT),
        };

        unsafe {
            // Disable first.
            driver.wait_input_empty();
            driver.status.write(CMD_DISABLE_KBD);
            driver.wait_input_empty();
            driver.status.write(CMD_DISABLE_MOUSE);

            // Flush.
            while driver.status.read() & STATUS_OUTPUT_BUFFER != 0 {
                let _nothing = driver.data.read();
            }

            driver.wait_input_empty();
            driver.status.write(CMD_CONTROLLER_TEST);
            driver.wait_output_full();
            if driver.data.read() != CONTROLLER_TEST_OK {
                return Err(Ps2Status {
                    keyboard_ok: false,
                    mouse_ok: false,
                });
            }

            // Read CCB.
            driver.wait_input_empty();
            driver.status.write(0x20);
            driver.wait_output_full();
            let mut ccb = driver.data.read();

            // CCB Disable interrupts and disable translation.
            ccb &= !0b01000011;

            // Write CCB.
            driver.wait_input_empty();
            driver.status.write(0x60);
            driver.wait_input_empty();
            driver.data.write(ccb);

            driver.wait_input_empty();
            driver.status.write(CMD_KBD_INTERFACE_TEST);
            driver.wait_output_full();
            if driver.data.read() != INTERFACE_TEST_OK {
                // Test failed. TODO: Add something here.
            }

            driver.wait_input_empty();
            driver.status.write(CMD_MOUSE_INTERFACE_TEST);
            driver.wait_output_full();
            if driver.data.read() != INTERFACE_TEST_OK {
                // Mouse port interface failed. TODO: Same here.
            }

            // Keyboard.
            driver.wait_input_empty();
            driver.status.write(CMD_ENABLE_KBD);
            if driver.send_keyboard_command(KBD_RESET) {
                if driver.send_keyboard_command(KBD_ENABLE_SCANNING) {
                    status.keyboard_ok = true;
                }
            }

            // Mouse.
            driver.wait_input_empty();
            driver.status.write(CMD_ENABLE_MOUSE);
            if driver.send_mouse_command(MOUSE_ENABLE_REPORTING) {
                status.mouse_ok = true;
            }

            driver.wait_input_empty();
            driver.status.write(0x20); // CCB
            driver.wait_output_full();
            let mut final_ccb = driver.data.read();

            if status.keyboard_ok {
                final_ccb |= 0b00000001;
            }
            if status.mouse_ok {
                final_ccb |= 0b00000010;
            }

            driver.wait_input_empty();
            driver.status.write(0x60);
            driver.wait_input_empty();
            driver.data.write(final_ccb);
        }

        if status.keyboard_ok && status.mouse_ok {
            return Ok(driver);
        } else {
            return Err(status);
        }
    }

    /// Enable mouse data reporting.
    pub fn enable_mouse_packets(&self) -> bool {
        unsafe {
            return self.send_mouse_command(MOUSE_ENABLE_REPORTING);
        }
    }

    /// Disable mouse data reporting.
    pub fn disable_mouse_packets(&self) -> bool {
        unsafe {
            return self.send_mouse_command(MOUSE_DISABLE_REPORTING);
        }
    }

    /// Enable keyboard scanning (start sending key scancodes).
    pub fn enable_keyboard(&self) -> bool {
        unsafe {
            return self.send_keyboard_command(KBD_ENABLE_SCANNING);
        }
    }

    /// Disable keyboard scanning (stop sending key scancodes).
    pub fn disable_keyboard(&self) -> bool {
        unsafe { return self.send_keyboard_command(KBD_DISABLE_SCANNING) }
    }
}

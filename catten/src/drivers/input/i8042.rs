//! # Intel 8042 Compatible Input Controller Driver
//!
//! This is the driver for the I8042 PS/2 Controller.
//! This uses interrupts for I/O so make sure these are handled.

use crate::cpu::isa::io::{IReg8Ifce, IoReg8, OReg8Ifce};

// I/O Ports.
const DATA_PORT: u16 = 0x60;
const STATUS_PORT: u16 = 0x64;

// Status bits.
const STATUS_OUTPUT_BUFFER: u8 = 1 << 0;
const STATUS_INPUT_BUFFER: u8 = 1 << 1;

// Commands.
const CMD_DISABLE_KBD: u8 = 0xAD;
const CMD_ENABLE_KBD: u8 = 0xAE;
const CMD_DISABLE_MOUSE: u8 = 0xA7;
const CMD_ENABLE_MOUSE: u8 = 0xA8;

// Device commands.
const KBD_RESET: u8 = 0xFF;
const KBD_ENABLE_SCANNING: u8 = 0xF4;
const KBD_ACK: u8 = 0xFA;

const MOUSE_ENABLE_REPORTING: u8 = 0xF4;

pub struct I8042{
    data: IoReg8,
    status: IoReg8
}

pub struct Ps2Status{
    pub keyboard_ok: bool,
    pub mouse_ok: bool
}

impl I8042{
    fn wait_input_empty(&self){
        unsafe{
            while self.status.read() & STATUS_INPUT_BUFFER != 0{}
        }
    }

    fn wait_output_full(&self) {
        unsafe{
            while self.status.read() & STATUS_OUTPUT_BUFFER == 0{}
        }
    }

    unsafe fn send_device_command(&self, cmd: u8) -> bool{
        self.wait_input_empty();
        unsafe{
            self.data.write(cmd)
        }

        self.wait_output_full();
        let ack = unsafe{
            self.data.read()
        };

        return ack == KBD_ACK;
    }

    pub fn try_new() -> Result<Self, Ps2Status>{
        let mut status = Ps2Status{
            keyboard_ok: false,
            mouse_ok: false
        };

        let driver = I8042{
            data: IoReg8::IoPort(DATA_PORT),
            status: IoReg8::IoPort(STATUS_PORT)
        };

        unsafe{
            // Disable first.
            driver.wait_input_empty();
            driver.status.write(CMD_DISABLE_KBD);
            driver.wait_input_empty();
            driver.status.write(CMD_DISABLE_MOUSE);

            // Flush.
            while driver.status.read() & STATUS_OUTPUT_BUFFER != 0{
                let _nothing = driver.data.read();
            }

            // Keyboard.
            driver.wait_input_empty();
            driver.status.write(CMD_ENABLE_KBD);
            if driver.send_device_command(KBD_RESET){
                if driver.send_device_command(KBD_ENABLE_SCANNING){
                    status.keyboard_ok = true;
                }
            }

            // Mouse.
            driver.wait_input_empty();
            driver.status.write(CMD_ENABLE_MOUSE);
            if driver.send_device_command(MOUSE_ENABLE_REPORTING){
                status.mouse_ok = true;
            }
        }

        if status.keyboard_ok && status.mouse_ok{
            return Ok(driver);
        }
        else{
            return Err(status);
        }
    }
}
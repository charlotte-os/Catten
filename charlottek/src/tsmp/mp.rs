//! # Multi-Processor Control
use limine::mp::Cpu;

use crate::environment::boot_protocol::limine::MP;
use crate::{ap_main, logln};

#[derive(Debug)]
pub enum MpControlError {
    SecondaryLpStartupFailed,
}

pub fn start_secondary_lps() -> Result<(), MpControlError> {
    logln!("Starting Secondary LPs...");
    if let Some(res) = MP.get_response() {
        logln!("Obtained multiprocessor response from Limine");
        if cfg!(target_arch = "x86_64") {
            if res.flags().contains(limine::mp::ResponseFlags::X2APIC) {
                logln!("Limine has set all LAPICs to x2APIC mode.")
            } else {
                logln!("Limine has not set any LAPICs to x2APIC mode.")
            }
        }
        let lps = res.cpus();
        for lp in lps {
            logln!("Writing entrypoint address for LP{}", (lp.id));
            lp.goto_address.write(ap_main);
        }
        Ok(())
    } else {
        Err(MpControlError::SecondaryLpStartupFailed)
    }
}

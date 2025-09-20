//! # Multi-Processor Control
use alloc::collections::btree_map::BTreeMap;

use spin::Lazy;
use spin::RwLock;
use spin::mutex::Mutex;

use crate::ap_main;
use crate::environment::boot_protocol::limine::MP;
use crate::isa::target::lp;
use crate::logln;

static LP_COUNT: RwLock<Lazy<u32>> = RwLock::new(Lazy::new(|| {
    if let Some(mp_res) = MP.get_response() {
        mp_res.cpus().len() as u32
    } else {
        panic!("Limine was not able to start the secondary logical processors!")
    }
}));

pub static id_counter: Mutex<u32> = Mutex::new(1);

pub fn get_lp_count() -> u32 {
    **LP_COUNT.read()
}

#[derive(Debug)]
pub enum MpError {
    SecondaryLpStartupFailed,
}

pub fn start_secondary_lps() -> Result<(), MpError> {
    logln!("Starting Secondary LPs...");
    if let Some(res) = MP.get_response() {
        logln!("Obtained multiprocessor response from Limine");
        if cfg!(target_arch = "x86_64") {
            if res.flags().contains(limine::mp::ResponseFlags::X2APIC) {
                logln!("Limine has set all LAPICs to x2APIC mode.")
            } else {
                panic!("Processor not supported: x2APIC mode is not available.");
            }
        }
        let lps = res.cpus();
        for lp in lps {
            logln!("Writing entry point address for LP{}", (lp.id));
            lp.goto_address.write(ap_main);
        }
        Ok(())
    } else {
        Err(MpError::SecondaryLpStartupFailed)
    }
}

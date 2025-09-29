//! # Multi-Processor Management
use spin::{Lazy, RwLock};

use crate::environment::boot_protocol::limine::MP;
use crate::{ap_main, logln};

static LP_COUNT: RwLock<Lazy<u32>> = RwLock::new(Lazy::new(|| {
    if let Some(mp_res) = MP.get_response() {
        mp_res.cpus().len() as u32
    } else {
        panic!("Limine was not able to start the secondary logical processors!")
    }
}));

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

use core::sync::atomic::{AtomicU32, Ordering};

use crate::isa::lp::ops::*;

pub static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub unsafe fn assign_id() {
    let lp_id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    store_lp_id(lp_id);
    logln!(
        "Logical Processor with local interrupt controller ID = {} has been designated LP{}.",
        (get_lic_id!()),
        (get_lp_id())
    );
}

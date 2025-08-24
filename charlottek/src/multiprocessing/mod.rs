//! # Multi-Processor Control
use alloc::collections::btree_map::BTreeMap;

use spin::mutex::Mutex;
use spin::{Lazy, RwLock};

use super::threading::thread::ThreadId;
use crate::environment::boot_protocol::limine::MP;
use crate::isa::current_isa::lp_control::LpControl;
use crate::isa::interface::lp_control::LpControlIfce;
use crate::{ap_main, logln};

static LP_COUNT: RwLock<Lazy<usize>> = RwLock::new(Lazy::new(|| {
    if let Some(mp_res) = MP.get_response() {
        mp_res.cpus().len()
    } else {
        panic!("Limine was not able to start the secondary logical processors!")
    }
}));

pub fn get_lp_count() -> usize {
    **LP_COUNT.read()
}

pub type LpId = <LpControl as LpControlIfce>::LpId;

pub static LP_TABLE: spin::Lazy<BTreeMap<LpId, Mutex<LpState>>> = spin::Lazy::new(|| {
    let mut lp_table = BTreeMap::new();
    for i in 0..get_lp_count() {
        lp_table.insert(i as LpId, Mutex::new(LpState::NotStarted));
    }
    *(lp_table.get(&LpControl::get_lp_id()).unwrap().lock()) = LpState::Uninitialized;

    lp_table
});

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
        Err(MpControlError::SecondaryLpStartupFailed)
    }
}

pub enum LpState {
    NotStarted,
    Uninitialized,
    Running(ThreadId),
    Halted,
}

pub struct LogicalProcessor {
    pub id: u32,
    #[allow(unused)]
    state:  LpState,
}

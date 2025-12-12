//! # Initialization Module

use crate::common::time::duration::ExtDuration;
use crate::cpu::isa::init::IsaInitializer;
use crate::cpu::isa::interface::init::InitInterface;
use crate::cpu::isa::interface::lp::LpIsaDataIfce;
use crate::cpu::isa::interface::timers::LpTimerIfce;
use crate::cpu::isa::lp;
use crate::cpu::isa::timers::apic_timer::ApicTimerDivisors;
use crate::cpu::multiprocessor::lp_local::LpLocal;
use crate::logln;
use crate::memory::PHYSICAL_FRAME_ALLOCATOR;
use crate::memory::allocators::global_allocator::init_primary_allocator;

pub fn bsp_init() {
    logln!("Performing ISA specific initialization...");
    match IsaInitializer::init_bsp() {
        Ok(_) => logln!("ISA specific initialization complete."),
        Err(e) => {
            // initialization failure is irrecoverable
            panic!("ISA specific initialization failed: {:?}", e);
        }
    }
    logln!("Performing ISA independent initialization...");
    logln!("Initializing physical memory...");
    match PHYSICAL_FRAME_ALLOCATOR.try_lock() {
        Some(pfa) => {
            logln!("PhysicalFrameAllocator: {:?}", pfa);
        }
        None => {
            panic!("Failed to acquire lock on PhysicalFrameAllocator.");
        }
    }
    logln!("Initializing kernel allocator...");
    init_primary_allocator();
    logln!("Intialized kernel allocator.");
    logln!("Initializing LP local data structure");
    unsafe {
        LpLocal::init();
    }
    logln!("LP local data structure initialized.");
    logln!("Starting the LP local timer.");
    let timer = LpLocal::get_mut().isa_data.get_lp_timer();
    timer.set_duration(ExtDuration::from_secs(10));
    timer.start();
    logln!("LP local timer started with 10s duration.");
    logln!("ISA independent initialization complete.");
    logln!("BSP initialization complete.");
}

pub fn ap_init() {
    let lp_id = lp::ops::get_lp_id!();
    logln!("Initializing LP {}...", lp_id);
    logln!("LP {}: Performing ISA specific initialization...", lp_id);
    match IsaInitializer::init_ap() {
        Ok(_) => logln!("LP {}: ISA specific initialization complete.", lp_id),
        Err(e) => {
            // initialization failure is irrecoverable
            panic!("LP {}: ISA specific initialization failed: {:?}", lp_id, e);
        }
    }
    logln!("LP{}: Initializing LP local data structure", lp_id);
    unsafe {
        LpLocal::init();
    }
    logln!("LP{}: LP local data structure initialized.", lp_id);
    logln!("LP{}: Starting the LP local timer.", lp_id);
    let timer = LpLocal::get_mut().isa_data.get_lp_timer();
    timer.set_interrupt_mask(false);
    timer.set_duration(ExtDuration::from_secs(10));
    timer.start();
    logln!("LP{}: LP local timer started with 10s duration.", lp_id);
    logln!("LP{}: ISA independent initialization complete.", lp_id);
}

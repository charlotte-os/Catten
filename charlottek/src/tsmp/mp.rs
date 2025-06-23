//! # Multi-Processor Control

use crate::environment::boot_protocol::limine::MP;

pub enum MpControlError {
    SecondaryLpStartupFailed,
}

static SLP_MAIN_LIMINE: fn(&limine::mp::Cpu) -> ! = crate::main::ap_main;

pub fn start_secondary_lps() -> Result<(), MpControlError> {
    /// Attempts to start the secondary logical processors (LPs) in the system using four different
    /// approaches:
    /// 1. **Limine MP Feature**: Uses the Limine Multiprocessing feature to take control of the
    ///    secondary LPs.
    /// 2. **ACPI MPSS**: Use the ACPI Multiprocessor Startup Structure Mailboxes to take control of
    ///    the secondary LPs.
    /// 3. **Device Tree CPU Mailbox**: Use the Device Tree CPU mailboxes to take control of the
    ///    secondary LPs.
    ///    - On x86_64, this is always a no-op since x86_64 effectively never uses DTs.
    /// 4. **ISA MP Control**: Use an ISA specific firmware interface or interrupt sequence to start
    ///    and take control of the secondary LPs.
    ///    - On ARM64 this uses PSCI.
    ///    - On x86_64 this uses the x2APIC to send an INIT IPI to the secondary LPs, followed by
    ///      two STARTUP IPIs with the appropriate timings to start them.
    if mp_start_limine() {
        Ok(())
    } else if mp_start_acpi() {
        Ok(())
    } else if mp_start_dt() {
        Ok(())
    } else if mp_start_isa() {
        Ok(())
    } else {
        Err(MpControlError::SecondaryLpStartupFailed)
    }
}

fn mp_start_limine() -> bool {
    // Attempt to start secondary LPs using the Limine MP feature.
    if let Some(mp) = MP.get_response_mut() {
        logln!("Attempting to start secondary LPs using Limine MP feature...");
        let cpus = mp.cpus();
        if cpus.is_empty() {
            logln!("No secondary LPs found in Limine MP feature response.");
        } else {
            for cpu in cpus {
                logln!("Starting secondary LP: {:?}", cpu);
                cpu.goto_address.write(SLP_MAIN_LIMINE);
            }
        }
        true
    } else {
        logln!("Limine MP feature not available.");
        false
    }
}
fn mp_start_acpi() -> bool {
    // todo
    false
}
fn mp_start_dt() -> bool {
    // todo
    false
}
fn mp_start_isa() -> bool {
    crate::isa::current_isa::lp_control::LpControl::start_secondary_lps()
}

//! #Self Testing Subsystem
//!
//! This subsystem contains diagnostic tests meant to test the kernel itself and aid in development
//! and troubleshooting. Almost all subsystems with the exception of drivers should have at least
//! some tests in this module. In software engineering terminology the tests in this module should
//! be whitebox integration tests that can be run after Purrnel initializes itself.

pub mod memory;

use crate::logln;

pub fn run_self_tests() {
    logln!("Running self tests...");
    memory::pmem::test_pmem();
    memory::vmem::test_vmem();
    logln!("Testing Complete. All Tests Passed!");
}

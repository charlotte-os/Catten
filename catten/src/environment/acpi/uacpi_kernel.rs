//! # uACPI Kernel Interface
//!
//! This module provides kernel hooks for the uACPI library, enabling it to make use of this
//! kernel's functionality. uACPI is a C library which we use via a thin Rust wrapper made with
//! bindgen and cc. As such everything we provide to interact with it uses the C ABI.

use core::ffi::*;

use uacpi_raw::*;

use crate::environment::boot_protocol::limine::RSDP_REQUEST;

#[allow(unused)]
#[unsafe(no_mangle)]
pub extern "C" fn uacpi_kernel_get_rsdp(out_rsdp_address: *mut uacpi_phys_addr) -> uacpi_status {
    let rsdp = RSDP_REQUEST.get_response().expect("Limine failed to provide an RSDP").address();
    unsafe {
        out_rsdp_address.write(rsdp as u64);
    }
    uacpi_status_UACPI_STATUS_OK
}

#[allow(unused)]
#[unsafe(no_mangle)]
pub extern "C" fn uacpi_kernel_map(addr: uacpi_phys_addr, len: uacpi_size) -> *mut c_void {
    let corrected_phys_addr = addr & !(0xfff);
    let corrected_len = len + (addr - corrected_phys_addr) as uacpi_size;
}

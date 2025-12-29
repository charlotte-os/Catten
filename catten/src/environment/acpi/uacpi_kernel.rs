//! # uACPI Kernel Interface
//!
//! This module provides kernel hooks for the uACPI library, enabling it to make use of this
//! kernel's functionality. uACPI is a C library which we use via a thin Rust wrapper made with
//! bindgen and cc. As such everything we provide to interact with it uses the C ABI.

use alloc::ffi::CString;
use core::ffi::*;

use uacpi_raw::*;

use crate::cpu::isa::interface::memory::address::VirtualAddress;
use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::environment::boot_protocol::limine::RSDP_REQUEST;
use crate::log;
use crate::memory::linear::address_map::{LA_MAP, RegionType};
use crate::memory::linear::{MemoryMapping, PageType};
use crate::memory::{AddressSpaceInterface, KERNEL_AS, PAddr, VAddr};

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
    let mut corrected_len = len + (addr & 0xfff) as uacpi_size;
    if corrected_len % PAGE_SIZE != 0 {
        corrected_len += PAGE_SIZE - (corrected_len % PAGE_SIZE);
    }
    let kernel_dyn_region = LA_MAP.get_region(RegionType::KernelAllocatorArena);
    let mut kas = KERNEL_AS.lock();
    let mapping_addr: VAddr = match kas.find_free_region(
        corrected_len / PAGE_SIZE,
        (kernel_dyn_region.base, kernel_dyn_region.base + kernel_dyn_region.length),
    ) {
        Ok(addr) => addr,
        Err(_) => return core::ptr::null_mut(),
    };
    for offset in (0..corrected_len).step_by(PAGE_SIZE) {
        kas.map_page(MemoryMapping {
            vaddr: mapping_addr + offset,
            paddr: PAddr::from(corrected_phys_addr) + offset,
            page_type: PageType::KernelData,
        });
    }
    // Preserve the offset within the first page
    (mapping_addr + (addr & 0xfff) as usize).into_mut()
}

#[allow(unused)]
#[unsafe(no_mangle)]
pub extern "C" fn uacpi_kernel_unmap(mapped_addr: *mut c_void, len: uacpi_size) {
    let corrected_lin_addr = VAddr::from(mapped_addr as usize & !(0xfff));
    let mut corrected_len = len + (mapped_addr as usize & 0xfff) as uacpi_size;
    if corrected_len % PAGE_SIZE != 0 {
        corrected_len += PAGE_SIZE - (corrected_len % PAGE_SIZE);
    }
    let mut kas = KERNEL_AS.lock();

    for va in (corrected_lin_addr..corrected_lin_addr + corrected_len).step_by(PAGE_SIZE) {
        kas.unmap_page(va);
    }
}

#[allow(unused)]
#[unsafe(no_mangle)]
pub extern "C" fn uacpi_kernel_log(ll: uacpi_log_level, cstr: *const uacpi_char) {
    let string = unsafe { CString::from_raw(cstr as *mut c_char) };
    let rust_str = string.to_str().unwrap_or("Invalid UTF-8 string passed by UACPI.");
    let prefix = match ll {
        uacpi_log_level_UACPI_LOG_LEVEL_DEBUG => "[DEBUG]",
        uacpi_log_level_UACPI_LOG_LEVEL_INFO => "[INFO]",
        uacpi_log_level_UACPI_LOG_LEVEL_WARN => "[WARN]",
        uacpi_log_level_UACPI_LOG_LEVEL_ERROR => "[ERROR]",
        uacpi_log_level_UACPI_LOG_TRACE => "[TRACE]",
        _ => "[NONE]",
    };
    log!("{} UACPI: {}", prefix, rust_str);
}

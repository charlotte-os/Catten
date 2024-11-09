mod gdt;

use core::ptr;
use spin::{Lazy, Mutex};
use gdt::{Gdt, Tss};
use super::interface;

/// The BSP stack size is 4 pages by default.
const BSP_STACK_SIZE: usize = 4096 * 4;

/// The BSP stack for the kernel.
/// DO NOT TOUCH THIS, IT IS USED BY THE CPU AS THE KERNEL STACK.
/// UNTIL THE DYNAMIC ALLOCATOR IS INITIALIZED
#[used]
static BSP_STACK: [u8; BSP_STACK_SIZE] = [0u8; BSP_STACK_SIZE];
/// The Task State Segment for the BSP.
/// In long mode, the TSS is used to store the stack pointer for the kernel 
/// for each privilege level and for interrupts. It also contains the I/O permission bitmap
/// which is used to expose or block I/O ports to user-space applications.
static BSP_TSS: Lazy<Mutex<Tss>> = Lazy::new(|| Mutex::new(Tss::new(ptr::addr_of!(BSP_STACK[BSP_STACK_SIZE - 1]) as u64)));
/// The Global Descriptor Table for the BSP.
/// The GDT is used to store the segment descriptors for the kernel and user-space applications.
/// In long mode, the GDT is used to store the segment descriptors for the kernel and user-space applications.
/// It is largely just a leftover in long mode, as segmentation is no longer supported.
/// It also contains a system segment descriptor pointing to the TSS which is mandatory for interrupts to work.
static BSP_GDT: Lazy<Mutex<Gdt>> = Lazy::new(|| Mutex::new(Gdt::new(&BSP_TSS.lock())));

pub struct IsaInitializer;

#[derive(Debug)]
enum Error {
    InvalidGdt,
    InvalidTss,
}

impl interface::InitInterface for IsaInitializer {
    type Error = Error;
    
    fn init()->Result<(), Self::Error> {
        BSP_GDT.lock().load();
        Ok(())
    }
    fn deinit()->Result<(), Self::Error> {
        // Nothing to do here
        Ok(())
    }
}
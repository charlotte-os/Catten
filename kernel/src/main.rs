#![no_std]
#![no_main]

mod hal;

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    loop {}
}

#[panic_handler]
fn rust_panic(_info: &PanicInfo) -> ! {
    loop {}
}

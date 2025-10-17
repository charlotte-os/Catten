//! # Rust Panic Handler

use core::panic::PanicInfo;

use crate::cpu::isa::lp::ops::halt;
use crate::logln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    halt!()
}

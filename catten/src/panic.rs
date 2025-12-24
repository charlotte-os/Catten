//! # Rust Panic Handler

use core::panic::PanicInfo;

use crate::cpu::isa::lp::ops::halt;
use crate::logln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("***\nA kernel panic has occurred with the following cause:\n{}\n***", _info);
    halt!()
}

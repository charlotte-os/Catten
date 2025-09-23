use core::panic::PanicInfo;

use crate::isa::lp::ops::halt;
use crate::logln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    halt!()
}

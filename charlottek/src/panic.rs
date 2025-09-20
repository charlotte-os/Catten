use core::panic::PanicInfo;

use crate::isa::target::lp;
use crate::logln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    lp::halt!()
}

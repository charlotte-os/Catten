use core::panic::PanicInfo;

use crate::isa::current_isa::lp;
use crate::logln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    lp::halt!()
}

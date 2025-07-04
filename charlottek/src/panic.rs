use core::panic::PanicInfo;

use crate::isa::current_isa::lp_control::LpControl;
use crate::isa::interface::lp_control::LpControlIfce;
use crate::logln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    LpControl::halt()
}

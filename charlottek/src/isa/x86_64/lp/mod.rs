// x86_64 Logical Processor Operations
mod core_state;
mod lp_local;
mod ops;

pub use core_state::*;
pub use lp_local::*;
pub use ops::*;

pub type LpId = u32;

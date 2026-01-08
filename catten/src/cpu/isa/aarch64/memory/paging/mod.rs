mod pte;
use crate::common::size::kibibytes;

pub type HwAsid = u16;
pub const PAGE_SIZE: usize = kibibytes(4);

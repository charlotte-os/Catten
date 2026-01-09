//!  # Fixed Interrupt Service Routine Vector Assignments

pub const CONTEXT_SWITCH_VECTOR: u8 = 32;
pub const WAKE_LP_VECTOR: u8 = 33;
pub const UNICAST_IPI_VECTOR: u8 = 34;
pub const MULTICAST_IPI_VECTOR: u8 = 35;
pub const BROADCAST_IPI_VECTOR: u8 = 36;
/* Others go here */
pub const SPURIOUS_INTERRUPT_VECTOR: u8 = 255;

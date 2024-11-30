//! Software Graphics Subsystem
//!
//! This subsystem provides an interface for rendering graphics into a framebuffer using only the CPU.
//! It attempts to use CPU based hardware acceleration where possible to achieve performance comparable to an entry
//! level integrated GPU. This subsystem only supports 2D graphics rendering and does not support 3D rendering as that
//! is not necessary inside the kernel and could be implemented in userspace if needed. The kernel will eventually aim
//! to support hardware accelerated 2D and 3D rendering using GPU hardware, but this is a difficult and long term goal.

pub mod framebuffer;


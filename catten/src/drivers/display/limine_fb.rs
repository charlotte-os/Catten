//! # Limine Framebuffer Driver
//!
//! Stub driver that presents a standard interface to the Limine framebuffer for use by this
//! kernel's rendering subsystem. If the Limine boot protocol is not used to boot the kernel, or
//! if no framebuffers are provided by Limine, this driver will not enumerate anything when queried
//! by the device manager. Ideally on builds where Limine is not used, this driver should be
//! configured out.

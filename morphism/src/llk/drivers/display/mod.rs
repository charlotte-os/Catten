//! # Display Drivers
//!
//! This module contains drivers for raw display devices. These drivers provide the ability to draw to the display
//! however they do not provide any higher level functionality such as text rendering or window management nor do they
//! provide the ability to perform hardware accelerated rendering. These drivers are intended to be used to provide a
//! place for higher level graphics drivers to draw their output.
//!
//! The graphics drivers are what do the actual rendering whether through the use of dedicated hardware or on the CPU.
//! The external `embedded_graphics` crate serves as one such driver which can be used if the display driver implements
//! the `Draw` trait from that crate which all of the drivers under this module will strive to do.

// This driver needs work to actually be useful as a terminal or anything however when uncommented it does allow full
// use of the `embedded_graphics` crate.
/* pub mod limine_fb; */

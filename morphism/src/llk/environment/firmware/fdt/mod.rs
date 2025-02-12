//! # Flattened Device Tree
//!
//! A device tree is a tree data structure used to describe the hardware comfiguration of a system so that the
//! operating system does not need to hard code that information. The device tree is a tree of named nodes and
//! properties. The device tree is compiled into a binary format called the Flattened Device Tree (FDT) or less
//! formally a device tree blob (DTB). This is a standardized and well understood format that is used by many
//! operating systems and devices particularly in the ARM and RISC-V ecosystems. It provide much of the same
//! information that might otherwise be obtained from UEFI ot ACPI although unlike ACPI an FDT does not contain any
//! executable bytecode and all power management and device recognition must be performed by the kernel directly.
//!
//! This module makes use of the [`flat_device_tree`](https://docs.rs/flat_device_tree/3.1.1/flat_device_tree/) crate to parse the FDT and provide a convenient interface for the rest
//! of the kernel to use in a manner that allows it to be written in a manner independent of the specific boot method
//! or firmware interfaces available on a given platform.

# CharlotteOS - Catten

`Catten` is an operating system kernel developed as a key component of the CharlotteOS project but it is designed to be flexible enough that we hope it can also find use in many other places. It is intended to be a monolithic kernel with low-level system call interfaces that borrows ideas from exokernels and Fuchsia. Its design should allow for almost any higher level interface to be layered on top. It also includes a typesafe system namespace (akin to the namespaces found in Fuschsia and Plan 9 but more flexible and typesafe) with URI-like paths which has the added benefit of allowing access to the namespace of another host over a network without having to mount anything locally all while being secured by granular capabilities and a persistent mandatory access control policy.

`Catten` is still in early development, and core subsystems are actively being built. We welcome contributions—feel free to grab an issue from the tracker, suggest features, or participate in discussions on our repository, Discord server or Matrix instance.

## Programming Languages

- `Catten` is written in Rust and ISA specific assembly languages
- x86_64 assembly should use Intel syntax as implemented by `rustc` and `llvm-mc` exclusively

## External Dependencies

- C language dependencies are allowed if vetted by maintainers.
- Any dependencies in languages other than Rust, C, and assembly are strictly forbidden.
- Always prefer a high-quality Rust equivalent over an external C library unless there is good
  reason to do otherwise

## Target System Requirements

- Processor:
  - x86_64 (Primary ISA)
    - Invariant Timestamp Counter
    - Local APIC supporting x2APIC operating mode
  - ARM64 (Secondary ISA)
    - ARMv8.2A or later
    - GICv3 or later
- Firmware:
  - Unified Extensible Firmware Interface (UEFI)
  - Advanced Configuration and Power Interface (ACPI)
  - ARM Secure Monitor Calls (SMCs)
  - DTSpec conforming Flattened Devicetree (FDT)
- Memory:
  - Recommended: >= 1 GiB
  - Required: 128 MiB
- Storage:
  - Recommended: >= 64 GiB
  - Required: 4 GiB
  - Device Types:
    - Non-Volatile Memory Express (NVMe)
    - USB Mass Storage Device Class
- Input and Output:
  - Display Adapter:
    - Linear Framebuffer
      - UEFI Graphics Output Protoocol
      - FDT `simplefb` node
  - Keyboard
    - i8042 compatible PS/2 controller interface
    - USB Human Interface Device Class
    - I2C Human Interface Device Class
  - Serial:
    - NS16550 compatible UART
    - PL011 compatible UART
    - USB Communications Device Class Abstract Control Model (Virtual UART over USB)
- Networking:
  - USB Communications Device Class Network Control Model (Ethernet over USB)

## Contributing

Please reach out to us on Matrix or Discord if you are interested in contributing.

## Licensing

This kernel is licensed under the GNU Affero General Public License version 3.0 (or at your option, any later version). By contributing to this project you agree to license your contributions under that license and potentially also under alternate paid commercial licenses to raise funds for the project in the future should that be deemed necessary by the maintainers.

Find us on
------------

- [Discord](https://discord.gg/vE7bCCKx4X)
- [Matrix](https://matrix.to/#/#charlotteos:matrix.org)

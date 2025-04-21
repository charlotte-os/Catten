# CharlotteOS - Morphism

Morphism is an operating system kernel developed as key component of the CharlotteOS project but it is designed to be flexible enough that we hope it can also find use in many other places. It seeks to be a monolithic kernel with low-level system call interfaces that borrows ideas from exokernels and modern systems like Fuchsia. Its design allows for almost any higher level interface to be layered on top and also includes a typesafe system namespace (akin to the namespaces found in Fuschsia and Plan9 but more flexible and typesafe) with URIs as paths which has the added benefit of allowing access to the namespace of another host over a network without having to mount anything all while being secured by granular capability based access control.

Morphism is still in early development, and core subsystems are actively being built. We welcome contributions—feel free to grab an issue from the tracker, suggest features, or participate in discussions on our repository or Matrix server.


[Find us on Matrix](https://matrix.to/#/#charlotteos:matrix.org)

## Programming Languages
- Morphism is written in Rust and ISA specific assembly Languages
- x86_64 assembly should use Intel syntax exclusively in this project

## External Dependencies:
    - C dependencies are allowed if vetted by maintainers.
    - Any dependencies in languages other than Rust, C, and assembly are strictly forbidden.
    - Always prefer a high-quality Rust equivalent over an external C library unless there is good
      reason to do otherwise

## Licensing
This kernel is licensed under the GNU General Public License version 3.0 (or at your option, any later version).
If you cannot comply with this license, please do not contribute.

---

## Target System Requirements
- Processor: x86_64 CPU with x2APICs
- Firmware: Must provide standards compliant implementations of the Unified Extensible Firmware
  Interface (UEFI) and Advanced Configuration and Power Interface (ACPI)
- Memory: 512 MiB required; 4 GiB or more recommended
- Display Adapter: Any adaptor capable of providing framebuffers via the UEFI Graphics Output Protocol
- Storage: NVMe compatible storage medium with a capacity of 4 GiB or more
- Input: PS/2 or USB keyboard

> **Note:**
> If you are interested in porting Morphism to a new target please reach out to us on Matrix or
> open an issue on our GitHub repository.

---

## Contributing

We welcome your contributions! Here’s how you can get involved:

- Pick an issue from our tracker.
- Participate in discussions on GitHub or Matrix
- Open issues for proposed features or improvements.

Everyone interested in contributing is encouraged to join the CharlotteOS community!

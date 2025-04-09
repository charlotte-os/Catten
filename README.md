# CharlotteOS - Morphism

Morphism is an opearting system kernel developed as key component of the CharlotteOS project but it is flexible enough that we hope it can also find use in many other places. It is a monolithic kernel with low-level system call interfaces that borrows ideas from exokernels and modern systems like Fuchsia. Its design allows for almost any higher level interface to be layered on top and also includes a typesafe system namespace (akin to the namespaces found in Fuschsia and Plan9 but more flexible) with URIs as paths which has the added benefit of allowing access to the namespace of another host on a network without having to mount anything all while being scured by strict, granular capability based access control.

Morphism is still in early development, and core subsystems are actively being built. We welcome contributions—feel free to grab an issue from the tracker, suggest features, or participate in discussions on our repository or Discord server.

---

## Design Philosophy

- **Portability:**
  Morphism strives to be as independent as possible from the target ISA, hardware features, and firmware interfaces.
  - ISA-specific features should be developed for the `x86_64-unknown-none` target first.
  - All assembly code is assembled by the rustc assembler to enable inlining and optimizations when interfacing with Rust.

- **Programming Languages and Dependencies:**
  - **Primary:** Rust
  - **Secondary:** Assembly
  - **External Dependencies:**
    - C dependencies are allowed if vetted by maintainers.
    - External dependencies in languages other than C are strictly forbidden.
    - Always prefer a high-quality Rust equivalent over an external C library.

- **Licensing:**
  This kernel is licensed under the GNU General Public License version 3.0 (or at your option, any later version).
  If you cannot comply with this license, please do not contribute.

---

## Supported Targets

### Under Active Development
- **x86_64**
### May be Supported in the Future
- **ARM64**
- **RISC-V64GC**

> **Note:**
> At present UEFI and ACPI are to be required on all targets. On some ARM and RISC-V machines,
Das U-Boot may be able to provide these interfaces if properly compiled and configured to do so.
If you are unsure if your device supports these interfaces please consult your hardware vendor.

> If you are interested in porting Morphism to a new target or developing support for a target that is under consideration,
please reach out to us on Discord or open an issue on our GitHub repository.

---

## Contributing

We welcome your contributions! Here’s how you can get involved:

- Pick an issue from our tracker.
- Participate in discussions.
- Submit pull requests for features or improvements.

Everyone interested in contributing is encouraged to join the CharlotteOS community!

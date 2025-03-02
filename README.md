# CharlotteOS - Morphism

Morphism is the kernel developed as part of the CharlotteOS project. It is a monolithic kernel with low-level system call interfaces that borrows ideas from exokernels and modern systems like Fuchsia. Its design allows almost any OS personality to be layered on top through virtual shared objects (VSOs) and a flexible, typesafe system namespace (akin to a Unix filesystem, but more flexible).

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

- **x86_64 (Primary)**
- **Aarch64 (Secondary)**

> **Note:**  
> UEFI is required on all targets. Additionally, either ACPI or a Flattened Devicetree must be available. On some ARM machines, Das U-Boot may provide these interfaces if properly compiled and configured.

---

## Contributing

We welcome your contributions! Here’s how you can get involved:

- Pick an issue from our tracker.
- Participate in discussions.
- Submit pull requests for features or improvements.

Everyone interested in contributing is encouraged to join the CharlotteOS community!

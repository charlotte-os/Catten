# CharlotteOS – Catten

`Catten` is the kernel of CharlotteOS, designed as a robust, general-purpose monolithic kernel with a strong emphasis on clarity, safety, and architectural flexibility. While developed as part of CharlotteOS, the kernel is structured so that it may also serve as a foundation for other systems.

Catten draws inspiration from exokernels, Fuchsia, and capability-based microkernel designs while remaining monolithic for performance and simplicity. Its low-level system call interface is intentionally minimal, enabling a wide range of higher-level runtimes and user environments to be layered on top.

Catten also provides a typed, capability-secured system namespace with URI-like paths, inspired by concepts from Plan 9 and Fuchsia but extended for stronger typing, clean access semantics, and the ability to access another host’s namespace without mounting. Namespace operations are governed by granular capabilities and a persistent mandatory access control policy.

The kernel is still early in development. Core subsystems are under active construction, and contributions are welcome. Join our discussions on Discord, Matrix, or via the issue tracker if you’re interested in participating.

---

## Programming Languages

- Catten is written primarily in Rust, with architecture-specific assembly where required.
- x86-64 assembly uses Intel syntax as implemented by `rustc`/`llvm-mc`.
- Minimal C is permitted for vetted components where a high-quality Rust alternative does not exist.

---

## External Dependencies

- Rust, C, and assembly are the only allowed implementation languages.
- Any C dependency must be approved and justified.
- Non-Rust/C/ASM dependencies are not permitted.
- Prefer high-quality Rust crates unless native C is unavoidable due to specification or interoperability constraints.

---

## Platform & Firmware Requirements

CharlotteOS aims to support platforms that offer **standardized, documented, and interoperable hardware interfaces**. The focus is on systems where the OS can rely on well-defined firmware and discoverability mechanisms, without requiring vendor-specific hacks or opaque initialization sequences.

### Supported Architectures

#### x86-64 (Primary ISA)

- Invariant Timestamp Counter  
- Local APIC with x2APIC mode  
- Full UEFI and ACPI firmware environment

#### RISC-V64 (Secondary ISA)

- RVA23 or later  
- V extension not required  
- SBI runtime for early boot  
- Either ACPI or a **DTSpec-conforming Devicetree**  
  - Devicetree must reference **publicly documented IP blocks**  
  - Vendor-specific peripherals require accessible documentation

---

## Firmware Model

Catten supports both ACPI and Devicetree, with equal weight. The format is not the determining factor—**documentation and correctness are.**

### UEFI

- Required for PC/server-class systems on all architectures.
- Provides boot services, memory descriptors, and framebuffer/console access.

### ACPI

- Expected on PC/server-class RISC-V and all x86-64 systems.
- ACPI tables must be complete and spec-compliant enough to allow device discovery without vendor-specific workarounds.

### Flattened Devicetree (FDT)

- Fully supported for SoC-style platforms.
- FDT must conform to DTSpec and accurately describe hardware resources.
- All `compatible` strings must map to publicly documented hardware blocks or IP cores.

### Documentation Requirement

Whether via ACPI or DT:

- Devices must be identifiable.  
- Devices must be documented.  
- “Unknown peripheral at address 0xXXXX” is not acceptable without vendor documentation.

This ensures that Catten can operate without relying on undocumented Linux driver behavior, hard-coded quirks, or vendor-specific hacks.

---

## Hardware Recommendations

### Memory

- Recommended: ≥ 1 GiB  
- Minimum: 128 MiB

### Storage

- Recommended: ≥ 64 GiB  
- Minimum: 4 GiB  
- Supported device classes:
  - NVMe (PCIe)  
  - USB Mass Storage (MSC)

### Display

- Linear framebuffer exposed via:
  - UEFI GOP  
  - FDT `simplefb` node

### Input Devices

- Keyboards:
  - i8042 PS/2  
  - USB HID  
  - I²C HID (documented ACPI/FDT only)
- Pointers:
  - USB HID

### Serial Console

- NS16550-compatible UART  
- USB CDC-ACM (virtual serial)

### Networking

- USB CDC-NCM (Ethernet over USB)

---

## Contributing

We welcome contributions of all forms—code, design proposals, documentation, and testing.  
Please join our Discord or Matrix communities if you’d like to get involved.

Community contributions for new hardware are accepted **only** when accompanied by adequate documentation and clean, maintainable code.  
Undocumented or vendor-specific board support cannot be merged into the core.

---

## Licensing

Catten is licensed under the GNU Affero General Public License version 3.0 (or any later version). By contributing, you agree that your work may be distributed under the AGPL version 3.0 or later. Your work will also be included in possible commercial licensing arrangements should the project maintainers and community agree to offer those in the future as a source of funding and ecosystem growth. If you object to the latter use then please state that fact in the relevant commit messages and pull requests or send an email with the subject "Commercial Licensing Opt-Out" with all relevant commit hashes and proof that you were are their current copyright holder to <charlotte-os@outlook.com>.

---

## Community

Find us on:

- **Discord:** <https://discord.gg/vE7bCCKx4X>  
- **Matrix:** <https://matrix.to/#/#charlotteos:matrix.org>
- **Reddit** <https://www.reddit.com/r/charlotteos>
- **E-Mail** <charlotte-os@outlook.com>

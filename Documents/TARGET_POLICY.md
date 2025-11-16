# CharlotteOS Target Platform Policy

This document defines the official platform support policy for CharlotteOS. It provides a clear, concise overview of which platforms the kernel supports, why they are supported, and what standards or documentation are required for any architecture or board to qualify.

CharlotteOS ships **one unified kernel**, fully maintained in-tree. Platforms that do not meet these requirements may still be used via **external static libraries**, but cannot be merged into mainline.

---

## 1. Core Principles

### **Open Documentation**
CharlotteOS only supports platforms with complete, public hardware documentation. Vendor documentation is preferred, but **reverse‑engineered documentation is fully acceptable** if it is public, complete, and stable.

### **Standards Compliance**
Supported platforms must use:

- **UEFI** for boot
- **ACPI** *or* a **DTSpec‑conforming Devicetree**
- **SBI** on RISC‑V
- Fully documented devices and initialization sequences

The discoverability format (ACPI vs DT) does not matter; correctness and documentation do.

### **Maintainability**
All supported platforms must be maintainable by the core team without per‑board quirks or vendor‑specific workarounds.

---

## 2. Tier 1 — Officially Supported Platforms

Tier 1 platforms receive full mainline support and represent the intended environment for CharlotteOS.

### **2.1 x86‑64 (Primary ISA)**

Requirements:
- UEFI boot
- ACPI 6.x static tables
- Invariant TSC
- x2APIC
- Fully documented hardware

x86‑64 is the primary launch platform for CharlotteOS.

---

### **2.2 RISC‑V (Interoperable Platforms — BRS‑I, RVA22+)**

Tier 1 RISC‑V must meet full interoperable‑class standards.

Requirements:
- **RVA22 or later**
- UEFI boot
- ACPI 6.6+ *or* DTSpec‑conforming FDT
- SBI-compliant firmware
- All peripherals documented (vendor documentation or fully reverse‑engineered)
- PCIe, interrupts, and memory behavior following published specifications
- No undocumented initialization sequences

**Initial Tier 1 target:** Milk‑V Titan featuring the **UltraRISC UR‑CP100** CPU (pending evaluation).

---

### **2.3 Virtualized Environments**

Virtualization is essential for data center use and fully supported.

Supported hypervisors:
- QEMU
- KVM
- Xen (HVM/PVH)
- Hyper‑V
- VMware ESXi
- Any VMM providing UEFI, ACPI, and virtio‑class documented virtual devices

Virtual environments appear last in ordering but have full Tier 1 status.

---

## 3. Tier 2 — Embedded RISC‑V Platforms (RVB + BRS‑B)

Tier 2 includes embedded RISC‑V boards that meet the following:
- **RVB** hardware profile
- **BRS‑B** firmware profile
- DTSpec‑compliant FDT
- All hardware blocks fully documented (vendor or reverse‑engineered)
- No vendor‑specific Linux assumptions
- No undocumented boot or power‑up sequences

Tier 2 platforms must be maintainable without per‑board quirks.

---

## 4. Unsupported Hardware

A platform is unsupported if it:
- Lacks documentation for **core system components** required for OS bring-up (CPU, interrupt controller, timers, memory controllers, PCIe root complex, USB/XHCI, UARTs, etc.)
- Uses incomplete or Linux-tailored ACPI/DT for **critical subsystems**
- Requires undocumented vendor-specific initialization sequences for essential hardware
- Prevents running modified software without vendor keys

**Important:** A platform is *not* rejected simply because it includes some undocumented peripherals (e.g., GPUs, Wi-Fi NICs, accelerators). These devices will simply remain unsupported by CharlotteOS until proper documentation exists. The presence of such devices does *not* disqualify the rest of the system.

Unsupported platforms may still be used via out-of-tree static libraries, but cannot be merged into mainline.

This preserves a single, unified kernel while avoiding unnecessary platform exclusion.

---

## 5. RISC‑V Support Roadmap

### **Phase 1 — QEMU Bring‑Up**
- Validate paging, traps, interrupts
- Validate UEFI boot
- Validate ACPI/FDT parsing
- Ensure architecture abstraction layer stays clean

### **Phase 2 — Hardware Validation (UR‑CP100 / Titan)**
Criteria:
- Fully functional UEFI
- ACPI or compliant FDT
- Documented device controllers
- Standard PCIe behavior
- No opaque or proprietary firmware

### **Phase 3 — Tier 1 Promotion**
Upon meeting all criteria, the RISC‑V platform becomes Tier 1.

### **Phase 4 — Expand Tier 2**
Additional RVB + BRS‑B platforms may be added if fully documented and predictable.

---

## 6. Summary

**Tier 1**
- x86‑64
- RISC‑V (RVA22+, BRS‑I, documented devices such as UR‑CP100/Titan)
- Virtualized environments (QEMU, KVM, Xen, Hyper‑V, VMware)

**Tier 2**
- Embedded RISC‑V following RVB + BRS‑B with full public documentation

**Unsupported**
- Hardware lacking documentation
- Platforms requiring vendor‑specific hacks
- Systems without UEFI/ACPI/DTSpec
- May still be targeted via out‑of‑tree static libraries

CharlotteOS supports platforms that uphold open standards, predictable behavior, and long‑term sustainability for a unified kernel.


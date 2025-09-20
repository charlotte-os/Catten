# Copilot / AI Agent Project Instructions

Concise, action-focused context to make high-quality changes rapidly. Keep responses specific to this repository; avoid generic OS dev boilerplate unless directly relevant.

## 1. Project Snapshot
- Repository: `charlottek` – a monolithic, modular Rust kernel (early stage) for multiple ISAs (currently x86_64 primary; aarch64 & riscv64 scaffolding present).
- Boot flow: Limine Boot Protocol → `bsp_main` (`src/main.rs`) → `init::bsp_init()` → start secondary logical processors (`multiprocessing::start_secondary_lps`) → optional self tests → idle (HLT).
- Secondary CPUs enter at `ap_main` (currently minimal) and halt.
- Kernel is `#![no_std]`, uses `alloc` + custom global allocator built atop physical + virtual memory layers.

## 2. Key Subsystem Layout
- `isa/` – Architectural abstraction. `isa::interface/*` defines traits (memory, lp_control, system_info, io, interrupts). Concrete per-arch modules live under `isa/x86_64`, `isa/aarch64`, etc. Use `isa::target` for arch-selected reexports.
- `memory/` – Layered memory mgmt: `pmem` (physical frame allocator), `vmem` (address space + paging abstraction), `allocator.rs` (heap init using Talc). Kernel heap region is dynamically found in higher half; page size & significant VA bits come from ISA module.
- `drivers/` – Early basic drivers (keyboard ps/2, uart ns16550). Pattern: each driver in its own submodule, minimal global state guarded by spin or custom raw mutex.
- `multiprocessing/` – Brings up secondary logical processors (x2APIC requirement noted in README). Uses Limine SMP info.
- `scheduler/` – Early scaffolding (global/local schedulers, simple RR in `local/simple_rr.rs`). Not fully integrated yet—treat as experimental.
- `framebuffer/` – Text/console rendering over UEFI GOP-provided framebuffer; `console.rs` likely central abstraction (scan before extending output paths).
- `log/` – Logging macro(s) (e.g., `logln!`) used pervasively; prefer integrating with existing logging rather than ad-hoc UART writes.
- `panic.rs` – Minimal panic handler logs and halts via `LpControl::halt()`.

## 3. Conventions & Patterns
- Use Intel syntax for x86_64 assembly (per README); keep arch-specific asm within the respective ISA tree.
- Avoid introducing non-Rust dependencies unless justified (only Rust/C/asm allowed; prefer Rust crates like existing Talc allocator usage).
- Global resources: guard with `spin::Mutex` or custom `klib::raw_mutex::RawMutex` (wraps `lock_api`). Follow existing allocator initialization style.
- Memory mapping: use `AddressSpaceInterface::map_page` with a `MemoryMapping { vaddr, paddr, page_type }`. Do NOT handcraft page tables in new code; route through the ISA abstraction.
- Architecture selection: gate new ISA code with `#[cfg(target_arch = "x86_64")]` etc and expose via `target` reexport for call sites.
- Logging & panics: always prefer `logln!` before halting. Panic path must remain minimal (no allocations assumed safe).

## 4. Build & Run Workflows (Do These, Don’t Recreate)
- Build debug x86_64 ISO: `make build-x86_64-debug` (generates `charlottek-x86_64-debug.iso`).
- Run (KVM accel, serial on stdio): `make run-x86_64-debug`.
- Multiprocessor / NUMA test variants: `make run-x86_64-debug-multicore`, `make run-x86_64-debug-numa`.
- Release build: `make run-x86_64-release` (includes build step).
- Other ISAs: `make run-aarch64-debug`, `make run-riscv64-debug` (scaffolding; feature completeness may lag x86_64).
- Clean: `make clean` / `make distclean` (also removes bootloader clone & OVMF blobs).
- Cargo alone: always pass explicit target triple (e.g., `cargo build --target x86_64-unknown-none`). Don’t assume host std env.

## 5. Allocator & Memory Gotchas
- Global allocator (`Talc`) is installed in `memory/allocator.rs`; call `init_allocator()` only once after paging + frame allocator ready.
- Heap size constant: `KERNEL_HEAP_PAGE_COUNT` (currently 4 MiB). Expanding it requires ensuring contiguous virtual region available and enough physical frames.
- HIGHER HALF layout is computed, not hardcoded; respect `HIGHER_HALF_START/END` when reserving regions.

## 6. Adding New Code Safely
- New driver: create submodule under `drivers/<device>` and expose through `drivers/mod.rs`. Provide a small init function invoked from the architecture-specific init path (search existing init calls first).
- New memory-related feature: extend trait in `isa/interface/memory/*` then implement per-ISA; keep trait minimal—avoid leaking arch-specific types.
- Scheduler changes: coordinate with existing `simple_rr` prototype—avoid breaking current placeholder unless also updating call sites.
- Cross-ISA additions: mirror directory structure (`init/`, `interrupts/`, `memory/`, etc.) for consistency.

## 7. Error Handling & Halting
- Kernel frequently halts on unrecoverable errors via `LpControl::halt()`; maintainers prefer explicit expect messages for now (early stage). Provide clear context in `expect()` strings.

## 8. Style & Stability Constraints
- Nightly Rust features already enabled; before adding more `#![feature]` gates, justify necessity in PR description.
- Avoid allocations in early init paths before allocator init (self-tests run after allocator & SMP startup).
- Keep public interfaces small, trait-based, and architecture-agnostic.

## 9. Quick Reference (When Unsure)
- Entry points: `bsp_main`, `ap_main` in `src/main.rs`.
- Logging macro: search `logln!` usage for patterns.
- Memory traits: `isa/interface/memory/mod.rs`.
- Heap init pattern: `memory/allocator.rs::init_allocator`.
- SMP startup: `multiprocessing::start_secondary_lps()`.

## 10. What NOT to Do
- Don’t introduce `std` or host OS assumptions.
- Don’t bypass ISA abstractions for page tables or CPU control registers in shared code.
- Don’t add dependencies outside Rust/C/asm domain.
- Don’t silently unwrap in low-level critical paths—use `expect` with rationale.

---
If a task touches initialization order, SMP, or memory mapping, double‑check sequencing against `init::` and `memory::` modules before proceeding. Ask for clarification if a required trait implementation for a non-primary ISA is missing.

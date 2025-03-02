# CharlotteOS - Morphism

Morphism is the kernel being developed as part of the CharlotteOS project. It is a monolithic kernel with low level system call interfaces that takes some
cues from exokernels and more recent systems like Fuchsia. It is designed to allow just about any OS personality to be layered atop it through the use of virtual shared objects (VSOs) and masking the system namespace which you can think of as being akin to a Unix filesystem albeit more flexible, easier to understand, and typesafe. This enables the kernel to be useful in a wide range of use cases far beyond just serving as the supervisor of CharlotteOS, including significatly easing
operating system and computer science research.

Morphism is currently in very early development with core subsystems still being developed and we can use all the help we can get. Please feel free to grab
any issue from the tracker or add more that you think make sense. Also feel free to start any discussions you think would be relevant or seek help getting
into development on this project in the discussions section of this repository or in our discord server.

We strive to make Morphism as independent of the target ISA, hardware features, and firmware interfaces as possible given that we seek to make it as portable as it can be however things that are ISA specific should be developed for the `x86_64-unknown-none` target first as that is the primary target for initial development on this project as of now. All assembly code should be assembled by the rustc assembler to allow for things like inlining and optimization around assembly code and the ways in which it interfaces with Rust code.

This project is mostly written in the Rust programming language however external dependencies written in C are allowed so long as they are vetted by the maintainers and version updates or other changes to them are also similarly vetted. External dependencies in other languages even if they provide C ABI compatible interfaces that can be consumed from Rust are strictly forbidden. External C libraries or dependencies should not be used whenever a high quality, production grade Rust equivalent is available. If you don't know if a library or crate you're considering meets the criteria, please ask a maintainer.

This kernel like nearly all software developed by the CharlotteOS project is licensed under the terms of the GNU General Public License version 3.0 or at your option any later version of that same license. This is the result of a deliberate choice on the part of the project's founders to use a strong copyleft license and it is non-negotiable. If you do not want your contributions to be made available under this license then please do not contribute. Please see the top level license folder for more details.

## Implementation Languages

- Rust
- Assembly Language

## Targets

- x86_64 (Primary)
- Aarch64 (Secondary)

*UEFI is required on all targets. Either ACPI or a Flattened Devicetree is also required on all targets. If your system does not currently meet these requirements then please contact your hardware vendor.
Note that on certain ARM machines Das U-Boot may be able to provide these interfaces if compiled and configured to do so.

### Contributing

Feel free to make a PR in this repo, participate in the discussion section, and open issues for any changes or features
you think would enhance the project. Everyone who wants to participate is welcome to do so!

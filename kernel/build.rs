use std::env;

fn main() {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

        match arch.as_str() {
                "x86_64" => {
                        // Tell cargo to pass the linker script to the linker...
                        println!("cargo:rustc-link-arg=-Tlinker/x86_64.ld");
                        // ...and to re-run if it changes.
                        println!("cargo:rerun-if-changed=linker/x86_64.ld");
                }
                "aarch64" => {
                        // Tell cargo to pass the linker script to the linker...
                        println!("cargo:rustc-link-arg=-Tlinker/aarch64.ld");
                        // ...and to re-run if it changes.
                        println!("cargo:rerun-if-changed=linker/aarch64.ld");
                }
                "riscv64" => {
                        // Tell cargo to pass the linker script to the linker...
                        println!("cargo:rustc-link-arg=-Tlinker/riscv64.ld");
                        // ...and to re-run if it changes.
                        println!("cargo:rerun-if-changed=linker/riscv64.ld");
                }
                _ => panic!("Invalid ISA"),
        }

        println!("cargo:rerun-if-changed=asm");
}

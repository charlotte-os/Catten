#!/bin/sh

cargo fmt --check --manifest-path charlotte_core/Cargo.toml

# TODO: Keep targets updated
cargo check --target x86_64-unknown-none --manifest-path charlotte_core/Cargo.toml
# cargo clippy --target x86_64-unknown-none --manifest-path charlotte_core/Cargo.toml

grep -rn '#\[allow\(.*\)\]' ./charlotte_core/src/

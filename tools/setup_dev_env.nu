# Note: This script only works on the Red Hat family of Linux distributions with NuShell installed.
# For other Unix-like systems, the necessary packages will need to be installed manually.
sudo dnf install -y git curl clang llvm lld lldb make xorriso qemu
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
source $"($nu.home-path)/.cargo/env.nu"

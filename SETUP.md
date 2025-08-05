# Development Environment Setup

The following software must be installed on a Unix like system in order to build and develop `charlottek`. On Windows WSL2 can be used.
The preferred developement operating system is Fedora Workstation and all documentation and helper scripts will assume that it is the development OS used.

- curl
- git
- make
- qemu-system
- GNU coreutils
- clang
- llvm
- lldb
- rustup
- rust toolchain (nightly)
- xorriso
- Balena Etcher (to test on real hardware)

On Fedora systems the [automatic set up shell script](./tools/setup_dev_env.bash) can be used to install all of the necessary software simply by running the following command from the directory in which the current file is located:

```bash
sudo chmod 777 ./tools/setup_dev_env.bash && ./tools/setup_dev_env.bash
```

On other systems please refer to your operating system's documentation to determine how to install all of the aforementioned software programs.

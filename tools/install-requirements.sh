#!/bin/sh

## Functions and constants
## -----------------------

# POSIX ONLY - This script is supposed to run on nearly any distros
# (well at least popular ones)

# CHANGE THIS when there are new requirements
# WARNING: A package might be named differently on different distributions
required_packages_common="xorriso make curl"

# Here are the requirements with different names on different distros
required_packages_ubuntu="qemu-system"
required_packages_debian="qemu-system-x86"
required_packages_arch="qemu libisoburn"
required_packages_fedora="qemu"
required_packages_macos="qemu"

# WARNING: sudo needed

# Wrapper:
#   Print out status of executed command in a human readable way
# Parameters:
#   $1 - Label
#   $2 - Command
# WARNING: String indexing in POSIX is undefined
wrapper() {
    output=$("${@:2}" 2>&1)
    if [ $? -eq 0 ]
    then
        printf "\033[1m[\e[32m Ok \e[0m\033[1m] %s\e[0m\n" "$1"
    else
        printf "\033[1m[\e[31mFail\e[0m\033[1m] %s\e[0m\n       \033[1m\e[31mError\e[0m $output\n" "$1"
        return 1
    fi
}


# Install Rust and Rust nightly toolchain
install_rust_nightly() {
    printf "[    ] Rust nightly toolchain:\n"
    # Test if the nightly toolchain is not already installed, if it is leave it alone
    cd charlotte_core || exit 1
    if [ "$(rustc --version | grep nightly)" = '' ]; then
      wrapper "    Nightly not found, it will be installed now"
      wrapper "    Download Rust installer" $(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >> /tmp/rust-installer.sh)
      wrapper "    Make Rust installer executable" chmod +x /tmp/rust-installer.sh
      wrapper "    Install Rust nightly" /tmp/rust-installer.sh -y --profile minimal
      wrapper "    Remove Rust installer" rm /tmp/rust-installer.sh
    else
      wrapper "    Found nightly rust installed on this system" rustc --version
    fi
    cd .. || exit 1
}


## Main execution
#----------------

CONTINUE="n"
echo "This script is gonna do the following:"
echo "  Check if you have rust nightly installed and if not install it."
echo "  Install the packages you need to build and run the project. [will use sudo]"
printf "Continue?[y/N]:"
read -r CONTINUE

if [ "$CONTINUE" != "${CONTINUE,,}" ]; then
  exit 1
fi


# Detect OS and distribution
if [ "$(uname)" = "Darwin" ]; then
    for package in $required_packages_common; do
        wrapper "Install \"$package\" package" brew install "$package"
    done
    install_rust_nightly

    for package in $required_packages_macos; do
        wrapper "Install \"$package\" package" brew install "$package"
    done
else
    # Check for specific Linux distributions
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        if [ -n "$ID" ]; then
            case "$ID" in
                ubuntu)
                    for package in $required_packages_common; do
                        wrapper "Install \"$package\" package" sudo apt-get -y install "$package"
                    done
                    install_rust_nightly

                    for package in $required_packages_ubuntu; do
                        wrapper "Install \"$package\" package" sudo apt-get -y install "$package"
                    done
                    ;;
                debian)
                    for package in $required_packages_common; do
                        wrapper "Install \"$package\" package" sudo apt-get -y install "$package"
                    done
                    install_rust_nightly

                    for package in $required_packages_debian; do
                        wrapper "Install \"$package\" package" sudo apt-get -y install "$package"
                    done
                    ;;
                arch)
                    for package in $required_packages_common; do
                        wrapper "Install \"$package\" package" sudo pacman -S --noconfirm --needed "$package"
                    done
                    install_rust_nightly

                    for package in $required_packages_arch; do
                        wrapper "Install \"$package\" package" sudo pacman -S --noconfirm --needed "$package"
                    done
                    ;;
                fedora)
                    for package in $required_packages_common; do
                        wrapper "Install \"$package\" package" sudo dnf install -y "$package"
                    done
                    install_rust_nightly

                    for package in $required_packages_fedora; do
                        wrapper "Install \"$package\" package" sudo dnf install -y "$package"
                    done
                    ;;

                *)
                    printf "\033[1m[\e[31mFail\e[0m\033[1m] Linux distribution is not supported\n"
                    exit 1
                    ;;
            esac
            printf "\033[1m[\e[32m Ok \e[0m\033[1m] Installation done\n"
        else
            printf "\033[1m[\e[31mFail\e[0m\033[1m] Linux distribution could not be determined\n"
        fi
    else
        printf "\033[1m[\e[31mFail\e[0m\033[1m] Linux distribution could not be determined: missing /etc/os-release\n"
    fi
fi
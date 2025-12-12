#!/usr/bin/env bash

# This script sets up packages for each distro.

# What we need to install is:
# curl,
# edk2-ovmf, edk2-aarch64, edk2-riscv64
# git,
# llvm,
# clang,
# lld,
# lldb,
# make,
# xorriso,
# qemu,
# rust nightly.

# To add a new distro:
# Add a new entry in pkgm.
# Make a new distro_name_setup function.
# Call it in the case statement.

# We want to check if the file even exists.
if [ ! -e /etc/os-release ]; then
    echo "The file /etc/os-release was not found. This bash script is linux-only."
    exit 1
fi

# os-release defines distro ID as ID="distro".
DISTRO=$(grep '^ID=' /etc/os-release | cut -d= -f2 | tr -d '"')

# Echo the distro that we found.
echo "Distro found: $DISTRO"

# Each distro has their package manager.
declare -A pkgm=(
    ["arch"]="pacman"
)

# Check if the package manager is supported.
if [[ ! -v pkgm[$DISTRO] ]]; then
    echo "The package manager for your distro is not yet supported."
    exit 1
fi

PKGMAN=${pkgm[$DISTRO]}

# Echo the package manager.
echo "Package manager: $PKGMAN"

# Does the user want to proceed?
read -p "Do you want to continue? [Y/n] " answer

# User's answer.
answer=${answer:-Y}

# Check if the user declined.
if [[ ! "$answer" =~ ^[Yy]$ ]]; then
    echo "Aborting..."
    exit 1
fi

# Continue...

ROOT_PERM=sudo

# Set up for arch linux
arch_setup(){
    echo "Setting up for $DISTRO using $PKGMAN..."
    # Check for packages first!
    echo "Checking packages..."

    # Packages, rust last!
    local -a arch_pkgs=("curl" "git" "llvm" "clang" "lld" "lldb" "make" "xorriso" "qemu")
    # Missing packages list.
    local -a missing_pkgs=()

    # Check each package
    for pkg in "${arch_pkgs[@]}"; do
        if ! $PKGMAN -Q "$pkg" &>/dev/null; then
            missing_pkgs+=("$pkg")
        fi
    done

    # report missing packages
    if (( ${#missing_pkgs[@]} )); then
        echo "These packages are missing:"
        for mpkg in "${missing_pkgs[@]}"; do
            echo "  - $mpkg"
        done

        read -p "Do you want to install them? [Y/n] " answer
        answer=${answer:-Y}

        if [[ "$answer" =~ ^[Yy]$ ]]; then
            echo "Installing missing packages..."
            if ! command -v $ROOT_PERM >/dev/null; then
                echo "Sudo not found..."
                echo "Checking doas..."
            fi
            if command -v doas >/dev/null; then
                ROOT_PERM=doas
            else
                echo "Sudo and doas not found. aborting..."
                exit 1
            fi
            $ROOT_PERM $PKGMAN -S --noconfirm "${missing_pkgs[@]}"
            echo "Installation complete."
        else
            echo "Skipping installation of missing packages."
        fi
    else
        # all packages are present
        echo "All base packages installed!"
    fi

    # Check for rust separately
    # On arch pacman rust and rustup are not recommended together.
    echo "Checking for rust..."
    if $PKGMAN -Q rust &>/dev/null; then
        echo "Please uninstall pacman rust first."
        exit 1
    else
        echo "No pacman rust found. Proceeding to rustup..."
    fi

    if command -v rustup >/dev/null 2>&1; then
        echo "Rustup already exists! Make sure nightly is present."
    else
        echo "Installing rustup..."
        if ! command -v curl >/dev/null 2>&1; then
            echo "curl isn't present, cancelling install..."
            exit 1
        fi

        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
    fi
}

# This is a case statement for different distros, add as you will.
case "$DISTRO" in
    arch) arch_setup ;;
    # To add a distro, you'd define a package manager first, then a function for it.
    # Then you would add a new case like
    # distro_name) distro_name_setup ;;
    *)
        echo "Unsupported distro: $DISTRO"
        exit 1
        ;;
esac
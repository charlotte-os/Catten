#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Code sanity checker for charlotte_core"""

import os
import subprocess
import sys

# When adding a new target, add the target to the TARGETS list
# a target set as "core" will cause the script to raise an error if the target fails
TARGETS = {
    "x86_64-unknown-none": "core",
    "aarch64-unknown-none": "secondary",
    "riscv64gc-unknown-none-elf": "secondary",
}
TARGET_RESULTS = {}

def run_grep(lookup, filename) -> str:
    """Run grep on the file and return the result"""
    try:
        return subprocess.run(
            ["grep", "--recursive", lookup, filename],
            stdout=subprocess.PIPE,
            check=True,
        ).stdout.decode("utf-8")
    except subprocess.CalledProcessError:
        return ""

def check_style():
    """Check code style"""
    print("Checking code style")
    try:
        subprocess.run(
            [
                "cargo",
                "fmt",
                "--check",
                "--manifest-path",
                "charlotte_core/Cargo.toml",
            ],
            check=True,
            stderr=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
    except subprocess.CalledProcessError:
        print("style issues detected please run 'cargo fmt'")
        sys.exit(1)

    print("Code style check passed")

def check_code():
    """Check the project"""
    grep_res = run_grep("allow(unused)", "./charlotte_core/src")

    for target in TARGETS:
        print(f"Checking target: {target}")
        os.environ["TARGET"] = target
        try:
            subprocess.run(
                [
                    "cargo",
                    "check",
                    "--target",
                    target,
                    "--manifest-path",
                    "charlotte_core/Cargo.toml",
                ],
                check=True,
                stderr=subprocess.PIPE,
                stdout=subprocess.PIPE,
            )
        except subprocess.CalledProcessError:
            target_result = "Failed"
        else:
            target_result = "Ok"
        TARGET_RESULTS[target] = target_result

    print("\n\nResults:")
    print("--------")
    if grep_res:
        print(
            "Unused code warning supression detected! Please check them and remove if not needed."
        )
        print("Affected files:")
        print(grep_res)
    print("Target results:")
    for target, result in TARGET_RESULTS.items():
        print(f"{target}: {result}")

    if "Failed" in TARGET_RESULTS.values():
        for target, result in TARGET_RESULTS.items():
            if result == "Failed" and TARGETS[target] == "core":
                print(
                    f"Core target {target} failed to build, please fix the build errors!"
                )
                sys.exit(1)
        print(
            "WARN: Some non core target failed to build"
        )
        sys.exit(0)
        
    print("All checks passed!")
    sys.exit(0)

if __name__ == "__main__":
    check_style()
    check_code()

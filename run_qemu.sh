#!/usr/bin/env bash
set -euo pipefail

# Get to the right directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_DIR"

# Clean everything
cargo clean
rm -f Cargo.lock

# Build (the flag in .cargo/config.toml will handle the getrandom error)
cargo bootimage

# Find the binary
KERNEL_BIN=$(find target -name "bootimage-orchid-kernel.bin" | head -n 1)

if [[ -z "$KERNEL_BIN" ]]; then
  echo "Error: Kernel image not found."
  exit 1
fi

qemu-system-x86_64 \
  -drive format=raw,file="$KERNEL_BIN" \
  -serial stdio \
  -display none \
  -machine accel=tcg,usb=off

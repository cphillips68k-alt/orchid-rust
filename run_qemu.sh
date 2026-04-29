#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_DIR"

# 1. Clear out the old broken builds
cargo clean

# 2. Build the image with the necessary RUSTFLAGS
# We prefix the command with RUSTFLAGS so it's available to getrandom
echo "Building kernel..."
RUSTFLAGS='--cfg getrandom_backend="custom"' cargo bootimage

# 3. Path check (Ensure this matches your actual target filename)
KERNEL_BIN="target/x86_64-orchid_kernel/debug/bootimage-orchid-kernel.bin"

if [[ ! -f "$KERNEL_BIN" ]]; then
  # If the above path fails, check the blog_os default path
  KERNEL_BIN="target/x86_64-blog_os/debug/bootimage-orchid-kernel.bin"
fi

if [[ ! -f "$KERNEL_BIN" ]]; then
  echo "Error: Kernel image not found."
  exit 1
fi

# 4. Launch QEMU
qemu-system-x86_64 \
  -drive format=raw,file="$KERNEL_BIN" \
  -serial stdio \
  -display none \
  -machine accel=tcg,usb=off

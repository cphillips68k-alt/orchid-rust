#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
cd "$SCRIPT_DIR"

export RUSTFLAGS="-Zjson-target-spec"
export CARGO_BUILD_RUSTFLAGS="-Zjson-target-spec"
export CARGO_TARGET_X86_64_BLOG_OS_RUSTFLAGS="-Zjson-target-spec"

cargo +nightly bootimage

KERNEL_BIN="target/x86_64-blog_os/debug/bootimage-orchid-kernel.bin"
if [[ ! -f "$KERNEL_BIN" ]]; then
  echo "Kernel image not found: $KERNEL_BIN"
  exit 1
fi

qemu-system-x86_64 \
  -drive format=raw,file="$KERNEL_BIN" \
  -serial stdio \
  -display none \
  -machine accel=tcg,usb=off

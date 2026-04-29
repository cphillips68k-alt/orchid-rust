# Orchid Microkernel

Orchid is a hobby x86_64 microkernel written in Rust. It is designed to demonstrate a scalable operating system foundation with a simple preemptive scheduler, basic IPC, interrupt handling, and kernel services.

## Features

- `#![no_std]` kernel core
- Bootloader-based x86_64 bare-metal target
- VGA text console + COM1 serial output
- Preemptive scheduler driven by timer interrupts with context switching
- Stackful preemptive task execution and IPC-driven task interaction
- Linked-list heap allocator for kernel heap allocations
- PIC interrupt controller setup and timer IRQ handling

## Architecture

### Subsystems

- `src/main.rs` — kernel entry point; initializes hardware and subsystems.
- `src/kernel.rs` — boot-time task registration and kernel main loop.
- `src/scheduler.rs` — preemptive task scheduler driven by timer interrupts.
- `src/task.rs` — task definitions, states, and example kernel tasks.
- `src/ipc.rs` — in-kernel message passing with a fixed-size queue.
- `src/interrupts.rs` — IDT setup, IRQ handler installation, and PIC configuration.
- `src/console.rs` — VGA text printing utilities.
- `src/serial.rs` — serial debug output via COM1.
- `src/allocator.rs` — heap allocator using `linked_list_allocator`.
- `src/memory.rs` — bootloader memory map integration.
- `src/cpu.rs` — CPU helpers for halting the processor.
- `src/panic.rs` — panic handler for kernel abort behavior.

### Scheduling model

The kernel uses a true preemptive scheduler with actual context switching. A timer interrupt occurs periodically, the kernel saves the current task stack state, chooses the next runnable task, and resumes execution from that task's saved stack context.

## Getting Started

### Requirements

- Rust toolchain with `rustup`
- `cargo` and `rustc`
- `cargo bootimage`
- `qemu-system-x86_64`

### Install dependencies

```bash
rustup install nightly
rustup target add x86_64-unknown-none --toolchain nightly
cargo install bootimage
```

### Build

```bash
export CARGO_MANIFEST_DIR="$PWD"
export RUSTFLAGS="-Zjson-target-spec"
export CARGO_BUILD_RUSTFLAGS="-Zjson-target-spec"
export CARGO_TARGET_X86_64_BLOG_OS_RUSTFLAGS="-Zjson-target-spec"
cargo +nightly -Zjson-target-spec bootimage
```

Alternatively, use the provided script which handles this automatically:

```bash
chmod +x run_qemu.sh
./run_qemu.sh
```

## Development Notes

The kernel uses a simple task abstraction rather than true process isolation. Example tasks are:

- `driver` — probes devices and sends events
- `service` — receives driver events and broadcasts status
- `shell` — queries service status and logs responses

### Extending the kernel

To add features, consider:

- implementing real context switching with separate task stacks
- adding memory protection and page tables
- introducing hardware device drivers for keyboard and disk
- building a filesystem abstraction
- adding syscall/message validation for user services

## QEMU launch script

`run_qemu.sh` is provided to build and launch the kernel in QEMU. It uses the bootimage output from the `target` folder.

## License

This project is released under the [GPL3 License](LICENSE).

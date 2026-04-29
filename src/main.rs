#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod console;
mod cpu;
mod interrupts;
mod ipc;
mod kernel;
mod memory;
mod scheduler;
mod serial;
mod task;

use bootloader::{entry_point, BootInfo};
use console::kprintln;
use getrandom::Error;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    allocator::init_heap();
    console::init();
    serial::init();
    interrupts::init_idt();
    interrupts::init_pics();
    memory::init(boot_info);
    interrupts::enable();

    kprintln!("Orchid Microkernel v0.1");
    kprintln!("Memory regions: {}", memory::region_count());
    kprintln!("Launching task manager...");

    kernel::run()
}
// A stub implementation for getrandom. 
// You can implement actual entropy gathering here later.
fn custom_getrandom(buf: &mut [u8]) -> Result<(), Error> {
getrandom::register_custom_getrandom!(custom_getrandom_v02);
fn custom_getrandom_v02(buf: &mut [u8]) -> Result<(), Error> {
    custom_fill(buf)
}

// For getrandom v0.3 and v0.4
#[unsafe(no_mangle)]
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error> {
    let slice = unsafe { core::slice::from_raw_parts_mut(dest, len) };
    custom_fill(slice)
}

// Shared implementation
fn custom_fill(buf: &mut [u8]) -> Result<(), Error> {
    for byte in buf.iter_mut() {
        *byte = 0;
    }
    Ok(())
}

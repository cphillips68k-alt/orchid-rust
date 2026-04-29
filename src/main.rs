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
    // For now, just fill with zeros or return an error
    for byte in buf.iter_mut() {
        *byte = 0;
    }
    Ok(())
}

getrandom::register_custom_getrandom!(custom_getrandom);

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

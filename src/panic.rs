use crate::{console::kprintln, cpu::halt_loop};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("[PANIC] {}", info);
    halt_loop()
}

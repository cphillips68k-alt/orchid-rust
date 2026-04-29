use x86_64::instructions::{hlt, interrupts};

pub fn halt_loop() -> ! {
    loop {
        hlt();
    }
}

pub fn shutdown() -> ! {
    interrupts::disable();
    halt_loop()
}

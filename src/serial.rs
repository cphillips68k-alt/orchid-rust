use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial = unsafe { SerialPort::new(0x3F8) };
        serial.init();
        Mutex::new(serial)
    };
}

pub fn init() {
    let _ = SERIAL1.lock();
}

pub fn write_byte(byte: u8) {
    SERIAL1.lock().send(byte);
}

pub fn write_string(message: &str) {
    for byte in message.bytes() {
        write_byte(byte);
    }
}

pub fn println(message: &str) {
    write_string(message);
    write_byte(b'\n');
}

use core::fmt;

use spin::{Mutex, MutexGuard, Once};

use crate::utils::io;

static COM_1: Once<Mutex<SerialPort>> = Once::new();

bitflags::bitflags! {
    pub struct InterruptEnable: u8 {
        const RECEIVED = 1;
        const SENT = 1 << 1;
        const ERRORED = 1 << 2;
        const STATUS_CHANGE = 1 << 3;
    }
}

bitflags::bitflags! {
    pub struct LineStatus: u8 {
        const INPUT_FULL = 1;
        const OUTPUT_EMPTY = 1 << 5;
    }
}

/// An interface to a serial port that allows sending out individual bytes.
#[repr(transparent)]
pub struct SerialPort(u16);

impl SerialPort {
    #[inline(always)]
    pub const fn new(port: u16) -> Self {
        Self(port)
    }

    /// Initialize the serial port.
    pub unsafe fn init(self) -> Self {
        // Disable interrupts.
        io::outb(self.0 + 1, 0x00);

        // Enable DLAB.
        io::outb(self.0 + 3, 0x80);

        // Set maximum speed to 38400 bps by configuring DLL and DLM.
        io::outb(self.0, 0x03);
        io::outb(self.0 + 1, 0x00);

        // Disable DLAB and set data word length to 8 bits.
        io::outb(self.0 + 3, 0x03);

        // Enable FIFO, clear TX/RX queues and set interrupt watermark at 14 bytes.
        io::outb(self.0 + 2, 0xC7);

        // Mark data terminal ready, signal request to send and enable auxilliary
        // output #2 (used as interrupt line for CPU).
        io::outb(self.0 + 4, 0x0B);

        // Enable interrupts.
        io::outb(self.0 + 1, 0x01);

        self
    }

    pub fn line_status(&self) -> LineStatus {
        unsafe {
            let status = io::inb(self.0 + 5);

            LineStatus::from_bits_truncate(status)
        }
    }

    pub fn send_byte(&mut self, byte: u8) {
        unsafe {
            match byte {
                8 | 0x7F => {
                    while !self.line_status().contains(LineStatus::OUTPUT_EMPTY) {
                        core::hint::spin_loop()
                    }
                    io::outb(self.0, 8);

                    while !self.line_status().contains(LineStatus::OUTPUT_EMPTY) {
                        core::hint::spin_loop()
                    }
                    io::outb(self.0, b' ');

                    while !self.line_status().contains(LineStatus::OUTPUT_EMPTY) {
                        core::hint::spin_loop()
                    }
                    io::outb(self.0, 8);
                }
                _ => {
                    while !self.line_status().contains(LineStatus::OUTPUT_EMPTY) {
                        core::hint::spin_loop()
                    }

                    io::outb(self.0, byte)
                }
            }
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.bytes() {
            self.send_byte(byte);
        }

        Ok(())
    }
}

pub fn get_com_1() -> MutexGuard<'static, SerialPort> {
    COM_1
        .get()
        .expect("Attempted to get a reference to the COM 1 port before it was initialized")
        .lock()
}

/// Initialize the serial ports if avaliable.
pub fn init() {
    unsafe {
        let com_1 = SerialPort::new(0x3F8).init();

        COM_1.call_once(move || Mutex::new(com_1));
    }
}
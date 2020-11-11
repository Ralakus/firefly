use crate::Mmio;
use core::fmt;
use spin::MutexGuard;

use super::devices::serial::UART1;
use crate::drivers::uart_16550::Uart16550;

pub struct Writer<'a> {
    uart1: MutexGuard<'a, Option<&'static mut Uart16550<Mmio<u8>>>>,
}

impl<'a> Writer<'a> {
    pub fn new() -> Self {
        Self {
            uart1: UART1.lock(),
        }
    }

    pub fn write(&mut self, buf: &[u8]) {
        if let Some(ref mut uart1) = *self.uart1 {
            uart1.write(buf);
        }
    }
}

impl<'a> fmt::Write for Writer<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.write(s.as_bytes());
        Ok(())
    }
}

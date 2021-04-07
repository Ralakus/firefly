use core::convert::TryInto;

use crate::{Io, Mmio, ReadOnly};

bitflags! {
    /// Interrupt enable flags
    struct IntEnFlags: u8 {
        const RECEIVED = 1;
        const SENT = 1 << 1;
        const ERRORED = 1 << 2;
        const STATUS_CHANGE = 1 << 3;
        // 4 to 7 are unused
    }
}

bitflags! {
    /// Line status flags
    struct LineStsFlags: u8 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}

#[allow(dead_code)]
#[repr(packed)]
pub struct Uart16550<T: Io> {
    /// Data register, read to receive, write to send
    data: T,
    /// Interrupt enable
    int_en: T,
    /// FIFO control
    fifo_ctrl: T,
    /// Line control
    line_ctrl: T,
    /// Modem control
    modem_ctrl: T,
    /// Line status
    line_sts: ReadOnly<T>,
    /// Modem status
    modem_sts: ReadOnly<T>,
}

impl Uart16550<Mmio<u8>> {
    pub unsafe fn new(base: usize) -> &'static mut Uart16550<Mmio<u8>> {
        &mut *(base as *mut Self)
    }
}

impl<T: Io> Uart16550<T>
where
    T::Value: From<u8> + TryInto<u8>,
{
    pub fn init(&mut self) {
        unsafe {
            // disable interrupts
            self.int_en.write(0x00.into());

            // special mode to set baud rate
            self.line_ctrl.write((1 << 7).into());

            // low byte for baud rate
            self.data.write(0x01.into());

            // high byte for baud rate
            self.int_en.write(0x00.into());

            // leave set-baud mode,
            // and set word length to 8 bits, no parity
            self.line_ctrl.write(0x03.into());

            // reset and enable FIFOs.
            self.fifo_ctrl.write(((1 << 0) | (3 << 1)).into());

            // mark data terminal ready, signal request to send
            // and enable auxilliary output #2 (used as interrupt line for CPU)
            self.modem_ctrl.write(0x0B.into());

            // enable transmit and receive interrupts
            self.int_en.write(((1 << 0) | (1 << 1)).into());
        }
    }

    fn line_sts(&self) -> LineStsFlags {
        LineStsFlags::from_bits_truncate(
            (unsafe { self.line_sts.read() } & 0xFF.into())
                .try_into()
                .unwrap_or(0),
        )
    }

    pub fn read(&mut self) -> Option<u8> {
        if self.line_sts().contains(LineStsFlags::INPUT_FULL) {
            Some(
                (unsafe { self.data.read() } & 0xFF.into())
                    .try_into()
                    .unwrap_or(0),
            )
        } else {
            None
        }
    }

    pub fn send(&mut self, data: u8) {
        while !self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {}
        unsafe { self.data.write(data.into()) }
    }

    pub fn write(&mut self, buf: &[u8]) {
        for &b in buf {
            match b {
                8 | 0x7F => {
                    self.send(8);
                    self.send(b' ');
                    self.send(8);
                }
                b'\n' => {
                    self.send(b'\r');
                    self.send(b'\n');
                }
                _ => {
                    self.send(b);
                }
            }
        }
    }
}

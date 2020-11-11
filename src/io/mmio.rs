use core::mem::MaybeUninit;
use core::ops::{BitAnd, BitOr, Not};
use core::ptr::{read_volatile, write_volatile};

use super::Io;

#[repr(packed)]
pub struct Mmio<T> {
    address: MaybeUninit<T>,
}

impl<T> Mmio<T> {
    pub unsafe fn zeroed() -> Self {
        Self {
            address: MaybeUninit::zeroed(),
        }
    }
    pub unsafe fn uninit() -> Self {
        Self {
            address: MaybeUninit::uninit(),
        }
    }
}

impl<T> Io for Mmio<T>
where
    T: Copy + PartialEq + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T>,
{
    type Value = T;

    fn read(&self) -> T {
        unsafe { read_volatile(self.address.as_ptr()) }
    }

    fn write(&mut self, value: T) {
        unsafe { write_volatile(self.address.as_mut_ptr(), value) };
    }
}

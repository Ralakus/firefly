#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(const_fn_transmute)]
#![feature(global_asm)]

#[macro_use]
extern crate bitflags;
extern crate spin;

#[macro_use]
pub mod arch;
pub use arch::*;

pub mod io;
pub use io::*;

pub mod panic;

pub mod drivers;

fn kmain() -> ! {
    println!("--==Firefly==--");
    println!("boot successful");
    loop {}
}

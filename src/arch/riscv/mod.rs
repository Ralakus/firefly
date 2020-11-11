#[macro_use]
pub mod macros;
pub mod debug;
pub mod devices;
pub mod start;

mod asm;

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

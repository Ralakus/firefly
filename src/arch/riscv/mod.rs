/// RISC-V specific macros.
#[macro_use]
pub mod macros;
/// RISC-V default debug IO.
pub mod debug;
/// RISC-V universal device initialization.
pub mod devices;
/// RISC-V boot code.
pub mod start;

/// RISC-V assembly.
mod asm;

/// RISC-V specific abort function.
/// Waits for interrupt.
#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

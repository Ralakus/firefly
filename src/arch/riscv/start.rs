use super::devices;

/// Architecture specific kernel initialization.
/// RISC-V Boot sequence.
/// 1. Assembly functions under `asm/`.
/// 2. Assembly calls `kstart()`.
/// 3. Device initialization.
/// 4. Kernel initialization.
#[no_mangle]
extern "C" fn kstart() -> ! {
    unsafe { devices::init() };

    println!("devices init");

    crate::kmain();
}

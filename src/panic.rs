/// Interal Rust function.
/// I honestly don't know what this is for, but it's required to do bare metal Rust.
#[lang = "eh_personality"]
#[no_mangle]
extern "C" fn rust_eh_personality() {}

/// Rust's internal panic handler for kernel.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(p) = info.location() {
        println!(
            "line {}, file {}: {}",
            p.line(),
            p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }
    crate::arch::abort();
}

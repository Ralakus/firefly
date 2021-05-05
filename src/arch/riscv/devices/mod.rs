/// Initializes serial devices.
pub mod serial;

/// # Safety
/// This function should only be called once before kernel is initialized.
/// Initializes devices.
pub unsafe fn init() {
    serial::init();
    println!("serial init");
}

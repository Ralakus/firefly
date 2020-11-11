pub mod serial;

pub unsafe fn init() {
    serial::init();
    println!("serial init");
}

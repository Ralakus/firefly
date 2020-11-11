use super::devices;

#[no_mangle]
extern "C" fn kstart() -> ! {
    unsafe { devices::init() };

    println!("devices init");

    crate::kmain();
}

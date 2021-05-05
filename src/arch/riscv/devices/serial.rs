use crate::drivers::uart_16550::Uart16550;
use crate::Mmio;
use spin::Mutex;

/// UART device one.
/// This is the device used by the default debug IO.
pub static UART1: Mutex<Option<&'static mut Uart16550<Mmio<u8>>>> = Mutex::new(None);

/// UART device two.
/// Only enabled for Kendryte K210 devices.
#[cfg(feature = "k210")]
pub static UART2: Mutex<Option<&'static mut Uart16550<Mmio<u8>>>> = Mutex::new(None);
/// UART device three.
/// Only enabled for Kendryte K210 devices.
#[cfg(feature = "k210")]
pub static UART3: Mutex<Option<&'static mut Uart16550<Mmio<u8>>>> = Mutex::new(None);

/// # Safety
/// This function should only be called once during device initialization.
/// Initializes serial deveices.
#[cfg(feature = "k210")]
pub(super) unsafe fn init() {
    let uart1 = Uart16550::<Mmio<u8>>::new(0x50210000);
    let uart2 = Uart16550::<Mmio<u8>>::new(0x50220000);
    let uart3 = Uart16550::<Mmio<u8>>::new(0x50230000);

    uart1.init();
    uart2.init();
    uart3.init();

    *UART1.lock() = Some(uart1);
    *UART2.lock() = Some(uart2);
    *UART3.lock() = Some(uart3);

    println!("UART1 ready");
    println!("UART2 ready");
    println!("UART3 ready");
}

/// # Safety
/// This function should only be called once during device initialization.
/// Initializes serial device.
/// **This address is inteded to work with qemu's UART configuration.**
#[cfg(not(feature = "k210"))]
pub(super) unsafe fn init() {
    let uart1 = Uart16550::<Mmio<u8>>::new(0x1000_0000);

    uart1.init();

    *UART1.lock() = Some(uart1);

    println!("UART1 ready");
}

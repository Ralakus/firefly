use crate::drivers::uart_16550::Uart16550;
use crate::Mmio;
use spin::Mutex;

pub static UART1: Mutex<Option<&'static mut Uart16550<Mmio<u8>>>> = Mutex::new(None);

#[cfg(feature = "k210")]
pub static UART2: Mutex<Option<&'static mut Uart16550<Mmio<u8>>>> = Mutex::new(None);
#[cfg(feature = "k210")]
pub static UART3: Mutex<Option<&'static mut Uart16550<Mmio<u8>>>> = Mutex::new(None);

#[cfg(feature = "k210")]
pub unsafe fn init() {
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

#[cfg(not(feature = "k210"))]
pub unsafe fn init() {
    let uart1 = Uart16550::<Mmio<u8>>::new(0x1000_0000);

    uart1.init();

    *UART1.lock() = Some(uart1);

    println!("UART1 ready");
}

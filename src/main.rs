#![no_std]
#![no_main]
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use usart::Usart;
mod usart;

mod gpio;

#[entry]
fn main() -> ! {
    let mut usart = Usart::new();
    usart.init(9600);

    loop {
        let message = b"Hello, USART! \n";
        for &byte in message {
            usart.transmit(byte);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod usart;
use usart::Usart;


#[no_mangle]
pub extern "C" fn main() -> ! {
    let usart = Usart::new();

    usart.init(9600);


    loop {
        // Envoi de "Hello World\n" caractère par caractère
        let message = b"Hello World\n";
        for &c in message {
            usart.transmit(c);
        }
    
        // Réception d'un caractère
        let received = Usart.receive();
        if received == b'H' {
            // Répondre "i\n"
            let response = b"i\n";
            for &c in response {
                usart.transmit(c);
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release
// qemu-system-avr -M uno -bios halfred.elf -serial tcp::5678,server=on,wait=off

// cargo +nightly build -Z build-std=core --target thumbv7m-none-eabi --release
// qemu-system-arm -M lm3s6965evb -kernel halfred -serial tcp::5678,server=on,wait=off

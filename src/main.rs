#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(target_arch = "avr")]
mod atmega328_usart;

#[cfg(target_arch = "arm")]
mod cortex_m3_usart;

#[cfg(target_arch = "avr")]
use atmega328_usart::Usart;

#[cfg(target_arch = "arm")]
use cortex_m3_usart::Usart;

#[no_mangle]
pub extern "C" fn main() -> ! {
    #[cfg(target_arch = "avr")]
    Usart::init(9600);

    #[cfg(target_arch = "arm")]
    Usart::init(9600, 16_000_000);

    loop {
        // Envoi de "Hello World\n" caractère par caractère
        let message = b"Hello World\n";
        for &c in message {
            Usart::send(c);
        }
    
        // Réception d'un caractère
        let received = Usart::receive();
        if received == b'H' {
            // Répondre "i\n"
            let response = b"i\n";
            for &c in response {
                Usart::send(c);
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

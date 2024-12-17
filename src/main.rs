#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod usart;
use usart::Usart;




#[cfg(target_arch = "avr")]
use cortex_m_rt::entry;

#[cfg(target_arch = "riscv32")]
use riscv_rt::entry;

#[cfg(target_arch = "riscv32")]
extern crate panic_halt;

#[cfg(target_arch = "riscv32")]
#[entry]
fn main() -> ! {
    program();
}

#[cfg(target_arch = "avr")]
#[entry]
unsafe fn main() -> ! {
    program();
}


pub fn program() -> ! {
    let usart = Usart::new();

    usart.init(9600);

    loop {
        // Envoi de "Hello World\n" caractère par caractère
        let message = b"Hello World\n";
        for &c in message {
            usart.transmit(c);
        }
    }
}

#[cfg(target_arch = "riscv32")]
#[no_mangle]
#[link_section = ".trap"]
pub fn _dispatch_core_interrupt() {
    // Handle core interrupts here
    loop {}
}

#[cfg(target_arch = "avr")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// for sifive1
//cargo build --target riscv32imac-unknown-none-elf --release
//qemu-system-riscv32 -nographic -machine sifive_e -kernel target/riscv32imac-unknown-none-elf/release/halfred

// For atmega328p
// cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release
// qemu-system-avr -M uno -bios target/avr-unknown-gnu-atmega328/release/halfred.elf -nographic

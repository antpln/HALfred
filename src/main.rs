#![no_std]
#![no_main]
use core::panic::PanicInfo;
use cortex_m_rt::entry;
mod gpio;
use gpio::*;

#[entry]
fn main() -> ! {
    let gpio = Gpio::new(5, Atmega_Port::PortB);
    gpio.set_mode(Gpio_Mode::Output);
    gpio.set_level(Gpio_Level::High);
    loop {
        for _ in 0..400000{
            gpio.set_level(Gpio_Level::High);
        }
        for _ in 0..400000{
            gpio.set_level(Gpio_Level::Low);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
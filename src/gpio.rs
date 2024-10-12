#![no_std]
#![no_main]
use core::ptr::{read_volatile, write_volatile};

#[cfg(target_arch = "avr")]
mod atmega328 {
    pub const DDRB: *mut u8 = 0x24 as *mut u8;
    pub const PORTB: *mut u8 = 0x25 as *mut u8;
    pub const PINB: *mut u8 = 0x23 as *mut u8;

    pub const DDRC: *mut u8 = 0x27 as *mut u8;
    pub const PORTC: *mut u8 = 0x28 as *mut u8;
    pub const PINC: *mut u8 = 0x26 as *mut u8;

    pub const DDRD: *mut u8 = 0x2A as *mut u8;
    pub const PORTD: *mut u8 = 0x2B as *mut u8;
    pub const PIND: *mut u8 = 0x29 as *mut u8;
}

#[cfg(target_arch = "xtensa")]
mod esp8266 {
    pub const GPIO_ENABLE_W1TS: *mut u32 = 0x60000310 as *mut u32;
    pub const GPIO_ENABLE_W1TC: *mut u32 = 0x60000314 as *mut u32;
    pub const GPIO_OUT_W1TS: *mut u32 = 0x60000304 as *mut u32;
    pub const GPIO_OUT_W1TC: *mut u32 = 0x60000308 as *mut u32;
    pub const GPIO_IN: *mut u32 = 0x60000318 as *mut u32;
}

#[repr(u8)]
pub enum Gpio_Mode {
    Output = 0,
    Input = 1,
}

#[repr(u8)]
pub enum Gpio_Level {
    Low  = 0,
    High = 1,
}


pub struct Gpio {
    pin: u8,
}

impl Gpio {
    pub fn new(pin: u8) -> Self {
        Gpio { pin }
    }

    pub fn set_mode(&self, mode: Gpio_Mode) {
        unsafe {
            #[cfg(target_arch = "avr")]
            {
                let ddrb = read_volatile(DDRB);
                match mode {
                    Gpio_Mode::Output => write_volatile(DDRB, ddrb | (1 << self.pin)),
                    Gpio_Mode::Input => write_volatile(DDRB, ddrb & !(1 << self.pin)),
                }
            }

            #[cfg(target_arch = "xtensa")]
            {
                match mode {
                    Gpio_Mode::Output => write_volatile(GPIO_ENABLE_W1TS, 1 << self.pin),
                    Gpio_Mode::Input => write_volatile(GPIO_ENABLE_W1TC, 1 << self.pin),
                }
            }
        }
    }

    pub fn set_level(&self, level: Gpio_Level) {
        unsafe {
            #[cfg(target_arch = "avr")]
            {
                let portb = read_volatile(PORTB);
                match level {
                    Gpio_Level::High => write_volatile(PORTB, portb | (1 << self.pin)),
                    Gpio_Level::Low => write_volatile(PORTB, portb & !(1 << self.pin)),
                }
            }

            #[cfg(target_arch = "xtensa")]
            {
                match level {
                    Gpio_Level::High => write_volatile(GPIO_OUT_W1TS, 1 << self.pin),
                    Gpio_Level::Low => write_volatile(GPIO_OUT_W1TC, 1 << self.pin),
                }
            }
        }
    }
}

fn main() {
    let gpio = Gpio::new(3);
    gpio.set_mode(Gpio_Mode::Output);
    gpio.set_level(Gpio_Level::High);
}
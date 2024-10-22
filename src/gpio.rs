#![no_std]
#![no_main]
use core::ptr::{read_volatile, write_volatile};

/* #[cfg(target_arch = "avr")]
mod atmega328 { */
    pub const DDRB: *mut u8 = 0x24 as *mut u8;
    pub const PORTB: *mut u8 = 0x25 as *mut u8;
    pub const PINB: *mut u8 = 0x23 as *mut u8;

    pub const DDRC: *mut u8 = 0x27 as *mut u8;
    pub const PORTC: *mut u8 = 0x28 as *mut u8;
    pub const PINC: *mut u8 = 0x26 as *mut u8;

    pub const DDRD: *mut u8 = 0x2A as *mut u8;
    pub const PORTD: *mut u8 = 0x2B as *mut u8;
    pub const PIND: *mut u8 = 0x29 as *mut u8;
/* } */

/* #[cfg(target_arch = "xtensa")]
mod esp8266 {
    pub const GPIO_ENABLE_W1TS: *mut u32 = 0x60000310 as *mut u32;
    pub const GPIO_ENABLE_W1TC: *mut u32 = 0x60000314 as *mut u32;
    pub const GPIO_OUT_W1TS: *mut u32 = 0x60000304 as *mut u32;
    pub const GPIO_OUT_W1TC: *mut u32 = 0x60000308 as *mut u32;
    pub const GPIO_IN: *mut u32 = 0x60000318 as *mut u32;
} */

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

pub enum Atmega_Port {
    PortB,
    PortC,
    PortD,
}


pub struct Gpio {
    pin: u8,
    pin_reg : *mut u8,
    ddr_reg : *mut u8,
    port_reg : *mut u8,
    
}

impl Gpio {
    pub fn new(pin: u8, port : Atmega_Port) -> Self {
        let mut pin_reg: *mut u8;
        let mut ddr_reg: *mut u8;
        let mut port_reg: *mut u8;

        match port{
            Atmega_Port::PortB => {
                pin_reg = PINB;
                ddr_reg = DDRB;
                port_reg = PORTB;
            },
            Atmega_Port::PortC => {
                pin_reg = PINC;
                ddr_reg = DDRC;
                port_reg = PORTC;
            },
            Atmega_Port::PortD => {
                pin_reg = PIND;
                ddr_reg = DDRD;
                port_reg = PORTD;
            },
        }
        Gpio { pin, pin_reg, ddr_reg, port_reg }
    }

    pub fn set_mode(&self, mode: Gpio_Mode) {
        unsafe {
            #[cfg(target_arch = "avr")]
            {
                let ddr = read_volatile(self.ddr_reg);
                match mode {
                    Gpio_Mode::Output => write_volatile(self.ddr_reg, ddr | (1 << self.pin)),
                    Gpio_Mode::Input => write_volatile(self.ddr_reg, ddr & !(1 << self.pin)),
                }
            }

            /* #[cfg(target_arch = "xtensa")]
            {
                match mode {
                    Gpio_Mode::Output => write_volatile(GPIO_ENABLE_W1TS, 1 << self.pin),
                    Gpio_Mode::Input => write_volatile(GPIO_ENABLE_W1TC, 1 << self.pin),
                }
            } */
        }
    }

    pub fn set_level(&self, level: Gpio_Level) {
        unsafe {
            #[cfg(target_arch = "avr")]
            {
                let portb = read_volatile(self.port_reg);
                match level {
                    Gpio_Level::High => write_volatile(self.port_reg, portb | (1 << self.pin)),
                    Gpio_Level::Low => write_volatile(self.port_reg, portb & !(1 << self.pin)),
                }
            }

            /* #[cfg(target_arch = "xtensa")]
            {
                match level {
                    Gpio_Level::High => write_volatile(GPIO_OUT_W1TS, 1 << self.pin),
                    Gpio_Level::Low => write_volatile(GPIO_OUT_W1TC, 1 << self.pin),
                }
            } */
        }
    }

    pub fn read_level(&self) -> Gpio_Level {
        unsafe {
            #[cfg(target_arch = "avr")]
            {
                let pinb = read_volatile(self.pin_reg);
                if (pinb & (1 << self.pin)) != 0 {
                    return Gpio_Level::High;
                } else {
                    return Gpio_Level::Low;
                }
            }
        }
        return Gpio_Level::Low; // Dummy return
    }

}

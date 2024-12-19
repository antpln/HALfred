use core::ptr::{read_volatile, write_volatile};

// Atmega registers
pub const DDRB: *mut u8 = 0x24 as *mut u8;
pub const PORTB: *mut u8 = 0x25 as *mut u8;
pub const PINB: *mut u8 = 0x23 as *mut u8;

pub const DDRC: *mut u8 = 0x27 as *mut u8;
pub const PORTC: *mut u8 = 0x28 as *mut u8;
pub const PINC: *mut u8 = 0x26 as *mut u8;

pub const DDRD: *mut u8 = 0x2A as *mut u8;
pub const PORTD: *mut u8 = 0x2B as *mut u8;
pub const PIND: *mut u8 = 0x29 as *mut u8;

pub const GPIO_INPUT_VAL: *mut u32 = 0x10012000 as *mut u32;
pub const GPIO_INPUT_EN: *mut u32 = 0x10012004 as *mut u32;
pub const GPIO_OUTPUT_EN: *mut u32 = 0x10012008 as *mut u32;
pub const GPIO_OUTPUT_VAL: *mut u32 = 0x1001200C as *mut u32;

#[repr(u8)]
pub enum GpioMode {
    Output = 0,
    Input = 1,
}

#[repr(u8)]
pub enum GpioLevel {
    Low = 0,
    High = 1,
}

pub struct Gpio {
    pin: u8,
    pin_reg: *mut u8,
    ddr_reg: *mut u8,
    port_reg: *mut u8,
}

impl Gpio {
    #[cfg(any(target_arch = "avr"))]
    pub fn new(pin: u8, port: Port) -> Self {
        assert!(pin < 8, "Pin number must be less than 8 for Atmega ports");
        let (pin_reg, ddr_reg, port_reg) = match port {
            Port::AtmegaPortB => (PINB, DDRB, PORTB),
            Port::AtmegaPortC => (PINC, DDRC, PORTC),
            Port::AtmegaPortD => (PIND, DDRD, PORTD),
        };
        Gpio {
            pin,
            pin_reg,
            ddr_reg,
            port_reg,
        }
    }

    #[cfg(any(target_arch = "riscv32"))]
    pub fn new(pin: u8) -> Self {
        Gpio {
            pin,
            pin_reg: GPIO_INPUT_VAL as *mut u8,
            ddr_reg: GPIO_INPUT_EN as *mut u8,
            port_reg: GPIO_OUTPUT_VAL as *mut u8,
        }
    }

    pub fn set_mode(&self, mode: GpioMode) {
        unsafe {
            #[cfg(any(target_arch = "riscv32"))]
            {
                let input_en = read_volatile(GPIO_INPUT_EN);
                let output_en = read_volatile(GPIO_OUTPUT_EN);
                match mode {
                    GpioMode::Output => {
                        write_volatile(GPIO_OUTPUT_EN, output_en | (1 << self.pin));
                        write_volatile(GPIO_INPUT_EN, input_en & !(1 << self.pin));
                    }
                    GpioMode::Input => {
                        write_volatile(GPIO_INPUT_EN, input_en | (1 << self.pin));
                        write_volatile(GPIO_OUTPUT_EN, output_en & !(1 << self.pin));
                    }
                }
            }

            #[cfg(any(target_arch = "avr"))]
            {
                let ddr = read_volatile(self.ddr_reg);
                match mode {
                    GpioMode::Output => write_volatile(self.ddr_reg, ddr | (1 << self.pin)),
                    GpioMode::Input => write_volatile(self.ddr_reg, ddr & !(1 << self.pin)),
                }
            }
        }
    }

    pub fn set_level(&self, level: GpioLevel) {
        unsafe {
            #[cfg(any(target_arch = "riscv32"))]
            {
                let port = read_volatile(GPIO_OUTPUT_VAL);
                match level {
                    GpioLevel::High => write_volatile(GPIO_OUTPUT_VAL, port | (1 << self.pin)),
                    GpioLevel::Low => write_volatile(GPIO_OUTPUT_VAL, port & !(1 << self.pin)),
                }
            }

            #[cfg(any(target_arch = "avr"))]
            {
                let port = read_volatile(self.port_reg);
                match level {
                    GpioLevel::High => write_volatile(self.port_reg, port | (1 << self.pin)),
                    GpioLevel::Low => write_volatile(self.port_reg, port & !(1 << self.pin)),
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn read_level(&self) -> Option<GpioLevel> {
        unsafe {
            #[cfg(any(target_arch = "riscv32"))]
            {
                let pin = read_volatile(GPIO_INPUT_VAL);
                if (pin & (1 << self.pin)) != 0 {
                    Some(GpioLevel::High)
                } else {
                    Some(GpioLevel::Low)
                }
            }

            #[cfg(any(target_arch = "avr"))]
            {
                let pin = read_volatile(self.pin_reg);
                if (pin & (1 << self.pin)) != 0 {
                    Some(GpioLevel::High)
                } else {
                    Some(GpioLevel::Low)
                }
            }
        }
    }
}

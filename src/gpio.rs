use core::ptr::{read_volatile, write_volatile};

pub const DDRB: *mut u8 = 0x24 as *mut u8;
pub const PORTB: *mut u8 = 0x25 as *mut u8;
pub const PINB: *mut u8 = 0x23 as *mut u8;

pub const DDRC: *mut u8 = 0x27 as *mut u8;
pub const PORTC: *mut u8 = 0x28 as *mut u8;
pub const PINC: *mut u8 = 0x26 as *mut u8;

pub const DDRD: *mut u8 = 0x2A as *mut u8;
pub const PORTD: *mut u8 = 0x2B as *mut u8;
pub const PIND: *mut u8 = 0x29 as *mut u8;

#[repr(u8)]
#[allow(dead_code)]
pub enum GpioMode {
    Output = 0,
    Input = 1,
}

#[repr(u8)]
pub enum GpioLevel {
    Low = 0,
    High = 1,
}

#[allow(dead_code)]
pub enum AtmegaPort {
    PortB,
    PortC,
    PortD,
}

pub struct Gpio {
    pin: u8,
    #[allow(dead_code)]
    pin_reg: *mut u8,
    ddr_reg: *mut u8,
    port_reg: *mut u8,
}

impl Gpio {
    pub fn new(pin: u8, port: AtmegaPort) -> Self {
        let (pin_reg, ddr_reg, port_reg) = match port {
            AtmegaPort::PortB => (PINB, DDRB, PORTB),
            AtmegaPort::PortC => (PINC, DDRC, PORTC),
            AtmegaPort::PortD => (PIND, DDRD, PORTD),
        };
        Gpio {
            pin,
            pin_reg,
            ddr_reg,
            port_reg,
        }
    }

    pub fn set_mode(&self, mode: GpioMode) {
        unsafe {
            let ddr = read_volatile(self.ddr_reg);
            match mode {
                GpioMode::Output => write_volatile(self.ddr_reg, ddr | (1 << self.pin)),
                GpioMode::Input => write_volatile(self.ddr_reg, ddr & !(1 << self.pin)),
            }
        }
    }

    pub fn set_level(&self, level: GpioLevel) {
        unsafe {
            let port = read_volatile(self.port_reg);
            match level {
                GpioLevel::High => write_volatile(self.port_reg, port | (1 << self.pin)),
                GpioLevel::Low => write_volatile(self.port_reg, port & !(1 << self.pin)),
            }
        }
    }

    #[allow(dead_code)]
    pub fn read_level(&self) -> Option<GpioLevel> {
        unsafe {
            let pin = read_volatile(self.pin_reg);
            if (pin & (1 << self.pin)) != 0 {
                Some(GpioLevel::High)
            } else {
                Some(GpioLevel::Low)
            }
        }
    }
}

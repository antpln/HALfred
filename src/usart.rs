#![no_std]
#![no_main]
use core::ptr;


const UBRR0H: *mut u8 = 0xC5 as *mut u8;
const UBRR0L: *mut u8 = 0xC4 as *mut u8;
const UCSR0A: *mut u8 = 0xC0 as *mut u8;
const UCSR0B: *mut u8 = 0xC1 as *mut u8;
const UCSR0C: *mut u8 = 0xC2 as *mut u8;
const UDR0: *mut u8 = 0xC6 as *mut u8;

pub struct Usart;

impl Usart {
    pub fn new() -> Self {
        Usart
    }
    pub fn init(&self, baud_rate: u32) {
        #[cfg(target_arch = "avr")]
        {
            let ubrr = (16000000 / 16 / baud_rate - 1) as u16;
            unsafe {
                ptr::write_volatile(UBRR0H, (ubrr >> 8) as u8);
                ptr::write_volatile(UBRR0L, ubrr as u8);
                ptr::write_volatile(UCSR0B, 1 << 3 | 1 << 4); // Enable RX and TX
                ptr::write_volatile(UCSR0C, 1 << 1 | 1 << 2); // Set frame format: 8 data bits, 1 stop bit
            }
        }
    }

    pub fn transmit(&self,data: u8) {
        #[cfg(target_arch = "avr")]
        unsafe {
            while ptr::read_volatile(UCSR0A) & (1 << 5) == 0 {}
            ptr::write_volatile(UDR0, data);
        }
    }

    pub fn receive(&self) -> u8 {
        #[cfg(target_arch = "avr")]
        unsafe {
            while ptr::read_volatile(UCSR0A) & (1 << 7) == 0 {}
            ptr::read_volatile(UDR0)
        }
    }
}
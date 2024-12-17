#![no_std]
#![no_main]
use core::ptr;


#[cfg(target_arch = "avr")]
const UBRR0H: *mut u8 = 0xC5 as *mut u8;
#[cfg(target_arch = "avr")]
const UBRR0L: *mut u8 = 0xC4 as *mut u8;
#[cfg(target_arch = "avr")]
const UCSR0A: *mut u8 = 0xC0 as *mut u8;
#[cfg(target_arch = "avr")]
const UCSR0B: *mut u8 = 0xC1 as *mut u8;
#[cfg(target_arch = "avr")]
const UCSR0C: *mut u8 = 0xC2 as *mut u8;
#[cfg(target_arch = "avr")]
const UDR0: *mut u8 = 0xC6 as *mut u8;

#[cfg(target_arch = "riscv32")]
const UART0_BASE: u32 = 0x1001_3000;
#[cfg(target_arch = "riscv32")]
const UART0_TXDATA: *mut u32 = (UART0_BASE + 0x00) as *mut u32;
#[cfg(target_arch = "riscv32")]
const UART0_RXDATA: *mut u32 = (UART0_BASE + 0x04) as *mut u32;
#[cfg(target_arch = "riscv32")]
const UART0_TXCTRL: *mut u32 = (UART0_BASE + 0x08) as *mut u32;
#[cfg(target_arch = "riscv32")]
const UART0_RXCTRL: *mut u32 = (UART0_BASE + 0x0C) as *mut u32;
#[cfg(target_arch = "riscv32")]
const UART0_DIV: *mut u32 = (UART0_BASE + 0x18) as *mut u32;
/* [CORRECTION USART] 

You could do something to remove those repetition, like a simple module for example :
#[cfg(target_arch = "riscv32")]
mod usart_register {
    const UART0_BASE: u32 = 0x1001_3000;
    const UART0_TXDATA: *mut u32 = (UART0_BASE + 0x00) as *mut u32;
    const UART0_RXDATA: *mut u32 = (UART0_BASE + 0x04) as *mut u32;
    const UART0_TXCTRL: *mut u32 = (UART0_BASE + 0x08) as *mut u32;
    const UART0_RXCTRL: *mut u32 = (UART0_BASE + 0x0C) as *mut u32;
    const UART0_DIV: *mut u32 = (UART0_BASE + 0x18) as *mut u32;
}

(Don't hesitate to remove this comment)
*/

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

        #[cfg(target_arch = "riscv32")]
        {
            let div = (16000000 / baud_rate) as u32;
            
            unsafe {
                ptr::write_volatile(UART0_DIV, div);
                ptr::write_volatile(UART0_TXCTRL, 1); // Enable TX
                ptr::write_volatile(UART0_RXCTRL, 1); // Enable RX
            }
        }
    }

    pub fn transmit(&self, data: u8) {
        #[cfg(target_arch = "avr")]
        unsafe {
            while ptr::read_volatile(UCSR0A) & (1 << 5) == 0 {}
            ptr::write_volatile(UDR0, data);
        }
        #[cfg(target_arch = "riscv32")]
        unsafe {
            while ptr::read_volatile(UART0_TXDATA) & 0x80000000 != 0 {}
            ptr::write_volatile(UART0_TXDATA, data as u32);
        }
    }

    pub fn receive(&self) -> u8 {
        let mut a : u8 = 0;
        #[cfg(target_arch = "avr")]
        unsafe {
            while ptr::read_volatile(UCSR0A) & (1 << 7) == 0 {}
            a = ptr::read_volatile(UDR0);
        }
        #[cfg(target_arch = "riscv32")]
        unsafe {
            while ptr::read_volatile(UART0_RXDATA) & 0x80000000 != 0 {}
            a = ptr::read_volatile(UART0_RXDATA) as u8;
        }
        return a;
    }
}
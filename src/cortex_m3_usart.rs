use core::ptr::{write_volatile, read_volatile};

// Base des registres UART0
const UART0_DR: *mut u32 = 0x4000C000 as *mut u32;
const UART0_FR: *mut u32 = 0x4000C018 as *mut u32;
const UART0_IBRD: *mut u32 = 0x4000C024 as *mut u32;
const UART0_FBRD: *mut u32 = 0x4000C028 as *mut u32;
const UART0_LCRH: *mut u32 = 0x4000C02C as *mut u32;
const UART0_CR: *mut u32 = 0x4000C030 as *mut u32;

pub struct Usart;

impl Usart {
    pub fn init(baud_rate: u32, clock_freq: u32) {
        unsafe {
            // Désactiver l'UART
            write_volatile(UART0_CR, 0);

            // Configurer le baud rate
            let ibrd = clock_freq / (16 * baud_rate);
            let fbrd = ((clock_freq % (16 * baud_rate)) * 64 + baud_rate / 2) / baud_rate;
            write_volatile(UART0_IBRD, ibrd);
            write_volatile(UART0_FBRD, fbrd);

            // Configurer les paramètres : 8 bits, pas de parité, 1 stop bit
            write_volatile(UART0_LCRH, 0b11 << 5); // WLEN = 3 (8 bits)

            // Activer l'UART, la transmission (TXE) et la réception (RXE)
            write_volatile(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));
        }
    }

    pub fn send(data: u8) {
        unsafe {
            // Attendre que le registre TX soit prêt
            while (read_volatile(UART0_FR) & (1 << 5)) != 0 {}
            write_volatile(UART0_DR, data as u32);
        }
    }

    pub fn receive() -> u8 {
        unsafe {
            // Attendre que des données soient disponibles
            while (read_volatile(UART0_FR) & (1 << 4)) != 0 {}
            read_volatile(UART0_DR) as u8
        }
    }
}

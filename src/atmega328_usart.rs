use core::ptr::{read_volatile, write_volatile};

// Registres pour l'USART sur l'Atmega328p
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // Registre pour configurer le baud rate (partie basse)
const UBRR0H: *mut u8 = 0xC5 as *mut u8; // Registre pour configurer le baud rate (partie haute)
const UCSR0A: *mut u8 = 0xC0 as *mut u8; // Registre de statut
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // Registre pour activer TX/RX
const UCSR0C: *mut u8 = 0xC2 as *mut u8; // Registre pour configurer le format de données
const UDR0: *mut u8 = 0xC6 as *mut u8;   // Registre pour envoyer/recevoir des données

pub struct Usart;

impl Usart {
    pub fn init(baud_rate: u16) {
        unsafe {
            // Configurer le baud rate
            write_volatile(UBRR0H, (baud_rate >> 8) as u8);
            write_volatile(UBRR0L, (baud_rate & 0xFF) as u8);

            // Activer la transmission et la réception
            write_volatile(UCSR0B, (1 << 3) | (1 << 4)); // TXEN et RXEN

            // Configurer 8 bits de données, 1 bit de stop, sans parité
            write_volatile(UCSR0C, (1 << 1) | (1 << 2));
        }
    }

    pub fn send(data: u8) {
        unsafe {
            // Attendre que le registre soit prêt
            while (read_volatile(UCSR0A) & (1 << 5)) == 0 {} // TXE
            write_volatile(UDR0, data);
        }
    }

    pub fn receive() -> u8 {
        unsafe {
            // Attendre que des données soient reçues
            while (read_volatile(UCSR0A) & (1 << 7)) == 0 {} // RXNE
            read_volatile(UDR0)
        }
    }
}

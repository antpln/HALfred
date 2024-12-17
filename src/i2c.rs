use core::ptr::{read_volatile, write_volatile};

#[cfg(target_arch = "avr")]
pub fn i2c_init() {
    // Initialize I2C for ATmega328
    unsafe {
        // Set SCL frequency to 100kHz with 16MHz clock
        write_volatile(0xB8 as *mut u8, 0x20);
        write_volatile(0xB9 as *mut u8, 0x00);
        // Enable TWI
        write_volatile(0xBC as *mut u8, 0x04);
    }
}

#[cfg(target_arch = "riscv32")]
pub fn i2c_init() {
    // Initialize I2C for SiFive HiFive1
    unsafe {
        // Set prescaler for 100kHz with 16MHz clock
        write_volatile(0x10016000 as *mut u32, 0x31);
        // Enable I2C
        write_volatile(0x10016004 as *mut u32, 0x01);
    }
}

#[cfg(target_arch = "avr")]
pub fn i2c_write_byte(addr: u8, data: u8) {
    unsafe {
        // Send START condition
        write_volatile(0xBB as *mut u8, 0xA4);
        // Wait for TWINT flag set
        while read_volatile(0xBB as *mut u8) & 0x80 == 0 {}
        
        // Send address
        write_volatile(0xBA as *mut u8, addr << 1);
        // Wait for TWINT flag set
        while read_volatile(0xBB as *mut u8) & 0x80 == 0 {}
        
        // Send data
        write_volatile(0xBA as *mut u8, data);
        // Wait for TWINT flag set
        while read_volatile(0xBB as *mut u8) & 0x80 == 0 {}
        
        // Send STOP condition
        write_volatile(0xBB as *mut u8, 0x94);
    }
}

#[cfg(target_arch = "riscv32")]
pub fn i2c_write_byte(addr: u8, data: u8) {
    unsafe {
        // Send START condition and address
        write_volatile(0x10016008 as *mut u32, (addr as u32) << 1 | 0x01);
        // Wait for completion
        while read_volatile(0x10016010 as *mut u32) & 0x01 == 0 {}
        
        // Send data
        write_volatile(0x1001600C as *mut u32, data as u32);
        // Wait for completion
        while read_volatile(0x10016010 as *mut u32) & 0x01 == 0 {}
        
        // Send STOP condition
        write_volatile(0x10016008 as *mut u32, 0x02);
    }
}

#[cfg(target_arch = "avr")]
pub fn i2c_read_byte(addr: u8) -> u8 {
    unsafe {
        // Send START condition
        write_volatile(0xBB as *mut u8, 0xA4);
        // Wait for TWINT flag set
        while read_volatile(0xBB as *mut u8) & 0x80 == 0 {}
        
        // Send address with read bit
        write_volatile(0xBA as *mut u8, (addr << 1) | 0x01);
        // Wait for TWINT flag set
        while read_volatile(0xBB as *mut u8) & 0x80 == 0 {}
        
        // Enable ACK and clear TWINT to receive data
        write_volatile(0xBB as *mut u8, 0xC4);
        // Wait for TWINT flag set
        while read_volatile(0xBB as *mut u8) & 0x80 == 0 {}
        
        // Read data
        let data = read_volatile(0xBA as *mut u8);
        
        // Send STOP condition
        write_volatile(0xBB as *mut u8, 0x94);
        
        data
    }
}

#[cfg(target_arch = "riscv32")]
pub fn i2c_read_byte(addr: u8) -> u8 {
    unsafe {
        // Send START condition and address with read bit
        write_volatile(0x10016008 as *mut u32, (addr as u32) << 1 | 0x01);
        // Wait for completion
        while read_volatile(0x10016010 as *mut u32) & 0x01 == 0 {}
        
        // Read data
        let data = read_volatile(0x1001600C as *mut u32) as u8;
        
        // Send STOP condition
        write_volatile(0x10016008 as *mut u32, 0x02);
        
        data
    }
}
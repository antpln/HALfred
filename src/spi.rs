use core::ptr;

#[cfg(target_arch = "avr")]
mod atmega328 {

    pub fn spi_init_master() {
        unsafe {
            // Set MOSI and SCK output, all others input
            ptr::write_volatile(0x24 as *mut u8, 1 << 3 | 1 << 5);
            // Enable SPI, Master, set clock rate fck/16
            ptr::write_volatile(0x4C as *mut u8, 1 << 6 | 1 << 4 | 1 << 0);
        }
    }

    pub fn spi_init_slave() {
        unsafe {
            // Set MISO output, all others input
            ptr::write_volatile(0x24 as *mut u8, 1 << 4);
            // Enable SPI, Slave
            ptr::write_volatile(0x4C as *mut u8, 1 << 6);
        }
    }

    pub fn spi_transmit(data: u8) -> u8 {
        unsafe {
            // Start transmission
            ptr::write_volatile(0x4E as *mut u8, data);
            // Wait for transmission complete
            while ptr::read_volatile(0x4D as *const u8) & (1 << 7) == 0 {}
            // Return data register
            ptr::read_volatile(0x4E as *const u8)
        }
    }

    pub fn spi_receive() -> u8 {
        unsafe {
            // Transmit dummy byte to initiate SPI clock
            ptr::write_volatile(0x4E as *mut u8, 0xFF);
            // Wait for reception complete
            while ptr::read_volatile(0x4D as *const u8) & (1 << 7) == 0 {}
            // Return data register
            ptr::read_volatile(0x4E as *const u8)
        }
    }
}

#[cfg(target_arch = "riscv32")]
mod hifive1 {

    const SPI0_BASE: usize = 0x10014000;
    const SPI_SCKDIV: usize = SPI0_BASE + 0x00;
    const SPI_CSID: usize = SPI0_BASE + 0x10;
    const SPI_CSMODE: usize = SPI0_BASE + 0x18;
    const SPI_TXDATA: usize = SPI0_BASE + 0x48;
    const SPI_RXDATA: usize = SPI0_BASE + 0x4C;
    const SPI_FCTRL: usize = SPI0_BASE + 0x60;
    const SPI_FFMT: usize = SPI0_BASE + 0x64;

    pub fn spi_init_master() {
        unsafe {
            // Set SPI clock divider
            ptr::write_volatile(SPI_SCKDIV as *mut u32, 0x04);
            // Set chip select ID
            ptr::write_volatile(SPI_CSID as *mut u32, 0x00);
            // Set chip select mode
            ptr::write_volatile(SPI_CSMODE as *mut u32, 0x00);
            // Set frame format
            ptr::write_volatile(SPI_FFMT as *mut u32, 0x80000000);
            // Enable SPI
            ptr::write_volatile(SPI_FCTRL as *mut u32, 0x01);
        }
        /*
        [CORRECTION SPI] (don't hesitate to remove this part)
        It is great that you explicit every element like that.
        */
    }

    pub fn spi_init_slave() {
        unsafe {
            // Set chip select mode to auto
            ptr::write_volatile(SPI_CSMODE as *mut u32, 0x02);
            // Set frame format
            ptr::write_volatile(SPI_FFMT as *mut u32, 0x80000000);
            // Enable SPI
            ptr::write_volatile(SPI_FCTRL as *mut u32, 0x01);
        }
    }

    pub fn spi_transmit(data: u8) -> u8 {
        unsafe {
            // Wait until TX FIFO is not full
            while ptr::read_volatile(SPI_TXDATA as *const u32) & 0x80000000 != 0 {}
            // Write data to TX FIFO
            ptr::write_volatile(SPI_TXDATA as *mut u32, data as u32);
            // Wait until RX FIFO is not empty
            while ptr::read_volatile(SPI_RXDATA as *const u32) & 0x80000000 != 0 {}
            // Read data from RX FIFO
            ptr::read_volatile(SPI_RXDATA as *const u32) as u8
        }
    }

    pub fn spi_receive() -> u8 {
        unsafe {
            // Wait until RX FIFO is not empty
            while ptr::read_volatile(SPI_RXDATA as *const u32) & 0x80000000 != 0 {}
            // Read data from RX FIFO
            ptr::read_volatile(SPI_RXDATA as *const u32) as u8
        }
    }
}

#[cfg(target_arch = "avr")]
pub use atmega328::*;

#[cfg(target_arch = "riscv32")]
pub use hifive1::*;
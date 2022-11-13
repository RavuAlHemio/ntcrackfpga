#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_spi: [u8; 0x31],
}
impl RegisterBlock {
    #[doc = "0x00..0x31 - USART Mode"]
    #[inline(always)]
    pub fn usart(&self) -> &USART {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART) }
    }
    #[doc = "0x00..0x31 - SPI Mode"]
    #[inline(always)]
    pub fn spi(&self) -> &SPI {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const SPI) }
    }
    #[doc = "0x00..0x29 - I2C Slave Mode"]
    #[inline(always)]
    pub fn i2cs(&self) -> &I2CS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const I2CS) }
    }
    #[doc = "0x00..0x31 - I2C Master Mode"]
    #[inline(always)]
    pub fn i2cm(&self) -> &I2CM {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const I2CM) }
    }
}
#[doc = "I2C Master Mode"]
pub use self::i2cm::I2CM;
#[doc = r"Cluster"]
#[doc = "I2C Master Mode"]
pub mod i2cm;
#[doc = "I2C Slave Mode"]
pub use self::i2cs::I2CS;
#[doc = r"Cluster"]
#[doc = "I2C Slave Mode"]
pub mod i2cs;
#[doc = "SPI Mode"]
pub use self::spi::SPI;
#[doc = r"Cluster"]
#[doc = "SPI Mode"]
pub mod spi;
#[doc = "USART Mode"]
pub use self::usart::USART;
#[doc = r"Cluster"]
#[doc = "USART Mode"]
pub mod usart;

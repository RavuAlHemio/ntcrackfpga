use atsamd21g18a::Peripherals;

use ardzero::i2c;


const ADDR_CONFIG: u8 = 0x04;
const ADDR_PORT_CONFIG: u8 = 0x08;
const BASE_ADDR_SINGLE_PORT: u8 = 0x20;
const BASE_ADDR_8_PORT_BANK: u8 = 0x40;


#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PinConfig {
    Output,
    InputFloating,
    InputPulledUp,
}
impl PinConfig {
    #[inline]
    pub const fn to_bits(&self) -> u8 {
        match self {
            Self::Output => 0b01,
            Self::InputFloating => 0b10,
            Self::InputPulledUp => 0b11,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PinBankConfig {
    pub pin0: PinConfig,
    pub pin1: PinConfig,
    pub pin2: PinConfig,
    pub pin3: PinConfig,
}
impl PinBankConfig {
    #[inline]
    pub const fn to_byte(&self) -> u8 {
        self.pin3.to_bits() << 6
        | self.pin2.to_bits() << 4
        | self.pin1.to_bits() << 2
        | self.pin0.to_bits() << 0
    }

    #[inline]
    pub const fn new(pin0: PinConfig, pin1: PinConfig, pin2: PinConfig, pin3: PinConfig) -> Self {
        Self { pin0, pin1, pin2, pin3 }
    }
}


pub struct Max7300 {
    address: u8,
}
impl Max7300 {
    pub fn new(address: u8) -> Self {
        assert_eq!(address & 0b1000_0000, 0);
        Self {
            address,
        }
    }

    pub fn config(
        &self,
        peripherals: &mut Peripherals,
        power_on: bool,
        transition_detection: bool,
    ) {
        let config_byte: u8 =
            if power_on { 1 << 0 } else { 0 }
            | if transition_detection { 1 << 7 } else { 0 }
        ;
        i2c::write(
            peripherals, self.address,
            &[ADDR_CONFIG, config_byte],
        );
    }

    pub fn config_pins(
        &self,
        peripherals: &mut Peripherals,
        first_bank: u8,
        configs: &[PinBankConfig],
    ) {
        let mut config_bytes = [0u8; 8+1];
        config_bytes[0] = ADDR_PORT_CONFIG + first_bank;
        for (i, config) in configs.into_iter().enumerate() {
            config_bytes[i+1] = config.to_byte();
        }

        i2c::write(
            peripherals, self.address,
            &config_bytes[0..configs.len()+1],
        );
    }

    pub fn write_pins(
        &self,
        peripherals: &mut Peripherals,
        first_pin: u8,
        pin_values: &[bool],
    ) {
        assert!(pin_values.len() <= 0x20);
        let mut bytes = [0u8; 0x21];
        bytes[0] = BASE_ADDR_SINGLE_PORT + first_pin;
        for (i, val) in pin_values.into_iter().enumerate() {
            bytes[i + 1] = if *val { 1 } else { 0 };
        }

        i2c::write(
            peripherals, self.address,
            &bytes[0..pin_values.len()+1],
        );
    }

    pub fn write_pin_bank(
        &self,
        peripherals: &mut Peripherals,
        bank: u8,
        value: u8,
    ) {
        let bytes = [BASE_ADDR_8_PORT_BANK + bank, value];
        i2c::write(
            peripherals, self.address,
            &bytes,
        );
    }

    pub fn read_pin(
        &self,
        peripherals: &mut Peripherals,
        pin: u8,
    ) -> bool {
        // write address
        i2c::write(
            peripherals, self.address,
            &[BASE_ADDR_SINGLE_PORT + pin],
        );

        // read from register at that address
        let mut val = 0;
        i2c::read(peripherals, self.address, |b| {
            val = b;
            false // read only the one byte
        });
        val != 0
    }

    pub fn read_pin_bank(
        &self,
        peripherals: &mut Peripherals,
        bank: u8,
    ) -> u8 {
        i2c::write(
            peripherals, self.address,
            &[BASE_ADDR_8_PORT_BANK + bank],
        );
        let mut val = 0;
        i2c::read(peripherals, self.address, |b| {
            val = b;
            false // read only the one byte
        });
        val
    }
}

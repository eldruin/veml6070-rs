//! This is a platform agnostic Rust driver for the VEML6070 UVA light
//! sensor, based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Enable/disable the sensor
//! - Read the UV measurement
//! - Set the integration time
//! - Enable/disable ACK signal
//! - Set ACK threshold value
//!
//! ## The device
//! VEML6070 is an advanced ultraviolet (UV) light sensor with I2C protocol
//! interface and designed by the CMOS process.
//! It is easily operated via a simple I2C command. The active acknowledge
//! (ACK) feature with threshold windows setting allows the UV sensor to
//! send out a UVI alert message. Under a strong solar UVI condition, the
//! smart ACK signal can be easily implemented by the software programming.
//! VEML6070 incorporates a photodiode, amplifiers, and analog / digital
//! circuits into a single chip. VEML6070's adoption of Filtron TM UV
//! technology provides the best spectral sensitivity to cover UV spectrum
//! sensing. It has an excellent temperature compensation and a robust refresh
//! rate setting that does not use an external RC low pass filter.
//! VEML6070 has linear sensitivity to solar UV light and is easily adjusted
//! by an external resistor.
//! Software shutdown mode is provided, which reduces power consumption to
//! be less than 1 μA. VEML6070's operating voltage ranges from 2.7 V to 5.5 V.
//!
//! Datasheet:
//! - [VEML6070](https://www.vishay.com/docs/84277/veml6070.pdf)
//!
//! Application note:
//! - [VEML6070 AN](https://www.vishay.com/docs/84310/designingveml6070.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read UV
//!
//! Import this crate and an `embedded_hal` implementation, then instantiate
//! the device:
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6070::Veml6070;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut uv_light_sensor = Veml6070::new(dev);
//! // initialization step is necessary
//! uv_light_sensor.init().unwrap();
//! uv_light_sensor.enable().unwrap();
//! let _uv_reading = uv_light_sensor.read_uv().unwrap();
//! ```
//!
//! ### Set integration time
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6070::{Veml6070, IntegrationTime};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut uv_light_sensor = Veml6070::new(dev);
//! // initialization step is necessary
//! uv_light_sensor.init().unwrap();
//! uv_light_sensor.enable().unwrap();
//! uv_light_sensor.set_integration_time(IntegrationTime::T1).unwrap();
//! ```
//!
//! ### Enable ACK and set a threshold of 145 steps
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use veml6070::{Veml6070, AckThreshold};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut uv_light_sensor = Veml6070::new(dev);
//! // initialization step is necessary
//! uv_light_sensor.init().unwrap();
//! uv_light_sensor.enable().unwrap();
//! uv_light_sensor.enable_ack_with_threshold(AckThreshold::Steps145).unwrap();
//! ```

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use embedded_hal::blocking::i2c;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Integration time
#[derive(Debug, Clone)]
pub enum IntegrationTime {
    /// Half T
    HalfT,
    /// 1 T
    T1,
    /// 2 T
    T2,
    /// 4 T
    T4,
}

/// ACK threshold
#[derive(Debug, Clone)]
pub enum AckThreshold {
    /// 102 steps
    Steps102,
    /// 145 steps
    Steps145,
}

struct BitFlags;

impl BitFlags {
    const SHUTDOWN: u8 = 0b0000_0001;
    const IT0: u8 = 0b0000_0100;
    const IT1: u8 = 0b0000_1000;
    const ACK_THD: u8 = 0b0001_0000;
    const ACK: u8 = 0b0010_0000;
}

struct Address;

impl Address {
    const ARA: u8 = 0x0C;
    const COMMAND: u8 = 0x38;
    const DATA_MSB: u8 = 0x39;
    const DATA_LSB: u8 = 0x38;
}

/// VEML6070 device driver.
#[derive(Debug, Default)]
pub struct Veml6070<I2C> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    /// Command register status.
    cmd: u8,
}

impl<I2C, E> Veml6070<I2C>
where
    I2C: i2c::Write<Error = E>,
{
    /// Create new instance of the VEML6070 device.
    pub fn new(i2c: I2C) -> Self {
        Veml6070 { i2c, cmd: 0x02 }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Enable the sensor.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let cmd = self.cmd;
        self.write_command(cmd & !BitFlags::SHUTDOWN)
    }

    /// Disable the sensor (shutdown).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let cmd = self.cmd;
        self.write_command(cmd | BitFlags::SHUTDOWN)
    }

    /// Set integration time.
    pub fn set_integration_time(&mut self, it: IntegrationTime) -> Result<(), Error<E>> {
        let mut cmd = self.cmd;
        cmd = match it {
            IntegrationTime::HalfT => cmd & !BitFlags::IT0 & !BitFlags::IT1,
            IntegrationTime::T1 => cmd | BitFlags::IT0 & !BitFlags::IT1,
            IntegrationTime::T2 => cmd & !BitFlags::IT0 | BitFlags::IT1,
            IntegrationTime::T4 => cmd | BitFlags::IT0 | BitFlags::IT1,
        };
        self.write_command(cmd)
    }

    /// Enable the ACK signal.
    ///
    /// *Note:* The ACK must be cleared every time after it has fired with `clear_ack()`.
    pub fn enable_ack(&mut self) -> Result<(), Error<E>> {
        let cmd = self.cmd;
        self.write_command(cmd | BitFlags::ACK)
    }

    /// Disable the ACK signal.
    pub fn disable_ack(&mut self) -> Result<(), Error<E>> {
        let cmd = self.cmd;
        self.write_command(cmd & !BitFlags::ACK)
    }

    /// Set ACK threshold.
    pub fn set_ack_threshold(&mut self, threshold: AckThreshold) -> Result<(), Error<E>> {
        let cmd = self.cmd;
        self.write_command(handle_ack_threshold_bit(cmd, threshold))
    }

    /// Enable the ACK signal and set the ACK threshold at once.
    ///
    /// *Note:* The ACK must be cleared every time after it has fired with `clear_ack()`.
    pub fn enable_ack_with_threshold(&mut self, threshold: AckThreshold) -> Result<(), Error<E>> {
        let mut cmd = self.cmd;
        cmd |= BitFlags::ACK;
        self.write_command(handle_ack_threshold_bit(cmd, threshold))
    }

    fn write_command(&mut self, cmd: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(Address::COMMAND, &[cmd])
            .map_err(Error::I2C)?;
        self.cmd = cmd;
        Ok(())
    }
}

fn handle_ack_threshold_bit(cmd: u8, threshold: AckThreshold) -> u8 {
    match threshold {
        AckThreshold::Steps102 => cmd & !BitFlags::ACK_THD,
        AckThreshold::Steps145 => cmd | BitFlags::ACK_THD,
    }
}

impl<I2C, E> Veml6070<I2C>
where
    I2C: i2c::Read<Error = E>,
{
    /// Clear ACK status.
    ///
    /// *Note:* The ACK status must be cleared every time after it has fired.
    /// Other registers will be blocked otherwise.
    pub fn clear_ack(&mut self) -> Result<(), Error<E>> {
        let mut buffer = [0];
        self.i2c.read(Address::ARA, &mut buffer).map_err(Error::I2C)
    }

    /// Read the UV sensor.
    pub fn read_uv(&mut self) -> Result<u16, Error<E>> {
        let mut msb = [0];
        let mut lsb = [0];
        self.i2c
            .read(Address::DATA_MSB, &mut msb)
            .map_err(Error::I2C)?;
        self.i2c
            .read(Address::DATA_LSB, &mut lsb)
            .map_err(Error::I2C)?;
        Ok(u16::from(msb[0]) << 8 | u16::from(lsb[0]))
    }
}

impl<I2C, E> Veml6070<I2C>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E>,
{
    /// Initialize and clear ACK.
    pub fn init(&mut self) -> Result<(), Error<E>> {
        self.clear_ack()?;
        let cmd = 0x02; // default setting
        self.write_command(cmd)
    }
}

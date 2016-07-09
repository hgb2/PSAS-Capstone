// use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use std::fmt;

pub struct LinuxI2CDevice {
    path: String,
    slave_address: u16,
}

pub enum Error {
    Unexpected(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sensor error")
    }
}

pub enum I2CError {
    NotSupported,
    Other(&'static str),
}

pub type I2CResult<T> = Result<T, I2CError>;

impl LinuxI2CDevice {
    fn new(path: &'static str, slave_address: u16) -> Result<LinuxI2CDevice, Error> {
        Ok(LinuxI2CDevice {
            path: String::from(path),
            slave_address: slave_address,
        })
    }

    pub fn read(&mut self, data: &mut [u8]) -> I2CResult<()> {
        Ok(())
    }

    pub fn write(&mut self, data: &[u8]) -> I2CResult<()> {
        Ok(())
    }
}

pub fn init(path: &'static str, slave_address: u16) -> Result<LinuxI2CDevice, i32> {
    let mut x = LinuxI2CDevice::new(path, slave_address);
    match x {
        Ok(y) => return Ok(y),
        Err(_) => return Err(1),
    }
}

pub fn update(x: i32) -> i32 {
    println!("test mode i2c update received {}", x);
    x
}

use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

pub fn init(path: &'static str, slave_address: u16) -> Result<LinuxI2CDevice, i32> {
    //i2cdev::linux::LinuxI2CDevice::new(path, slave_address)
    let mut x = LinuxI2CDevice::new(path, slave_address);
    match x {
        Ok(y) => return Ok(y),
        Err(_) => return Err(1),
    }
}

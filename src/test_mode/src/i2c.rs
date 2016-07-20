use std::io;

use wrapper;

pub struct SimulationI2CDevice {
    fdm: *mut wrapper::FDM,
}

impl SimulationI2CDevice {
    fn new() -> Result<SimulationI2CDevice, io::Error> {
        Ok(SimulationI2CDevice { fdm: wrapper::init() })
    }
}

impl Drop for SimulationI2CDevice {
    fn drop(&mut self) {
        println!("Closing Null I2C Device");
        wrapper::close(self.fdm);
    }
}

pub struct Myi2c {
    pub i2c: SimulationI2CDevice,
}

impl Myi2c {
    pub fn init() -> Result<Myi2c, io::Error> {
        return Ok(Myi2c { i2c: try!(SimulationI2CDevice::new()) });
    }

    pub fn write(&mut self, reg: &[u8]) -> Result<(), io::Error> {
        return Ok(());
    }

    pub fn read(&mut self, mut buf: &mut [u8]) -> Result<(), io::Error> {
        return Ok(());
    }
}

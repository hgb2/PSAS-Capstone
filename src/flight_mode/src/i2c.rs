use i2cdev::core::*;
use i2cdev::linux::*;
use std::io;



pub struct Myi2c {
    pub i2c: LinuxI2CDevice,
}

impl Myi2c {

    pub fn init() -> Result<Myi2c, io::Error> {
        let mut dev = try!(LinuxI2CDevice::new("/dev/i2c-1", 0x68));

        // Try and read from the WhoAmI register.
        // This should return a 0x68 if this is a compatable device (i.e. MPU-6050 or the MPU-9150)
        let mut buf = [0u8; 1];
        try!(dev.write(&[0x75]));
        try!(dev.read(&mut buf));
    	if buf[0] != 0x68 {
    		return Err(io::Error::new(io::ErrorKind::NotFound, "MPU-6050 WhoAmI returned wrong value").into());
    	}

        // Wake device up, using internal oscillator.
        try!(dev.write(&[0x6b, 0x00]));

        // Set configuration:
        // - Sample rate divider: 1kHz / 200
        // - Config: no FSYNC, low-pass filter at 5Hz
        // - Gyro config: full scale range at +/- 250 dps
        // - Accel config: full scale range at +/- 2g
        try!(dev.write(&[0x19, 199, 0x06, 0x00, 0x00]));

        return Ok(Myi2c{i2c: dev});
    }

    pub fn write (&mut self, reg: &[u8]) -> Result<(), io::Error> {
        try!(self.i2c.write(reg));
        return Ok(());
    }

    pub fn read(&mut self, mut buf: &mut [u8]) -> Result<(), io::Error> {
        try!(self.i2c.read(buf));
        return Ok(());
    }
}

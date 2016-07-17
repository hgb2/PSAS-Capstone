use i2cdev::core::*;
use i2cdev::linux::*;
use std::io;

fn read_reg(bus: &mut LinuxI2CDevice, reg: u8, buf: &mut [u8]) -> Result<(), io::Error> {
    try!(bus.write(&[reg]));
    try!(bus.read(buf));
    return Ok(());
}


pub fn init() -> Result<LinuxI2CDevice, io::Error> {
    let mut dev = try!(LinuxI2CDevice::new("/dev/i2c-1", 0x68));

    // Try and read from the WhoAmI register.
    // This should return a 0x68 if this is a compatable device (i.e. MPU-6050 or the MPU-9150)
    let mut buf = [0u8; 1];
	try!(read_reg(&mut dev, 0x75, &mut buf));
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

    return Ok(dev);
}

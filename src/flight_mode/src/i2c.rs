
//use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use i2cdev::core::*;
use i2cdev::linux::*;

fn read_reg(bus: &mut LinuxI2CDevice, reg: u8, buf: &mut [u8]) {
    bus.write(&[reg]); // 0x43 is the beginning address of the block of registers that we want to read
    bus.read(buf); // puts block (buf.length) of registers in buf (accel, temp, and gyro.
}


pub fn init() -> Result<LinuxI2CDevice, String> {
    let mut x = LinuxI2CDevice::new("/dev/i2c-1", 0x68);
    let mut dev;
    match x {
        Ok(y) => dev = y,
        Err(e) => return Err(format!("Failed to open the i2c device")),
    }



    let mut buf = [0u8; 1];
	read_reg(&mut dev, 0x75, &mut buf);
	if buf[0] != 0x68 {
		return Err(format!("MPU-9150 WhoAmI returned wrong value"));
	}

    // Wake device up, using internal oscillator.
    dev.write(&[0x6b, 0x00]);

    // Set configuration:
    // - Sample rate divider: 1kHz / 200
    // - Config: no FSYNC, low-pass filter at 5Hz
    // - Gyro config: full scale range at +/- 250 dps
    // - Accel config: full scale range at +/- 2g
    dev.write(&[0x19, 199, 0x06, 0x00, 0x00]);

    return Ok(dev);
}

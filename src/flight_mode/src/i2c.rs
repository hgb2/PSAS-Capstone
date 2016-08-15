extern crate byteorder;
use self::byteorder::{BigEndian, ReadBytesExt};
use i2cdev::core::*;
use i2cdev::linux::*;
use std::io;

pub struct Myi2c {
    pub i2c: LinuxI2CDevice,
}

impl Myi2c {

    pub fn init() -> Result<Myi2c, io::Error> {
        let mut dev = try!(LinuxI2CDevice::new("/dev/i2c-6", 0x68));

        // Try and read from the WhoAmI register.
        // This should return a 0x68 if this is a compatible device (i.e. MPU-6050 or the MPU-9150)
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

    pub fn get_gyro(&mut self) -> Result<(f32, f32, f32), io::Error> {
        // 1 temp (Registers 41-42), 3 gyro (Registers 43-48)
        //only using the gyro registers for now.
        //buf: &mut [u8]
        let mut buf = [0u8; (3) * 2];

        // 0x43 is the beginning address of the block of registers that we want to read
        try!(self.i2c.write(&[0x43]));
        try!(self.i2c.read(&mut buf));

        let mut rdr = io::Cursor::new(buf);

        //114.3 degrees per second (/114.3 when sensitivity is set to 250 dps)
        //Or this could be /131.0 degrees per second
        let gyro_x = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;
        let gyro_y = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;
        let gyro_z = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;

        Ok((gyro_x, gyro_y, gyro_z))
    }
}

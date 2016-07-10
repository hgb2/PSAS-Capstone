extern crate byteorder;
extern crate i2cdev;

use libs::i2c;
use self::byteorder::{BigEndian, ReadBytesExt};
use SharedMemory;
use self::i2cdev::core::*;
use self::i2cdev::linux::*;
use std::io;





pub struct Sensor_Module {
    pub i2c: LinuxI2CDevice,
}

impl Sensor_Module {

	//On Ok it returns a Sensor_Module object
	pub fn init() -> Result<Sensor_Module, String> {
		match i2c::init() {
			Ok(x) => return Ok(Sensor_Module{i2c: x}),
			Err(e) => return Err(e),
		}
	}


	fn read_reg(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), String> {
		self.i2c.write(&[reg]); // 0x43 is the beginning address of the block of registers that we want to read
		self.i2c.read(buf); // puts block (buf.length) of registers in buf (accel, temp, and gyro.
            return Ok(());
	}


    pub fn update(&mut self, mem: &mut SharedMemory) -> Result<(), String> {
        println!("sensor update");

        // 3 accel (Registers 3b-40),
        // 1 temp (Registers 41-42), 3 gyro (Registers 43-48)
        //only using the gyro registers for now.
        //buf: &mut [u8]
        let mut buf = [0u8; (3) * 2];

        try!(self.read_reg(0x43, &mut buf));

        let mut rdr = io::Cursor::new(buf);

        match rdr.read_i16::<BigEndian>() {
            Ok(n) => mem.gyro_x = (n as f32) / 131.0,
            Err(e) => println!("{}", "There was an error"),
        }

        match rdr.read_i16::<BigEndian>() {
            Ok(n) => mem.gyro_y = (n as f32) / 131.0,
            Err(e) => println!("{}", "There was an error"),
        }

        match rdr.read_i16::<BigEndian>() {
            Ok(n) => mem.gyro_z = (n as f32) / 131.0,
            Err(e) => println!("{}", "There was an error"),
        }
        return Ok(());
    }
}

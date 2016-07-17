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
	pub fn init() -> Result<Sensor_Module, io::Error> {
		match i2c::init() {
			Ok(x) => return Ok(Sensor_Module{i2c: x}),
			Err(e) => return Err(e),
		}
	}


	fn read_reg(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), io::Error> {
		try!(self.i2c.write(&[reg])); // 0x43 is the beginning address of the block of registers that we want to read
		try!(self.i2c.read(buf)); // puts block (buf.length) of registers in buf (accel, temp, and gyro.
        return Ok(());
	}


    pub fn update(&mut self, mem: &mut SharedMemory) -> Result<(), io::Error> {
        println!("sensor update");

        // 3 accel (Registers 3b-40),
        // 1 temp (Registers 41-42), 3 gyro (Registers 43-48)
        //only using the gyro registers for now.
        //buf: &mut [u8]
        let mut buf = [0u8; (3) * 2];

        try!(self.read_reg(0x43, &mut buf));

        let mut rdr = io::Cursor::new(buf);

        //114.3 degrees per second (/114.3 when sensitivity is set to 250 dps)
        //Or this could be /131.0 degrees per second
        mem.gyro_x = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;
	    mem.gyro_y = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;
		mem.gyro_z = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;


        return Ok(());
    }
}

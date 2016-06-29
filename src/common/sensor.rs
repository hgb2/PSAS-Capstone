extern crate byteorder;
extern crate i2cdev;

use libs::i2c;
use self::byteorder::{BigEndian, ReadBytesExt};
use SharedMemory;
use self::i2cdev::core::*;
use self::i2cdev::linux::*;
use std::io;
use UpdateResult;


//On Ok it returns a Sensor_Module object
pub fn init() -> Result<Sensor_Module, i32> {
	match i2c::init("/dev/i2c-0/", 0x68) {
		//This should work with Brians part.
		Ok(x) => return Ok(Sensor_Module::new(x)),
		//This will return whatever error the i2c module sends to it
		Err(_) => return Err(-1),
	}


}


//This will be replaced with the above function when the i2c lib is done.
//pub fn init() {
//	i2c::init();
//}

pub fn update(mem: &mut SharedMemory) -> UpdateResult {
    println!("sensor update");
    i2c::update(30);

    mem.gyro_x = 114.75;

    Ok(0)
}


pub struct Sensor_Module {
    pub i2c: LinuxI2CDevice,
}

impl Sensor_Module {

		pub fn new(myi2c: LinuxI2CDevice) -> Sensor_Module{
			return Sensor_Module{i2c: myi2c};
		}

	fn read_reg(&mut self, reg: u8, buf: &mut [u8]) {
		self.i2c.write(&[reg]); // 0x43 is the beginning address of the block of registers that we want to read
		self.i2c.read(buf); // puts block (buf.length) of registers in buf (accel, temp, and gyro.
	}

    pub fn update(&mut self, mem: &mut SharedMemory) {
        println!("sensor update");

        // 3 accel (Registers 3b-40),
        // 1 temp (Registers 41-42), 3 gyro (Registers 43-48)
        //only using the gyro registers for now.
		//buf: &mut [u8]
        let mut buf = [0u8; (3) * 2];

		self.read_reg(0x43, &mut buf);

        let mut rdr = io::Cursor::new(buf);

		//This will eventually be done with the try! macro just wanted it to work for now.
        match rdr.read_i16::<BigEndian>() {
            Ok(n) => mem.gyro_x = (n as f32) / 131.0,
            Err(e) => println!("{}", "There was an error"),
        }



    	//mem.gyro_x = (rdr.read_i16::<BigEndian>() as f32) / 131.0;
    	//mem.gyro_y = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0,
    	//mem.gyro_z = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0,
    }
}

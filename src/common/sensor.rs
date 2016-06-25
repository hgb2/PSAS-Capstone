extern crate byteorder;
//extern crate i2cdev;

use self::byteorder::{BigEndian, ReadBytesExt};

use libs::i2c;
use SharedMemory;
//use i2cdev::core::*;
//use i2cdev::linux::*;
use std::io;

pub fn init() {
    i2c::init();
=======
use UpdateResult;

pub fn init() {
	i2c::init()
}

pub fn update(mem: &mut SharedMemory) -> UpdateResult {
    println!("sensor update");
    i2c::update(30);

    mem.gyro_x = 114.75;

    Ok(0)
}


pub struct Sensor_Module {
//    myi2c: LinuxI2CDevice,
}

impl Sensor_Module {
    pub fn update(mem: &mut SharedMemory) {
        println!("sensor update");

        // 3 accel (Registers 3b-40),
        // 1 temp (Registers 41-42), 3 gyro (Registers 43-48)
        //only using the gyro registers for now.
        let mut buf = [0u8; (3) * 2];

        //myi2c.write(0x43) // 0x3b is the beginning address of the block of registers that we want to read
        //myi2c.read(&buf); // puts block (buf.length) of registers in buf (accel, temp, and gyro)


        let mut rdr = io::Cursor::new(buf);

        match rdr.read_i16::<BigEndian>() {
            Ok(n) => mem.gyro_x = (n as f32) / 131.0,
            Err(e) => println!("{}", "There was an error"),
        }

    	//mem.gyro_x = (rdr.read_i16::<BigEndian>() as f32) / 131.0;
    	//mem.gyro_y = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0,
    	//mem.gyro_z = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0,


    }

}

///////////////////////////////////////////////////////////////////////////////
// File Name: sensor.rs
//
// Purpose: The sensor module reads gyroscope data from the sensors
//          the writes it to shared memory. 
//
///////////////////////////////////////////////////////////////////////////////


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
pub fn init() -> Result<Sensor_Module, io::Error> {
	match i2c::init() {
		Ok(x) => return Ok(Sensor_Module{i2c: x}),
		Err(e) => return Err(e),
	}
}

fn read_reg(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), io::Error> {
	try!(self.i2c.write(&[reg]));
	try!(self.i2c.read(buf));
    return Ok(());
}

///////////////////////////////////////////////////////////////////////////////
// Function Name: update
//
// Purpose: Reads gyroscope data from i2cDev then converts it to usable data
//          and writes it to shared memory.
//
// INPUTS: mem -- reference to shared memory
//
// RETURNS: Ok(()) -- empty tuple
//          Err(err) -- io::Error
//
///////////////////////////////////////////////////////////////////////////////
pub fn update(&mut self, mem: &mut SharedMemory) -> Result<(), io::Error> {
    println!("sensor update");

    // 1 temp (Registers 41-42), 3 gyro (Registers 43-48)
    //only using the gyro registers for now.
    //buf: &mut [u8]
    let mut buf = [0u8; (3) * 2];

    try!(self.read_reg(0x43, &mut buf)); // 0x43 is the beginning address of the block of registers that we want to read
    // 3 accel (Registers 3b-40),


    let mut rdr = io::Cursor::new(buf);

    //114.3 degrees per second (/114.3 when sensitivity is set to 250 dps)
    //Or this could be /131.0 degrees per second
    mem.gyro_x = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;
    mem.gyro_y = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;
	mem.gyro_z = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;

    return Ok(());
}
}

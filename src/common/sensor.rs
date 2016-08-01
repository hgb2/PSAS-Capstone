///////////////////////////////////////////////////////////////////////////////
// File Name: sensor.rs
//
// Purpose: The sensor module reads gyroscope data from the sensors
//          the writes it to shared memory.
//
///////////////////////////////////////////////////////////////////////////////


extern crate byteorder;

use libs::i2c::Myi2c;
use self::byteorder::{BigEndian, ReadBytesExt};
use SharedMemory;
use std::io;

// MyPins are only used in test mode
use libs::gpio::MyPins;


pub struct SensorModule {
    pub i2c: Myi2c,
}


impl SensorModule {
pub fn init(pins: &MyPins) -> Result<SensorModule, io::Error> {
    match Myi2c::init(pins) {
        Ok(x) => return Ok(SensorModule { i2c: x }),
        Err(e) => return Err(e),
    }
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

    // 0x43 is the beginning address of the block of registers that we want to read
    try!(self.i2c.write(&[0x43]));
      try!(self.i2c.read(&mut buf));

    let mut rdr = io::Cursor::new(buf);

    //114.3 degrees per second (/114.3 when sensitivity is set to 250 dps)
    //Or this could be /131.0 degrees per second
    mem.gyro_x = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;
    mem.gyro_y = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;
    mem.gyro_z = (try!(rdr.read_i16::<BigEndian>()) as f32) / 131.0;

    return Ok(());
}
}

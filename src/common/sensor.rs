///////////////////////////////////////////////////////////////////////////////
// File Name: sensor.rs
//
// Purpose: The sensor module reads gyroscope data from the sensors
//          the writes it to shared memory.
//
///////////////////////////////////////////////////////////////////////////////

use libs::i2c::Myi2c;
use SharedMemory;
use std::io;

pub struct SensorModule {
    pub i2c: Myi2c,
}

impl SensorModule {
pub fn init() -> Result<SensorModule, io::Error> {
    match Myi2c::init() {
        Ok(x) => return Ok(SensorModule{i2c: x}),
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

    let gyro = try!(self.i2c.get_gyro());
    mem.gyro_x = gyro.0;
    mem.gyro_y = gyro.1;
    mem.gyro_z = gyro.2;

    return Ok(());
}
}

use std::io;

use libs::i2c;
use SharedMemory;
use UpdateResult;
use libs::i2c::LinuxI2CDevice;


// On Ok it returns a Sensor_Module object
pub fn init() -> Result<Sensor_Module, i32> {
    match i2c::init("/dev/i2c-0/", 0x68) {
        // This should work with Brians part.
        Ok(x) => return Ok(Sensor_Module::new(x)),
        // This will return whatever error the i2c module sends to it
        Err(_) => return Err(-1),
    }
}

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
    }
}


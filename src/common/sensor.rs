use libs::i2c;
use SharedMemory;

pub fn init() {
	i2c::init() 
}

pub fn update(mem: &mut SharedMemory) -> i32 {
    println!("sensor update");
    i2c::update(30);

    mem.gyro_x = 114.75;
    return 0;
}


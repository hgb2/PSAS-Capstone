use libs::i2c;
use SharedMemory;
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


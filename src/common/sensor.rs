use libs::i2c;
use SharedMemory;
use UpdateResult;

pub fn init() {
	let x = i2c::init("/root/a/place", 123);
}

pub fn update(mem: &mut SharedMemory) -> UpdateResult {
    println!("sensor update");

    mem.gyro_x = 114.75;
    
    Ok(0)
}


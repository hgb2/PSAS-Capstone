use libs::i2c;
use SharedMemory;

pub fn init() {
}

pub fn update(mem: &mut SharedMemory) {
    println!("sensor update");
    i2c::update(30);

    mem.gyro_x = 114.75;
}


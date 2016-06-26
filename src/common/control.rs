use libs::gpio;
use SharedMemory;
use UpdateResult;

pub fn init() {
	gpio::init()
}

pub fn update(mem: &mut SharedMemory) -> UpdateResult {
    println!("control update");
    gpio::update(42);

    mem.cw_state = 1;
    mem.ccw_state = 2;
    Ok(0)
}


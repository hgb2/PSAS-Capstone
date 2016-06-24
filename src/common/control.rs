use libs::gpio;
use SharedMemory;

pub fn init() {
	gpio::init()
}

pub fn update(mem: &mut SharedMemory) -> i32 {
    println!("control update");
    gpio::update(42);

    mem.cw_state = 1;
    mem.ccw_state = 2;
    return 0;
}


use libs::gpio;
use SharedMemory;
use UpdateResult;


pub fn init() {
	let cw_pin = gpio::init(54); // example pin number 54
}

pub fn update(mem: &mut SharedMemory) -> UpdateResult {
    println!("control update");

    mem.cw_state = 1;
    mem.ccw_state = 2;
    Ok(0)
}


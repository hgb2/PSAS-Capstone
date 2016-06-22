use libs::gpio;
use SharedMemory;

pub fn init(x: i32) {
    println!("control init received {}", x);
    gpio::init();
}

pub fn update(mem: &mut SharedMemory) {
    println!("control update");
    gpio::update(42);

    mem.cw_state = 1;
    mem.ccw_state = 2;
}


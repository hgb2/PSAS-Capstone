use SharedMemory;

pub fn init() {
}

pub fn update(mem: &SharedMemory, socket: i32) {
    println!("data formatter update socket: {}", socket);

    println!("shared memory contains:");
    println!("  gyro: {}, {}, {}", mem.gyro_x, mem.gyro_y, mem.gyro_z);
    println!("  pin states: {}, {}", mem.cw_state, mem.ccw_state);
}



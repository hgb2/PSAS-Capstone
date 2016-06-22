extern crate libs;

mod control;
mod sensor;
mod data_fmt;

use std::thread;

pub struct SharedMemory {
    gyro_x:    f32,
    gyro_y:    f32,
    gyro_z:    f32,
    cw_state:  u8,
    ccw_state: u8,
}

fn main() {

    println!("main function");
    println!("");

    let addr = 0x8675309;
    sensor::init(addr);
    control::init(addr);
    data_fmt::init(addr);
    println!("");

    let mut i = 0;
    let socket = 0x12345;
    let mut mem = SharedMemory{gyro_x: 1.1, gyro_y: 2.2, gyro_z: 3.3,
                               cw_state: 0, ccw_state: 0};

    loop {
        if i == 3 { break; }

        sensor::update(&mut mem);
        control::update(&mut mem);
        data_fmt::update(&mem, socket);
        println!("");

        i += 1;
        thread::sleep_ms(1000)
    }
}

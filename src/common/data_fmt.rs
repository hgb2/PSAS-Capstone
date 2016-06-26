use SharedMemory;
use std::net::UdpSocket;
use UpdateResult;

pub fn init() {
}

pub fn send_packet(socket: &UdpSocket, mem: &SharedMemory) -> UpdateResult {

    println!("shared memory contains:");
    println!("  gyro: {}, {}, {}", mem.gyro_x, mem.gyro_y, mem.gyro_z);
    println!("  pin states: {}, {}", mem.cw_state, mem.ccw_state);
    Ok(0)
}



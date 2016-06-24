use std::net::UdpSocket;

extern crate libs;
extern crate libc;

mod control;
mod sensor;
mod data_fmt;

// External c clock method
mod ffi {
  extern {
    pub fn clock() -> ::libc::clock_t;
  }
}

// Add: use control_interface::Control; soon
// Shared memory structure 
pub struct SharedMemory {
    gyro_x:    f32,
    gyro_y:    f32,
    gyro_z:    f32,
    cw_state:  u8,
    ccw_state: u8,
}

fn main() {
    println!("main function\n");

    let mut mem = SharedMemory{gyro_x: 0.0, gyro_y: 0.0, gyro_z: 0.0,
                               cw_state: 0, ccw_state: 0};

    // Timestep variables
    // The timestep in microseconds (500000 -> 500 milliseconds) -- Update when JSBSim timestep 
    let expected_timestep = 100000; // Should translate to 10 Hz. Replace with let expected_timestep = 8333; to translate to 120 Hz
    let mut running = true;
    let mut previous_time;
    let mut current_time = unsafe { ffi::clock() };
    let mut time_since_last = unsafe { ffi::clock() };

    sensor::init(); // Replace with let mut sen = sensor::init(&mut mem); soon
    control::init();        // Replace with let mut ctl = control::init(); soon

    let mut socket = UdpSocket::bind("0.0.0.0:0").unwrap(); // Update with correct IP/Port later

    while running{
        // Update time variables
        previous_time = current_time;
        current_time = unsafe { ffi::clock() };     
        time_since_last = time_since_last + current_time-previous_time;

        while time_since_last >= expected_timestep {
          if sensor::update(&mut mem)==1{ // Replace with sen.update(&mut mem); soon
            println!("Error during sensor update.\n");
            running = false;
            break;
          }
          if control::update(&mut mem)==1{
            println!("Error during control update.\n");
            running = false;
            break;
          }
          data_fmt::update(&mem, &socket);
          // Decrease by expected timestep
          time_since_last -= expected_timestep;
          println!("\n"); // Remove this when done testing otherwise outputting to console is a bottleneck
        }

    }
}
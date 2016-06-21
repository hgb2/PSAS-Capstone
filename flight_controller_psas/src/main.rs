extern crate lib;
extern crate libc;

mod control_interface;
mod sensor_interface;
mod data_formatter;
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
    let expected_timestep = 8333; // This should translate to 120 HZ
    let running = true;
    let mut previous_time;
    let mut current_time = unsafe { ffi::clock() };
    let mut time_since_last = unsafe { ffi::clock() };

    sensor_interface::init(&mut mem); // Replace with let mut sen = sensor_interface::init(&mut mem); soon
    control_interface::init();        // Replace with let mut ctl = control_interface::init(); soon
    data_formatter::init(&mut mem);

    let socket = 0x12345;    

    while running{
        // Update time variables
        previous_time = current_time;
        current_time = unsafe { ffi::clock() };	    
        time_since_last = time_since_last + current_time-previous_time;

        while time_since_last >= expected_timestep {
          sensor_interface::update(&mut mem); // Replace with sen.update(&mut mem); soon
          control_interface::update(&mut mem);
          data_formatter::update(&mem, socket);
          // Decrease by expected timestep
          time_since_last -= expected_timestep;
          //println!("\n"); // Remove this when done testing otherwise outputting to console is a bottleneck
        }

    }
}

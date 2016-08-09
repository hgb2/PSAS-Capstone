use std::net::UdpSocket;
use time::precise_time_s;
use std::time::Instant;
use sensor::SensorModule;

extern crate libs;
extern crate libc;
extern crate time;

mod control;
use control::Control;
mod sensor;
mod data_fmt;
mod config_reader;

// Results
pub type UpdateResult = Result<i32, i32>;

// Add: use control_interface::Control; soon
// Shared memory structure
pub struct SharedMemory {
    gyro_x:    f32,
    gyro_y:    f32,
    gyro_z:    f32,
    cw_state:  u8,
    ccw_state: u8,
    boot_time: Instant,
    sequence_number: u32,
    telemetry_buffer: Vec<u8>,    // Buffer of messages to build a telemetry Packet
}

//Pin values from configuration file.
pub struct ConfigPins {
    cw_pin: u64,
    ccw_pin: u64,
    estop_pin: u64,    
}


fn main() {
    println!("main function\n");

    let mut mem = SharedMemory{gyro_x: 0.0, gyro_y: 0.0, gyro_z: 0.0,
                               cw_state: 0, ccw_state: 0, sequence_number: 0,
                               boot_time: std::time::Instant::now(), 
                               telemetry_buffer: Vec::with_capacity(1432)};

    // Timestep variables
    let Hz :f64 = 2.0;  // Define the Hz to be used -- Using 2 Hz for testing
    let expected_timestep = 1.0/Hz; // Inverse of frequency
    let mut running = true;
    let mut previous_time;
    let mut current_time = precise_time_s();
    let mut time_since_last : f64 = 0.0;
     
    let mut config_pins = ConfigPins { cw_pin: 0, ccw_pin: 0, estop_pin: 0};
    
    match config_reader::xml_reader(&mut config_pins) {
        Err(val) => {
            panic!("XMLEvent error from configuration file: {}", val);
        }
        Ok(_) => (),
    }
    
//*REMOVE LATER*
    print!("\ncw_pin from Pin_Config: {}\n", config_pins.cw_pin);
    print!("ccw_pin from Pin_Config: {}\n", config_pins.ccw_pin);
    print!("estop_pin from Pin_Config: {}\n", config_pins.estop_pin);
//    

    let mut sen: SensorModule;

    match SensorModule::init() {
        Ok(s) => sen = s,
        Err(e) => {
            panic!(e);
        },
    }

    let mut ctl = Control::init();
    
    let socket: UdpSocket;
    match UdpSocket::bind(("127.0.0.1:1234")) {
        Ok(sock) => { socket = sock; },
        Err(e) => { panic!(e) },
    }
	
    while running{
        // Update time variables
        previous_time = current_time;
        current_time = precise_time_s();
        time_since_last = time_since_last + current_time-previous_time;

        while time_since_last >= expected_timestep {
          match sen.update(&mut mem){ // Replace with sen.update(&mut mem); soon
            Err(val) => {
                println!("Sensor update error with code: {}", val);
                running = false;
                break;
            }
            Ok(_) => (),
          }
          match ctl.update(&mut mem) {
            Err(err) => {
                println!("Control update error: {}", err);
                running = false;
                break;
            }
            Ok(val) => if val == control::SHUT_DOWN {
                println!("Main received shut down signal from control module.");
                running = false;
                break;
            }
          }
          match data_fmt::send_packet(&socket, &mut mem){
            Err(val) => {
                println!("Error inside Data Formatter: {}", val);
                running = false;
                break;
            }
            Ok(_) => (),
          }
          // Decrease by expected timestep
          time_since_last -= expected_timestep;
          println!("\n"); // Remove this when done testing otherwise outputting to console is a bottleneck
        }

    }
}

// Run as: cargo test -- --nocapture to see useful output about cycles
#[test]
fn timestep(){
    let Hz :f64 = 2.0;  // Define the HZ to be used
    let mut freq = 0;
    let mut cycles : f64 = 0.0;

    let expected_timestep = 1.0/Hz; // Inverse of frequency
    let mut running = true;
    let mut previous_time;
    let mut current_time = precise_time_s();
    let mut time_since_last : f64 = 0.0;
    let mut elapsed_time= precise_time_s()- precise_time_s();


    while elapsed_time<=10.0{ // Run for 10 seconds
        // Update time variables
        previous_time = current_time;
        current_time = precise_time_s();
        time_since_last = time_since_last + current_time-previous_time;
        elapsed_time += current_time-previous_time;
        while time_since_last >= expected_timestep {
        println!("Cycles: {}", cycles);
          cycles+=1.0;
          // Decrease by expected timestep
          time_since_last -= expected_timestep;
        }
    }
    println!("Cycles: {}", cycles);
    println!("Time: {}", elapsed_time);
    assert_eq!(within(0.0001, cycles/(elapsed_time), Hz), true); // Accept if the frequency is within .0001 Hz
}

// Since time libraries can only be so precise, I use this to give a little bit of error
fn within(error : f64, value : f64, expected : f64) -> bool{
    if value < expected + error && value > expected - error{
        return true;
    }
    return false;
}

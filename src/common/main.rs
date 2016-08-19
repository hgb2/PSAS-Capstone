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

fn main() {
    println!("main function\n");

    let mut mem = SharedMemory{gyro_x: 0.0, gyro_y: 0.0, gyro_z: 0.0,
                               cw_state: 0, ccw_state: 0, sequence_number: 0,
                               boot_time: std::time::Instant::now(),
                               telemetry_buffer: Vec::with_capacity(1432)};

    // Timestep variables
    let Hz :f64 = 100.0;  // Define the Hz to be used -- Using 2 Hz for testing
    let expected_timestep = 1.0/Hz; // Inverse of frequency
    let mut running = true;
    let mut previous_time;
    let mut current_time = precise_time_s();
    let mut time_since_last : f64 = 0.0;

    let mut sen = SensorModule::init().unwrap();

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
          match sen.update(&mut mem){
            Err(val) => {
                println!("Sensor update error with code: {}", val);
                running = false;
                break;
            }
            Ok(_) => println!("{} {} {}", mem.gyro_x, mem.gyro_y, mem.gyro_z),
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

// This test takes awhile to run, use "cargo run -- --ignored" to run longer tests
// Run as: cargo test -- --nocapture to see useful output about cycles
// To run this test and get output, use 'cargo test -- --ignored --nocapture'
#[test]
#[ignore]
fn timestep(){
    let Hz :f64 = 2.0;  // Define the HZ to be used
    // let mut freq = 0; // unused for now
    let mut cycles : f64 = 0.0;

    let expected_timestep = 1.0/Hz; // Inverse of frequency
    // let mut running = true; // unused for now
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
// Not really dead code, but only used on tests
#[allow(dead_code)]
fn within(error : f64, value : f64, expected : f64) -> bool{
    if value < expected + error && value > expected - error{
        return true;
    }
    return false;
}

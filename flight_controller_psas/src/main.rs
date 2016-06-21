extern crate lib;

use lib::*;

fn main() {
    println!("Hello world");
    test();
    control_interface::hello();
    control_module::hello();
    sensor_interface::hello();
    sensor_module::hello();
    jsbsim::hello();
    embedded::hello();
    data_formatter::hello();
}
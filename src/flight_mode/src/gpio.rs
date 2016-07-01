
//! GPIO access under Linux using the rust-embedded sysfs_gpio library 
//! (https://github.com/rust-embedded/rust-sysfs-gpio)
//!
//! The methods exposed by this module are centered around
//! the `MyPins` struct.

use sysfs_gpio::{Pin, Direction};
use std::collections::LinkedList;

struct Node {
    num: u64,
    dir: Direction,
    io: Pin,
}
pub struct MyPins {
    pins: LinkedList<Node>,
}

/// Unexports any pins that were added to MyPins.
impl Drop for MyPins {
    fn drop(&mut self) {
        for pin in &self.pins {
            if let Err(err) = pin.io.unexport() {
                println!("gpio pin {} unexport error: {}", pin.num, err);
            }
        }
    }
}


impl MyPins {

/// Create a new MyPins object. MyPins can contain multiple GPIO pins
/// that are created with the `add_pin` method.
pub fn new() -> MyPins {
    MyPins {
        pins: LinkedList::new(),
    }
}


/// Used to add GPIO pins to a MyPins object. Sets the direction of 
/// the pin and exports it.
///
/// Inputs:
///
///    `pin_number` -- the desired GPIO pin number
///
///    `direction` -- `"in"` configures a pin as an input, 
///                   `"out"` configures a pin as an output, 
///                   `"high"` configures pin as an output and sets its value to 1, and 
///                   `"low"` configures pin as an output and sets its value to 0.
///
/// # Panics
/// 1) User does not have root privileges.
/// 2) Invalid 'direction' argument.
/// 3) The system does not support the GPIO sysfs interface.
/// 4) The requested GPIO is out of range and cannot be exported.
/// 5) The requested GPIO is in use by the kernel and cannot
///    be exported by use in userspace.
pub fn add_pin(&mut self, pin_number: u64, direction: &str) {

    let pin_dir = match direction.trim() {
                         "in" => Direction::In,
                         "out" => Direction::Out,
                         "high" => Direction::High,
                         "low" => Direction::Low,
                         other => panic!("invalid gpio pin direction {}", other)};

    let node = Node {num: pin_number,
                     dir: pin_dir,
                     io: Pin::new(pin_number)};

    if let Err(err) = node.io.export() {
        panic!("error exporting gpio pin {}: {}", pin_number, err);
    }

    if let Err(err) = node.io.set_direction(match direction.trim() {
                         "in" => Direction::In,
                         "out" => Direction::Out,
                         "high" => Direction::High,
                         "low" => Direction::Low,
                         other => panic!("invalid gpio pin direction {}", other)})
    {
        panic!("error setting gpio pin {} direction: {}", pin_number, err);
    }

    self.pins.push_back(node);
}


/// Get the value of the a pin (0 or 1)
///
/// If successful, 1 will be returned if the pin is high
/// and 0 will be returned if the pin is low.
///
/// Inputs:
///
///    `pin_number` -- the desired GPIO pin number
///
/// # Errors
/// 1) Attempt to read an uninitialized pin.
/// 2) Underlying library had a problem reading the pin.
pub fn get_value(&mut self, pin_number: u64) -> Result<u8, String> {

    for pin in &self.pins {
        if pin.num == pin_number {
            match pin.io.get_value() {
                Ok(val) => return Ok(val),
                Err(err) => return Err(format!("bad read from gpio pin {}: {}", pin.num, err)),
            }
        }
    }
    Err(format!("attempt to read uninitialized gpio pin {}", pin_number))
}


/// Set the value of a pin
///
/// This will set the value of the pin either high or low.
///
/// Inputs:
///
///    `pin_number` -- the desired GPIO pin number
///
///    `value` -- A 0 value will set the pin low and any other value will
///               set the pin high (1 is typical).
///
/// # Errors
/// 1) Attempt to write to an uninitialized pin.
/// 2) Attempt to write to an input pin.
/// 3) Underlying library had a problem writing to the pin.
pub fn set_value(&mut self, pin_number: u64, value: u8) -> Result<(), String> {
    for pin in &self.pins {
        if pin.num == pin_number {
            if pin.dir == Direction::In {
                return Err(format!("attempt to write to gpio input pin {}", pin.num));
            }
            if let Err(err) = pin.io.set_value(value) {
                return Err(format!("bad write to gpio pin {}: {}", pin.num, err));
            }
            return Ok(());
        }
    }
    Err(format!("attempt to write to uninitialized gpio pin {}", pin_number))
}

}

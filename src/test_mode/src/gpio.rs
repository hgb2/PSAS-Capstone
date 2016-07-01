
//! GPIO access to JSBSim.
//!
//! The methods exposed by this module are centered around
//! the `MyPins` struct.

use std::collections::LinkedList;

struct Node {
    num: u64,
    dir: String,
}
pub struct MyPins {
    pins: LinkedList<Node>,
}

/// TODO: add any necessary cleanup code here
/// or remove the Drop Trait
impl Drop for MyPins {
    fn drop(&mut self) {
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


/// Used to add GPIO pins to a MyPins object and sets its direction.
///
/// Inputs:
///
///    `pin_number` -- the desired GPIO pin number
///
///    `direction` -- `"in"` configures a pin as an input, 
///                   `"out"` configures a pin as an output, 
///                   `"high"` configures pin as an output and sets its value to 1, and 
///                   `"low"` configures pin as an output and sets its value to 0.
pub fn add_pin(&mut self, pin_number: u64, direction: &str) {
    let node = Node { num: pin_number, dir: direction.trim().to_string() };
    self.pins.push_back(node);
}


/// Get the value of the a pin (0 or 1)
///
/// For the current design, 0 is returned to keep on running and 
/// 1 is returned to exit the program gracefully.
///
/// Inputs:
///
///    `pin_number` -- the desired GPIO pin number
///
/// # Errors
/// 1) Attempt to read an uninitialized pin.
/// 2) TBD: something bad happened in JSBSim?
pub fn get_value(&mut self, pin_number: u64) -> Result<u8, String> {

    for pin in &self.pins {
        if pin.num == pin_number {
            //println!("get_value found pin: {}", pin.num);

            // In the current design, this is only called for the ESTOP
            // pin. Return 0 to keep running. Return 1 to allow
            // the program to exit gracefully.
            return Ok(0);
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
/// 3) TBD: something bad happened in JSBSim?
pub fn set_value(&mut self, pin_number: u64, value: u8) -> Result<(), String> {
    for pin in &self.pins {
        if pin.num == pin_number {
            //println!("set_value found pin: {}", pin.num);
            if pin.dir == "in" {
                return Err(format!("attempt to write to gpio input pin {}", pin.num));
            }
            // TODO: send 'value' to JSBSim

            return Ok(());
        }
    }
    Err(format!("attempt to write to uninitialized gpio pin {}", pin_number))
}

}

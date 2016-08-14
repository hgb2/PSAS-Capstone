//! GPIO access to JSBSim.
//!
//! The methods exposed by this module are centered around
//! the `MyPins` struct.

use std::vec::Vec;
use wrapper;

// These pin numbers should match what common/control.rs does.
// There is probably a better way to do this...
const CW: u64 = 53;
const CCW: u64 = 54;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    In,
    Out,
    High,
    Low,
}


struct Node {
    num: u64,
    dir: Direction,
    state: u8,
}

pub struct MyPins {
    pins: Vec<Node>,
    cw_state: u8,
    ccw_state: u8,
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
            pins: Vec::new(),
            cw_state: 0,
            ccw_state: 0,
        }
    }


    /// Used to add GPIO pins to a MyPins object and sets its direction.
    ///
    /// Inputs:
    ///
    ///    `pin_number` -- the desired GPIO pin number
    ///
    ///    `direction` -- `In` configures a pin as an input, 
    ///                   `Out` configures a pin as an output, 
    ///                   `High` configures pin as an output and sets its value to 1, and 
    ///                   `Low` configures pin as an output and sets its value to 0.
    pub fn add_pin(&mut self, pin_number: u64, direction: Direction) {
        let initial_state = match direction {
            Direction::High => 1,
            _ => 0, // otherwise set the initial state to low
        };
        let node = Node { num: pin_number, dir: direction, state: initial_state };
        self.pins.push(node);
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
                return Ok(pin.state);
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
        for pin in &mut self.pins {
            if pin.num == pin_number {
                //println!("set_value found pin: {}", pin.num);
                if pin.dir == Direction::In {
                    return Err(format!("attempt to write to gpio input pin {}", pin.num));
                }
                pin.state = value;
                // update the direction states if needed
                if pin.num == CW {
                    self.cw_state = value;
                } else if pin.num == CCW {
                    self.ccw_state = value;
                }
                // For the current design, the pin that just changed state was
                // probably CW or CCW, so tell JSBSim about it.
                wrapper::send_to_jsbsim(self.cw_state, self.ccw_state);

                return Ok(());
            }
        }
        Err(format!("attempt to write to uninitialized gpio pin {}", pin_number))
    }
}


#[test]
fn test_new_pin() {
    let mut p = MyPins::new();
    assert_eq!(p.pins.len(), 0);
}

#[test]
fn test_multiple_pins() {
    let mut p = MyPins::new();
    p.add_pin(0, Direction::In);
    p.add_pin(42, Direction::In);
    p.add_pin(43, Direction::In);
    assert_eq!(p.pins.len(), 3);
}

#[test]
#[should_panic]
fn test_fail_on_get_pin_value() {
    let mut p = MyPins::new();
    let v = p.get_value(42).unwrap();
}

#[test]
fn test_set_pin() {
    let mut p = MyPins::new();
    p.add_pin(42, Direction::In);
    p.set_value(42, 0);
    let v = p.get_value(42).unwrap();
    assert_eq!(v, 0);
}

#[test]
fn convert_dir_works() {
    assert_eq!(convert_dir(Direction::In), sysfs_gpio::Direction::In);
    assert_eq!(convert_dir(Direction::Out), sysfs_gpio::Direction::Out);
    assert_eq!(convert_dir(Direction::High), sysfs_gpio::Direction::High);
    assert_eq!(convert_dir(Direction::Low), sysfs_gpio::Direction::Low);
}

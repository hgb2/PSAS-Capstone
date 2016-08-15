//! GPIO access under Linux using the rust-embedded sysfs_gpio library
//! (https://github.com/rust-embedded/rust-sysfs-gpio)
//!
//! The methods exposed by this module are centered around
//! the `MyPins` struct.

use sysfs_gpio;
use std::vec::Vec;

/// Direction is included to mimic sysfs_gpio::Direction,
/// This is useful to keep dependencies under control
pub enum Direction {
    In,
    Out,
    High,
    Low,
}


// Convert gpio::Direction to sysfs_gpio::Direction. This is
// required to keep the common components compatible with
// both flight and test modes. That is, common component control.rs
// can't know anything about sysfs_gpio because we can't include
// sysfs_gpio in test mode.
fn convert_dir(dir: Direction) -> sysfs_gpio::Direction {
    match dir {
        Direction::In => sysfs_gpio::Direction::In,
        Direction::Out => sysfs_gpio::Direction::Out,
        Direction::High => sysfs_gpio::Direction::High,
        Direction::Low => sysfs_gpio::Direction::Low,
    }
}

pub struct MyPins {
    pins: Vec<sysfs_gpio::Pin>,
}

/// Unexports any pins that were added to MyPins.
impl Drop for MyPins {
    fn drop(&mut self) {
        for pin in &self.pins {
            if let Err(err) = pin.unexport() {
                println!("gpio pin {} unexport error: {}", pin.get_pin_num(), err);
            }
        }
    }
}


impl MyPins {

    /// Create a new MyPins object. MyPins can contain multiple GPIO pins
    /// that are created with the `add_pin` method.
    pub fn new() -> MyPins {
        MyPins {
            pins: Vec::new(),
        }
    }


    /// Used to add GPIO pins to a MyPins object. Sets the direction of
    /// the pin and exports it.
    ///
    /// Inputs:
    ///
    ///    `pin_number` -- the desired GPIO pin number
    ///
    ///    `direction` -- `In`   configures a pin as an input,
    ///                   `Out`  configures a pin as an output,
    ///                   `High` configures pin as an output and sets its value to 1, and
    ///                   `Low`  configures pin as an output and sets its value to 0.
    ///
    /// # Panics
    /// 1) User does not have root privileges.
    /// 2) Invalid 'direction' argument.
    /// 3) The system does not support the GPIO sysfs interface.
    /// 4) The requested GPIO is out of range and cannot be exported.
    /// 5) The requested GPIO is in use by the kernel and cannot
    ///    be exported by use in userspace.
    pub fn add_pin(&mut self, pin_number: u64, direction: Direction) {

        let pin = sysfs_gpio::Pin::new(pin_number);

        if let Err(err) = pin.export() {
            panic!("error exporting gpio pin {}: {}", pin_number, err);
        }

        if let Err(err) = pin.set_direction(convert_dir(direction)) {
            panic!("error setting gpio pin {} direction: {}", pin_number, err);
        }

        self.pins.push(pin);
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
            if pin.get_pin_num() == pin_number {
                match pin.get_value() {
                    Ok(val) => return Ok(val),
                    Err(err) => return Err(format!("bad read from gpio pin {}: {}",
                                                    pin.get_pin_num(), err)),
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
            if pin.get_pin_num() == pin_number {
                match pin.get_direction() {
                    Ok(val) => if val == sysfs_gpio::Direction::In {
                                   return Err(format!("attempt to write to gpio input pin {}",
                                                       pin.get_pin_num()));
                               },

                    Err(err) => return Err(format!("can't read direction from gpio pin {}: {}",
                                                    pin.get_pin_num(), err)),
                }

                if let Err(err) = pin.set_value(value) {
                    return Err(format!("bad write to gpio pin {}: {}",
                                        pin.get_pin_num(), err));
                }
                return Ok(());
            }
        }
        Err(format!("attempt to write to uninitialized gpio pin {}", pin_number))
    }
}

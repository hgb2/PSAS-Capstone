//! GPIO access is emulated with the JSBSim wrapper library
//!
//! The methods exposed by this module are centered around
//! the `MyPins` struct.


use std::fmt;
use std::io;
use std::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    In,
    Out,
    High,
    Low,
}

// Used source enum for gpio: https://github.com/rust-embedded/rust-sysfs-gpio/blob/master/src/error.rs
enum Error {
    /// Simple IO error
    Io(io::Error),
    /// Read unusual data from sysfs file.
    Unexpected(String),
    /// Invalid Path
    InvalidPath(String),
}


impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pin error")
    }
}

struct Pin {
    num: u64,
    dir: Direction,
    value: u8,
}

pub struct MyPins {
    pins: Vec<Pin>,
}

impl Pin {
    fn new(number: u64) -> Pin {
        Pin {
            num: number,
            dir: Direction::In,
            value: 0u8,
        }
    }

    fn export(&self) -> Result<(), Error> {
        println!("export called on pin {}", self.num);
        Ok(())
    }

    fn unexport(&self) -> Result<(), Error> {
        println!("unexport called on pin {}", self.num);
        Ok(())
    }
    
    fn get_direction(&self) -> Result<Direction, Error> {
    	return Ok(self.dir.clone());
    }

    fn get_value(&self) -> Result<u8, Error> {
        return Ok(self.value);
    }

    fn get_pin_num(&self) -> u64 {
        return self.num;
    }

    fn set_direction(&self, dir: Direction) -> Result<(), Error> {
        // self.dir = dir;
        Ok(())
    }

    fn set_value(&self, value: u8) -> Result<(), Error> {
        // self.value = value;
        Ok(())
    }
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
        MyPins { pins: Vec::new() }
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

        let pin = Pin::new(pin_number);

        if let Err(err) = pin.export() {
            panic!("error exporting gpio pin {}: {}", pin_number, err);
        }

        if let Err(err) = pin.set_direction(direction) {
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
                    Err(err) => {
                        return Err(format!("bad read from gpio pin {}: {}", pin.get_pin_num(), err))
                    }
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
                    Ok(val) => {
                        if val == Direction::In {
                            return Err(format!("attempt to write to gpio input pin {}",
                                               pin.get_pin_num()));
                        }
                    }

                    Err(err) => {
                        return Err(format!("can't read direction from gpio pin {}: {}",
                                           pin.get_pin_num(),
                                           err))
                    }
                }

                if let Err(err) = pin.set_value(value) {
                    return Err(format!("bad write to gpio pin {}: {}", pin.get_pin_num(), err));
                }
                return Ok(());
            }
        }
        Err(format!("attempt to write to uninitialized gpio pin {}", pin_number))
    }
}

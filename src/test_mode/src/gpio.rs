use std::collections::LinkedList;
use std::fmt;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    In,
    Out,
    High,
    Low,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Pin {
    num: u64,
    dir: Direction,
    value: u8,
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

pub type Result<T> = ::std::result::Result<T, Error>;

impl Pin {
    fn new(number: u64) -> Pin {
        Pin { num: number, dir: Direction::In, value: 0u8 }
    }
    
    fn export(&self) -> Result<()> {
        println!("export called on pin {}", self.num);
        Ok(())
    }
     
    fn get_value(&self) -> Result<u8> {
    	return Ok(self.value);
    }
    
    fn get_pin_num(&self) -> u64 {
    	return self.num;
    }
   
    fn set_direction(&self, dir: Direction) -> Result<()> {
    	//self.dir = dir;
    	Ok(())
    } 
    
    fn set_value(&self, value: u8) -> Result<()> {
    	//self.value = value;
    	Ok(())
    } 
}

struct Node {
    num: u64,
    dir: Direction,
    io: Pin,
}
pub struct MyPins {
    pins: LinkedList<Node>,
}


impl Drop for MyPins {
    fn drop(&mut self) {
        // Flight Mode:
        // for pin in &self.pins {
        //  if let Err(err) = pin.io.unexport() {
        //   println!("gpio pin {} unexport error: {}", pin.num, err);
        //  }
        // }
        // Test Mode: no-op
        println!("drop called on pins");
    }
}


impl MyPins {
    pub fn new() -> MyPins {
        MyPins { pins: LinkedList::new() }
    }

    pub fn add_pin(&mut self, pin_num: u64, direction: &str) {

        let pin_dir = match direction.trim() {
            "in" => Direction::In,
            "out" => Direction::Out,
            "high" => Direction::High,
            "low" => Direction::Low,
            other => panic!("invalid gpio pin direction {}", other),
        };

        let node = Node {
            num: pin_num,
            dir: pin_dir,
            io: Pin::new(pin_num),
        };

        if let Err(err) = node.io.export() {
            panic!("error exporting gpio pin {}: {}", pin_num, err);
        }

        if let Err(err) = node.io.set_direction(match direction.trim() {
            "in" => Direction::In,
            "out" => Direction::Out,
            "high" => Direction::High,
            "low" => Direction::Low,
            other => panic!("invalid gpio pin direction {}", other),
        }) {
            panic!("error setting gpio pin {} direction: {}", pin_num, err);
        }

        self.pins.push_back(node);
    }

    pub fn get_value(&mut self, pin_number: u64) -> u8 {

        for pin in &self.pins {
            if pin.num == pin_number {
                // println!("get_value found pin: {}", pin.num);
                match pin.io.get_value() {
                    Ok(val) => return val,
                    Err(err) => panic!("bad gpio read on pin {}: {}", pin.num, err),
                }
            }
        }
        panic!("gpio pin {} not initialized", pin_number);
    }

    pub fn set_value(&mut self, pin_number: u64, value: u8) {
        for pin in &self.pins {
            if pin.num == pin_number {
                // println!("set_value found pin: {}", pin.num);
               
                if pin.dir == Direction::In {
                    panic!("pin {} is an input", pin.num);
                }
                if let Err(err) = pin.io.set_value(value) {
                    panic!("bad gpio write on pin {}: {}", pin.num, err);
                } 
                return;
            }
        }
        panic!("gpio pin {} not initialized", pin_number);
    }
}

pub fn init() {
	println!("initializing GPIO test stub");
}

pub fn update(num: u64) {
	println!("initializing GPIO test stub pin {}", num);
}

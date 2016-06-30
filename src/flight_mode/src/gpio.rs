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

pub fn new() -> MyPins {
    MyPins {
        pins: LinkedList::new(),
    }
}

pub fn add_pin(&mut self, pin_num: u64, direction: &str) {

    let pin_dir = match direction.trim() {
                         "in" => Direction::In,
                         "out" => Direction::Out,
                         "high" => Direction::High,
                         "low" => Direction::Low,
                         other => panic!("invalid gpio pin direction {}", other)};

    let node = Node {num: pin_num,
                     dir: pin_dir,
                     io: Pin::new(pin_num)};

    if let Err(err) = node.io.export() {
        panic!("error exporting gpio pin {}: {}", pin_num, err);
    }

    if let Err(err) = node.io.set_direction(match direction.trim() {
                         "in" => Direction::In,
                         "out" => Direction::Out,
                         "high" => Direction::High,
                         "low" => Direction::Low,
                         other => panic!("invalid gpio pin direction {}", other)})
    {
        panic!("error setting gpio pin {} direction: {}", pin_num, err);
    }

    self.pins.push_back(node);
}

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


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

pub fn get_value(&mut self, pin_number: u64) -> u8 {

    for pin in &self.pins {
        if pin.num == pin_number {
            //println!("get_value found pin: {}", pin.num);
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
            //println!("set_value found pin: {}", pin.num);
            if pin.dir == Direction::In { panic!("pin {} is an input", pin.num); }
            if let Err(err) = pin.io.set_value(value) {
                panic!("bad gpio write on pin {}: {}", pin.num, err);
            }
            return;
        }
    }
    panic!("gpio pin {} not initialized", pin_number);
}

}


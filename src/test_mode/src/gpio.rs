use std::collections::LinkedList;

struct Node {
    num: u64,
    dir: String,
}
pub struct MyPins {
    pins: LinkedList<Node>,
}

impl Drop for MyPins {
    fn drop(&mut self) {
        //TODO: add any necessary cleanup code here
    }
}

impl MyPins {

pub fn new() -> MyPins {
    MyPins {
        pins: LinkedList::new(),
    }
}

pub fn add_pin(&mut self, pin_num: u64, direction: &str) {
    let node = Node { num: pin_num, dir: direction.trim().to_string() };
    self.pins.push_back(node);
}


pub fn get_value(&mut self, pin_number: u64) -> u8 {

    for pin in &self.pins {
        if pin.num == pin_number {
            println!("get_value found pin: {}", pin.num);
            // TODO: get the value from JSBSim and return it
            return 0;
        }
    }
    panic!("gpio pin {} not initialized", pin_number);
}

pub fn set_value(&mut self, pin_number: u64, value: u8) {
    for pin in &self.pins {
        if pin.num == pin_number {
            println!("set_value found pin: {}", pin.num);
            if pin.dir == "in" { panic!("pin {} is an input", pin.num); }
            // TODO: set the value in JSBSim

            return;
        }
    }
    panic!("gpio pin {} not initialized", pin_number);
}

}

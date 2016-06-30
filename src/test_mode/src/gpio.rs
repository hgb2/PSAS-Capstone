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


pub fn get_value(&mut self, pin_number: u64) -> Result<u8, String> {

    for pin in &self.pins {
        if pin.num == pin_number {
            println!("get_value found pin: {}", pin.num);
            // In the current design, this is only called for the ESTOP
            // pin. Return 0 to keep running. Return 1 to allow
            // the program to exit gracefully.
            return Ok(0);
        }
    }
    Err(format!("attempt to read uninitialized gpio pin {}", pin_number))
}

pub fn set_value(&mut self, pin_number: u64, value: u8) -> Result<(), String> {
    for pin in &self.pins {
        if pin.num == pin_number {
            println!("set_value found pin: {}", pin.num);
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

// for flight mode, cross compile for ARM:
// cargo build --target=arm-unknown-linux-gnueabihf
//
// run as sudo on a raspberry pi

use libs::gpio::MyPins;
use SharedMemory;

const PI_OUT: u64 = 2;
const PI_IN: u64 = 3;

pub struct Control {
    pins: MyPins,
}

impl Control {
pub fn init() -> Control {

    let mut ctl = Control {
        pins: MyPins::new(),
    };

    ctl.pins.add_pin(PI_OUT, "low");
    ctl.pins.add_pin(PI_IN, "in");
    ctl
}

pub fn update(&mut self, mem: &mut SharedMemory) -> Result<u8, String> {
    println!("control update");

    // should be 0 at first because we initialized with "low"
    let mut pi_pin = match self.pins.get_value(PI_OUT) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };
    println!("pi_pin: {}", pi_pin);

    if let Err(msg) = self.pins.set_value(PI_OUT, 1) { return Err(msg); }
    pi_pin = match self.pins.get_value(PI_OUT) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };
    println!("pi_pin: {}", pi_pin);

    if let Err(msg) = self.pins.set_value(PI_OUT, 0) { return Err(msg); }
    pi_pin = match self.pins.get_value(PI_OUT) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };
    println!("pi_pin: {}", pi_pin);

    if let Err(msg) = self.pins.set_value(PI_OUT, 1) { return Err(msg); }
    pi_pin = match self.pins.get_value(PI_OUT) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };
    println!("pi_pin: {}", pi_pin);

    // test write to input pin
    //if let Err(msg) = self.pins.set_value(PI_IN, 1) { return Err(msg); }

    // test write to uninitialized pin
    //if let Err(msg) = self.pins.set_value(5, 1) { return Err(msg); }

    // test read from uninitialized pin
    /*
    pi_pin = match self.pins.get_value(5) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };
    */

    // Using ^C to exit the loop leaves gpio pins exported, but
    // raspberry pi doesn't seem to care.
    // uncomment the following line to test continuous looping
    // pi_pin = 0;
    Ok(pi_pin)
}
}


use libs::gpio::MyPins;
use SharedMemory;


// Use pin 53 as clockwise (CW)
// Use pin 54 as counter clockwise (CCW)
// Use pin 0 as emergency stop (ESTOP)
const CW: u64 = 53;
const CCW: u64 = 54;
const ESTOP: u64 = 0;

const RASPBERRY_PI: u64 = 2;

pub struct Control {
    pins: MyPins,
}

impl Control {
pub fn init() -> Control {
    let mut ctl = Control {
        pins: MyPins::new(),
    };

//    ctl.pins.add_pin(CW, "low");
//    ctl.pins.add_pin(CCW, "low");
//    ctl.pins.add_pin(ESTOP, "in");

    ctl.pins.add_pin(RASPBERRY_PI, "low");
    ctl
}

pub fn update(&mut self, mem: &mut SharedMemory) -> Result<u8, &str> {
    println!("control update");

    // should be 0 at first because we initialized with "low"
    let mut pi_pin = self.pins.get_value(RASPBERRY_PI);
    println!("pi_pin: {}", pi_pin);

    self.pins.set_value(RASPBERRY_PI, 1);
    pi_pin = self.pins.get_value(RASPBERRY_PI);
    println!("pi_pin: {}", pi_pin);

    self.pins.set_value(RASPBERRY_PI, 0);
    pi_pin = self.pins.get_value(RASPBERRY_PI);
    println!("pi_pin: {}", pi_pin);

    self.pins.set_value(RASPBERRY_PI, 1);
    pi_pin = self.pins.get_value(RASPBERRY_PI);
    println!("pi_pin: {}", pi_pin);

    // uncomment the following line to test error returns
    //return Err("something bad happened!");

    // Using ^C to exit the loop leaves gpio pins exported, but
    // raspberry pi doesn't seem to care.
    // uncomment the following line to test continuous looping
    // pi_pin = 0;
    Ok(pi_pin)
}
}


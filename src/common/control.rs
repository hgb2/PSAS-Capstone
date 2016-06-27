use libs::gpio::MyPins;
use SharedMemory;
use UpdateResult;


// Use pin 53 as clockwise (CW)
// Use pin 54 as counter clockwise (CCW)
// Use pin 0 as emergency stop (ESTOP)
const CW: u64 = 53;
const CCW: u64 = 54;
const ESTOP: u64 = 0;

pub struct Control {
    pins: MyPins,
}

impl Control {
pub fn init() -> Control {
    let mut ctl = Control {
        pins: MyPins::new(),
    };

    ctl.pins.add_pin(CW, "low");
    ctl.pins.add_pin(CCW, "low");
    ctl.pins.add_pin(ESTOP, "in");

    ctl
}

pub fn update(&mut self, mem: &mut SharedMemory) -> UpdateResult {
    println!("control update");

    let stop_pin = self.pins.get_value(ESTOP);
    self.pins.set_value(CW, 1);

    mem.cw_state = 1;
    mem.ccw_state = 2;
    Ok(0)
}
}


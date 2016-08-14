///////////////////////////////////////////////////////////////////////////////
// File Name: control.rs
//
// Purpose: The control module applies the control algorithm based on sensor
//          inputs and asserts GPIO pins to achieve course correction.
//
///////////////////////////////////////////////////////////////////////////////

use libs::gpio::{MyPins, Direction};
use SharedMemory;

const CW: u64 = 43; // clockwise; Pin 11 on cold gas jet?
const CCW: u64 = 42; // counter clockwise; Pin 12 on col gas jet?
const ESTOP: u64 = 0;


// shut down value for the ESTOP pin
pub const SHUT_DOWN: u8 = 1;


pub struct Control {
    state: bool,
    pins: MyPins,
}


impl Control {
    pub fn init() -> Control {
        // create a Control instance
        let mut ctl = Control {
            state: false,
            pins: MyPins::new(),
        };

        // add the GPIO pins to it
        ctl.pins.add_pin(CW, Direction::Low);
        ctl.pins.add_pin(CCW, Direction::Low);
        ctl.pins.add_pin(ESTOP, Direction::In);

        // return to caller
        ctl
    }


    ///////////////////////////////////////////////////////////////////////////////
    // Function Name: update
    //
    // Purpose: Reads sensor data from shared memory, executes the control
    //          algorithm, and updates GPIO pin states in shared memory.
    //
    // INPUTS: mem -- reference to shared memory
    //
    // RETURNS: Ok(0)    -- all is well; continue running
    //          Ok(1)    -- shut down
    //          Err(err) -- an error occurred; shut down
    //
    ///////////////////////////////////////////////////////////////////////////////
    pub fn update(&mut self, mem: &mut SharedMemory) -> Result<u8, String> {
        let stop_pin = try!(self.pins.get_value(ESTOP));

        if false && stop_pin == SHUT_DOWN { // TODO: revisit why false && is here
            return Ok(stop_pin);
        }

        let rate_x = mem.gyro_y - 0.41; // Offsetting the led edison board
    	println!("gyro rate {}", rate_x);

        const ACTIVATION_THRESHOLD: f32 = 0.175;

        if rate_x >= ACTIVATION_THRESHOLD {
            // Spin is too fast counter-clockwise, so turn clockwise
            self.state_update(rate_x);
            let value = self.state as u8;
            try!(self.write_pin(CW, value, mem));
        }
        else if rate_x <= -ACTIVATION_THRESHOLD {
            // Spin is too fast clockwise, so turn counter-clockwise
            self.state_update(rate_x);
            let value = self.state as u8;
            try!(self.write_pin(CCW, value, mem));
        }
        else {
            // no spin case
            // turn off both gpio pins and update their state
            try!(self.write_pin(CW, 0, mem));
            try!(self.write_pin(CCW, 0, mem));
        }

        return Ok(0);

    }


    ///////////////////////////////////////////////////////////////////////////////
    // Function Name: state_update
    //
    // Purpose: Updates the control state.
    //
    // INPUTS: rate_x -- rotational rate about the x axis
    //
    // OUTPUTS: Writes new value to the 'state' variable in the Control structure.
    //
    ///////////////////////////////////////////////////////////////////////////////
    fn state_update(&mut self, mut rate_x: f32) {
        // Wish the variables names were more descriptive here, but that's what
        // they are called in Gain_v3.py ... don't want to make any wrong
        // assumptions that make it worse. see:
        // https://github.com/psas/reaction-control/blob/master/Controller_Scripts/Gain_v3.py
        //
        // kp <- 0.25 // proportional gain for duty cycle
        // a <- 2.0 * kp  // (I/(r*.1s))/Ftot equation to dc from radian error
        // u <- a*abs(rate_x)

        let kp = 0.25;
        let a = 2.0 * kp;
        rate_x = rate_x.abs();

        let u = a * rate_x;

        if u >= 1.0 {
            self.state = true;
        }
        else if u < 0.1 {
            self.state = false;
        }
        else {
            // Toggle the state value as the default case
            self.state = !self.state;
        }
    }


    ///////////////////////////////////////////////////////////////////////////////
    // Function Name: write_pin
    //
    // Purpose: Writes a new value to a GPIO pin and updates its state in
    //          shared memory.
    //
    // INPUTS: pin   -- the GPIO pin number (must be an output pin)
    //         value -- the value to write to the pin (0 or 1)
    //         mem   -- reference to shared memory
    //
    // RETURNS: Ok()     -- all is well
    //          Err(err) -- an error occurred
    //
    ///////////////////////////////////////////////////////////////////////////////
    fn write_pin(&mut self, pin: u64, value: u8, mem: &mut SharedMemory) -> Result<(), String> {
        try!(self.pins.set_value(pin, value));

        // STORE value to the pin's state in shared memory
        match pin {
            CW => mem.cw_state = value,
            CCW => mem.ccw_state = value,
            _ => return Err(format!("Invalid pin number: {}", pin)),
        }

        Ok(())
    }
} // impl Control

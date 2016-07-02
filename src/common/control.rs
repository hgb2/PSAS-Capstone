use libs::gpio::MyPins;
use SharedMemory;

// Use pin 53 as clockwise (CW)
// Use pin 54 as counter clockwise (CCW)
// Use pin 0 as emergency stop (ESTOP)
const CW: u64 = 53;
const CCW: u64 = 54;
const ESTOP: u64 = 0;


pub struct Control {
    state: u8,
    pins: MyPins,
}


impl Control {
pub fn init() -> Control {

    let mut ctl = Control {
        state: 0,
        pins: MyPins::new(),
    };

    ctl.pins.add_pin(CW, "low");
    ctl.pins.add_pin(CCW, "low");
    ctl.pins.add_pin(ESTOP, "in");
    ctl
}


pub fn update(&mut self, mem: &mut SharedMemory) -> Result<u8, String> {
    // stop_pin <- CALL ESTOP_pin.get_value()
    let stop_pin = match self.pins.get_value(ESTOP) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };
    // IF stop_pin is 1
    if stop_pin == 1 {
        // RETURN 1
        return Ok(stop_pin);
    } // END IF

    // READ GyX from shared memory
    // (/114.3 when sensitivity is set to 250 dps)
    let rate_x = mem.gyro_x;

    const ACTIVATION_THRESHOLD: f32 = 0.175;
    // IF rateX GE 0.175
    if rate_x >= ACTIVATION_THRESHOLD {
        // CALL state_update()
        // CALL write_pin(CW_pin, state)
        self.state_update(rate_x);
        let value = self.state;
        if let Err(err) = self.write_pin(CW, value, mem) { return Err(err); }
    } // ELSE IF rateX LE -0.175
    else if rate_x <= -ACTIVATION_THRESHOLD {
        // CALL state_update()
        // CALL write_pin(CCW_pin, state)
        self.state_update(rate_x);
        let value = self.state;
        if let Err(err) = self.write_pin(CCW, value, mem) { return Err(err); }
    } // ELSE
    else {
        // turn off both gpio pins
        // CALL write_pin(CW_pin, 0)
        // CALL write_pin(CCW_pin, 0)
        if let Err(err) = self.write_pin(CW, 0, mem) { return Err(err); }
        if let Err(err) = self.write_pin(CCW, 0, mem) { return Err(err); }
    } // END IF

    // RETURN 0
    return Ok(stop_pin);

}

// FUNCTION state_update
// INPUTS: pin -- MyPin object - CW or CCW pins
//         rate_x -- rotational rate about the x axis
// OUTPUTS: Returns void
fn state_update(&mut self, mut rate_x: f32) {
    // wish the variables names were more descriptive here, but that's what they
    // are called in Gain_v3.py ... don't want to make any wrong assumptions that
    // make it worse
    //
    // kp <- 0.25 // proportional gain for duty cycle
    // a <- 2.0 * kp  // (I/(r*.1s))/Ftot equation to dc from radian error
    // u <- a*abs(rate_x)
    let kp = 0.25;
    let a = 2.0 * kp;
    if rate_x < 0.0 {
        rate_x = -rate_x;
    }
    let u = a * rate_x;

    // IF u GE 1.0
    if u >= 1.0 {
        // state <- 1
        self.state = 1;
    } // ELSE IF u LT 0.1
    else if u < 0.1 {
        // state <- 0
        self.state = 0;
    } // ELSE
    else {
        // Toggle the state value
        if self.state == 0 {
            self.state = 1;
        } else {
            self.state = 0;
        }
    } // END IF
/*
    match pin {
        CW => self.cw_pin.state = self.state,
        CCW => self.ccw_pin.state = self.state,
        _ => println!("Invalid pin number: {}", pin) ,
    }
    //Ok(0)
*/

}


// FUNCTION write_pin
// INPUTS:  pin -- MyPin object - CW or CCW pins
//          mem -- reference to shared memory
// OUTPUTS: write pin state to hardware and stores state in shared memory
fn write_pin(&mut self, pin: u64, value: u8, mem: &mut SharedMemory) -> Result<(), String> {
    // CALL pin.set_value(value)
    if let Err(err) = self.pins.set_value(pin, value) {
        return Err(err);
    }

    // STORE value to the pin's state in shared memory
    match pin {
        CW => mem.cw_state = value,
        CCW => mem.ccw_state = value,
        _ => return Err(format!("Invalid pin number: {}", pin)),
    }
    
    Ok(())
}

}


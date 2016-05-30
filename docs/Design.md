# Reaction Control System (RCS) Software Design

## Overview

The Reaction Control System (RCS) software can be compiled for two distinct
target modes -- flight mode and test mode. Common components to both flight and
test modes include Main, Control, Sensor, and Data Formatter modules.

In flight mode, sensor data is retrieved from the hardware over an I2C bus
using Rust libraries. The control algorithm uses sensor data to
compute any required changes in trajectory and again uses Rust
libraries to assert signals on the hardware GPIO pins to control the rocket
nozzles.

In test mode, the Rust libraries are replaced by a controller/sensor
interface that matches the library interface used in flight mode.
The hardware is replaced by JSBSim to model sensor responses to control inputs.

## Software Components

### _Common Components_
#### Main
The main module is executed at program startup and does the following:

```
PRINT startup information including whether test mode is enabled

INIT struct containing to-be-determined information which will be shared between modules
CALL InitializeDataFormatterModule with address of shared memory structure
CALL InitializeSensorModule with address of shared memory structure
CALL InitializeControlModule with address of shared memory structure
INIT variables/class/struct containing means of using high-precision time constructs
     to perform a fixed timestep loop
INIT UDP_Socket connection

WHILE Running EQUAL true
    SET TimeConstruct.CurrentTime TO CALL RustLibraryGetCurrentTime
    INCREMENT TimeConstruct.TimeSinceLastUpdate BY TimeConstruct.CurrentTime
                                                - TimeConstruct.PreviousTime
    SET TimeConstruct.PreviousTime TO TimeConstruct.CurrentTime;
    WHILE TimeConstruct.TimeSinceLastUpdate >= constant_time_step
        CALL SensorModuleUpdate
        IF CALL ControlModuleUpdate EQUAL 1 THEN
            THROW emergency_control_exception
        ENDIF
        CALL Data_Formatter::send_packet with UDP_Socket
        INCREMENT TimeConstruct.TimeSinceLastUpdate BY NEGATE constant_time_step;
        WHEN any exception
            PRINT information about exception
            SET SimulationRunning TO false
    ENDWHILE
ENDWHILE

PRINT information about the RCS run
READ wait for user input to terminate program
```

#### Control Module
The control module implements the control algorithm. Sensor data is retrieved from shared memory and GPIO pins are asserted for course correction.

```
FUNCTION init(addr)
    INPUTS: address of shared memory
    OUTPUTS: Returns void

    STORE address of shared memory

    STORE state <- 0 // control state variable

    // initialize gpio pins
    // Use pin 53 as clockwise (CW)
    // Use pin 54 as counter clockwise (CCW)
    // Use pin 0 as emergency stop (ESTOP)
    STORE CW_pin <- CALL libs::gpio::init(CW pin number)
    CALL CW_pin.set_direction(DIR_OUT)

    STORE CCW_pin <- CALL libs::gpio::init(CCW pin number)
    CALL CCW_pin.set_direction(DIR_OUT)

    STORE ESTOP_pin <- CALL libs::gpio::init(ESTOP pin number)
    CALL ESTOP_pin.set_direction(DIR_IN)

    // turn gpio pins off
    CALL write_pin(CW_pin, state)
    CALL write_pin(CCW_pin, state)

END FUNCTION


FUNCTION update()
    INPUTS: None
    OUTPUTS: Returns 0 -- all is well
                     1 -- Shut Down! (HW asserted the emergency stop pin)

    stop_pin <- CALL ESTOP_pin.get_value()
    IF stop_pin is 1
        RETURN 1
    END IF

    READ GyX from shared memory

    rateX <- GyX/114.3 // degrees per second
                       // (/114.3 when sensitivity is set to 250 dps)
    IF rateX GE 0.175
        CALL state_update()
        CALL write_pin(CW_pin, state)
    ELSE IF rateX LE -0.175
        CALL state_update()
        CALL write_pin(CCW_pin, state)
    ELSE
        // turn off both gpio pins
        CALL write_pin(CW_pin, 0)
        CALL write_pin(CCW_pin, 0)
    END IF

    RETURN 0
END FUNCTION


FUNCTION state_update(rateX)
    INPUTS:  rateX -- rotational rate about the x axis
    OUTPUTS: Returns void

    kp <- 0.25           // proportional gain for duty cycle

    // wish the variables names were more descriptive here, but that's what they
    // are called in Gain_v3.py ... don't want to make any wrong assumptions that
    // make it worse
    a <- 2.0 * kp        // (I/(r*.1s))/Ftot equation to dc from radian error
    u <- a*abs(rateX)

    IF u GE 1.0
        state <- 1
    ELSE IF u LT 0.1
        state <- 0
    ELSE
        Toggle the state value
    END IF

END FUNCTION

FUNCTION write_pin(pin, value)
    INPUTS:  pin -- gpio pin object
             value -- value to write to the pin (0 or 1)
    OUTPUTS: stores gpio pin's state in shared memory and returns void

    CALL pin.set_value(value)

    STORE value to the pin's state in shared memory
END FUNCTION
```

#### Sensor Module
The sensor module retrieves sensor data and stores it in shared memory.  The sensor module provides:

* An initialization function that receives the location of shared memory and sets up the sensor hardware
* An update function that reads sensor data from hardware and stores it in shared memory

```

FUNCTION InitializeSensorModule(sharedMem: &mut SharedMemory)
    INPUTS: address of shared memory
    OUTPUTS: Returns void

    CALL myi2c <- i2c::init() 

ENDFUNCTION


FUNCTION SensorModuleUpdate(sharedMem: &mut SharedMemory)
    INPUTS: address of shared memory
    OUTPUTS: Returns void

    let mut buf = [0u8; (3 + 1 + 3) * 2]  // 3 accel (Registers 3b-40), 
                                          // 1 temp (Registers 41-42), 3 gyro (Registers 43-48)

    CALL myi2c.write(0x3b) // 0x3b is the beginning address of the block of registers that we want to read
    CALL myi2c.read(&buf) // puts block (buf.length) of registers in buf (accel, temp, and gyro)

    WRITE buf into Shared Memory

ENDFUNCTION


```

#### Data Formatter
The data formatter gets telemetry data from the control module, transforms it to [psas-packet format](http://psas-packet-serializer.readthedocs.org/), and sends a UDP packet to a server.

```
FUNCTION init(addr)
    INPUTS: address of shared memory
    OUTPUTS: Returns void
    
    STORE address of shared memory
    
END FUNCTION


FUNCTION send_packet(Socket)
    INPUTS: Socket binding
    OUTPUTS: Returns void
    
    READ RateX from Shared Memory
    READ Sensor Data from Shared Memory
    READ Selected JSBsim data pieces
    
    SET Message type using PSAS-packet API
    SET Data_Package from Shared Memory
    SEND UDP_Packet containing Message type and Data_Package from shared memory

END FUNCTION
```

### _Flight Mode Components_
#### Embedded Rust Libraries
During flight mode, the system reads sensor input and dispatches control signals via [I2C](https://github.com/rust-embedded/rust-i2cdev) and [GPIO](https://github.com/rust-embedded/rust-sysfs-gpio) Rust libraries.

This is mostly a wrapper around the gpio/i2c libraries, calls we can use in our JSBSim library call as well.

```
// gpio File, accessible with gpio::init()
// Import libraries
extern crate sysfs-gpio;
use sysfs_gpio::{Direction, Pin}

FUNCTION init(pin)
   INPUTS: gpio pin number
   OUTPUTS: Linux interface to GPIOs
  
   // embedded linux libraries found here:
   //https://github.com/rust-embedded/rust-sysfs-gpio
   RETURN Pin::new(pin)
END FUNCTION

//i2c File, accessible with i2c::init()
// Import libraries
extern crate i2cdev;
use i2cdev::*;

FUNCTION init(path, slave_address)
   INPUTS: path -- path to i2c device
           slave_address -- component of interest (gyro or accelerometer)
   OUTPUTS: Returns Linux interface to I2C bus
   
   // embedded linux libraries found here:
   // https://github.com/rust-embedded/rust-i2cdev.git
   Set up the i2c_device hardware // Refer to PSAS code for this
   RETURN LinuxI2CDevice::new(path, slave_address)
END FUNCTION
```


### _Test Mode Components_
#### Controller Interface
The controller interface provides a set of functions that are equivalent to
the function calls made by the control module in flight mode. Hardware
compatible compatible data received from the control module is converted to
a compatible format and sent on to JSBSim.

```
extern crate pin-proxy;
use pin_proxy::{Direction, Pin}

/*
struct gpio
  pin = Pin,
}
*/

impl gpio
  FUNCTION init(Pin: u64) -> Option<gpio>
    INITIALIZE pin
    RETURN struct if did not fail
  END FUNCTION

  FUNCTION set_direction(dir: Direction)
	Log event
  END FUNCTION
  
  FUNCTION set_value(value: u8) -> Option<()>
	SET pin.value TO value of 1 or 0
	buffer_to_jsbsim(pin)
    RETURN okay if try did not fail
  END FUNCTION

  FUNCTION get_value() -> Option<u8>
    IF pin is ESTOP RETURN 0
    RETURN pin.value
  END FUNCTION

}

```

#### Sensor Interface
The sensor interface provides a set of functions that are equivalent to the
function calls made by the sensor module in flight mode. JSBSim sensor data
is retrieved, converted into a hardware compatible format, and made available
to the sensor module.

```
extern crate sensor-proxy;
use sensor_proxy::{I2CDevice}

struct i2c
  myi2c = I2CDevice,
}

impl i2c
  FUNCTION init() -> Option<i2c>
    INITIALIZE the proxy I2CDevice
	INITIALIZE JSBsim
    RETURN okay if try did not fail
  END FUNCTION

  FUNCTION read(address: u8) -> Option<u16>
    accel, gyro <- buffer_from_jsbsim()
    data <- Convert to MPU-6050 format {accel, gyro}
	buffer_to_jsbsim(data)
  END FUNCTION
  
  FUNCTION write(address: u8) -> Option<u16>
	Log event
    RETURN okay if try did not fail
  END FUNCTION
}

```

#### JSBSim
JSBSim is commercial off the shelf (COTS) software that is used to
simulate sensor outputs based on control inputs.

```
INPUT:  sim_actuator_output     
OUTPUT: sim_sensor_output


///this function initializes the JSBSim Binder
FUNCTION INITIALIZE
     //set up data buffers
     SET buffer_to_jsbsim                   //data in csv format    
     SET buffer_from_jsbsim                   //data in csv format

     //set up jsbsim
     INIT jsbsim exec
     INSTANTIATE fgfdmexec
     INSTANTIATE script object
     LOAD script into script object
     RUN startup loop (empty)
     PAUSE until ready to launch
     SET rocket launch
ENDFUNCTION


///this is the primary work loop
FUNCTION LOOPDATA (sim_actuator_output):
     IF (testing)
          GET actuator response from sim_actuator_output
          PARSE actuator response into buffer_to_jsbsim //collapse structured data into csv
          SEND buffer_to_jsbsim to jsbsim

          IF (script)
        RUN script object’s runscript()           //will need to know how data is to be blended
        RUN fgfdmexec’s run method                // … between script & sim actuator output
          ENDIF

          PUT data from jsbsim into buffer_from_jsbsim      //structure csv data
          PARSE buffer_from_jsbsim
          SET data into sim_sensor_input
     ENDIF
ENDFUNCTION

///this function closes out the JSBSim Binder
FUNCTION TERMINATE:
     CLOSE buffer_to_jsbsim       
     CLOSE jsbsim output
     CLOSE jsbsim
     CLOSE buffer_from_jsbsim
ENDFUNCTION
```

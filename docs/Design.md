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
CALL InitializeSensorModule with shared memory structure
CALL InitializeControlModule with shared memory structure
INIT variables/class/struct containing means of using high-precision time constructs
     to perform a fixed timestep loop
WHILE SimulationRunning EQUAL true
    SET TimeConstruct.CurrentTime TO CALL RustLibraryGetCurrentTime
    INCREMENT TimeConstruct.TimeSinceLastUpdate BY TimeConstruct.CurrentTime 
                                                - TimeConstruct.PreviousTime
    SET TimeConstruct.PreviousTime TO TimeConstruct.CurrentTime;
    WHILE TimeConstruct.TimeSinceLastUpdate >= constant_time_step
        CALL SensorModuleUpdate with data
        CALL ControlModuleUpdate with data
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
The control module implements the control algorithm. Sensor data is retrieved from shared memory and GPIO pins are asserted for course correction. The control module provides :

* An initialization function that receives the location of shared memory and sets up the control hardware
* An update function that uses sensor data contained in shared memory
to calculate course updates


#### Sensor Module
The sensor module retrieves sensor data and stores it in shared memory.  The sensor module provides:

* An initialization function that receives the location of shared memory and sets up the sensor hardware
* An update function that reads sensor data from hardware and stores it in shared memory

#### Data Formatter
The data formatter gets telemetry data from the control module, transforms it to [psas-packet format](http://psas-packet-serializer.readthedocs.org/), and writes it out to a file.


### _Flight Mode Components_
#### Std Rust Libraries
During flight mode, the system reads sensor input and dispatches control signals via [I2C](https://github.com/rust-embedded/rust-i2cdev) and [GPIO](https://github.com/rust-embedded/rust-sysfs-gpio) Rust libraries.

```
// Import libraries
extern crate sysfs-gpio;
extern crate i2cdev;
use i2cdev::*;
use sysfs_gpio::{Direction, Pin}

// to use, must initiate a new gpio object, call init, then set_value
struct gpio {
  myGpio = Pin,
}

impl gpio {
  public function init(pin: u64, dir: Direction) -> Option<()> {
      // this pseudocode contains possible rust code to run
      initialize myGpio with a new Pin code(myGpio = Pin::new(pin))
      explicitely set the myGpio pin direction code(try!(input.set_direction(dir)))
      return okay if try did not fail code(Ok(()))
  }

  public function set_value(value: u8) -> Option<()> {
    set the value of myGpio with value code(try!(myGpio.set_value(value)))
    return okay if try did not fail code(Ok(()))
  }

}

struct i2c {
  myi2c = I2CDevice,
}

impl i2c {
  public function init(bus: u8) -> Option<()> {
    initialize the I2CDevice code(myi2c = try!(I2CDevice::new(bus)))
    return okay if try did not fail code(Ok(()))
  }

  public function read_value(address: u8) -> Option<u16> {
    let x = read register value from myi2c at address code(myi2c.smbus_read_word_data(address))
    let r = x converted to u16 code(LittleEndian::read_u16(&x))
    return okay if try did not fail code(Ok(r))
  }
}

```


### _Test Mode Components_
#### Controller Interface
The controller interface provides a set of functions that are equivalent to
the function calls made by the control module in flight mode. Hardware
compatible compatible data received from the control module is converted to
a compatible format and sent on to JSBSim.

#### Sensor Interface
The sensor interface provides a set of functions that are equivalent to the
function calls made by the sensor module in flight mode. JSBSim sensor data
is retrieved, converted into a hardware compatible format, and made available
to the sensor module.

#### JSBSim
JSBSim is commercial off the shelf (COTS) software that is used to
simulate sensor outputs based on control inputs.

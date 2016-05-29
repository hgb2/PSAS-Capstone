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

    stop_pin <- CALL ESTOP_pin.read_value()
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

struct SensorModule

  gyro: i2c,
  accel: i2c,

  //accelerometer register addresses (ADXL345B)
  accel_ADXL345B: u8, //slave address
  OFSX: u8, //Axis offsets
  OFSY: u8,
  OFSZ: u8,
  BW_RATE: u8, //data rate and power mode control (need to find out i2c rate)
  POWER_CTL: u8, //power saving features, default is fine


  //Accelerometer data is in two's compliment
  //"0" is the least significant byte
  //"1" is the most significant byte
  AX0: u8,
  AX1: u8,
  AY0: u8,
  AY1: u8,
  AZ0: u8,
  AZ1: u8,


  //gyro register addresses (L3G4200D) max i2c rate 400kHz
  gyro_L3G4200D: u8,
  //Gyro data is two's complement with same format as accelerometer
  GX0: u8,
  GX1: u8,
  GY0: u8,
  GY1: u8,
  GZ0: u8,
  GZ1: u8,

}


impl SensorModule

  Function InitializeSensorModule(sharedMem: &mut SharedMemory)
      INPUTS: address of shared memory
      OUTPUTS: Returns void

    //accelerometer register addresses (ADXL345B)
    accel_ADXL345B = 0x53 //slave address
    OFSX = 0x1E //Axis offsets
    OFSY = 0x1F
    OFSZ = 0x20
    BW_RATE = 0x2C //data rate and power mode control (need to find out i2c rate)
    POWER_CTL = 0x2D //power saving features, default is fine

    //Accelerometer data is in two's compliment
    //"0" is the least significant byte
    //"1" is the most significant byte
    AX0 = 0x32
    AX1 = 0x33
    AY0 = 0x34
    AY1 = 0x35
    AZ0 = 0x36
    AZ1 = 0x37

    //gyro register addresses (L3G4200D) max i2c rate 400kHz
    gyro_L3G4200D = 0x35
    //Gyro data is two's complement with same format as accelerometer
    GX0 = 0x28
    GX1 = 0x29
    GY0 = 0x2A
    GY1 = 0x2B
    GZ0 = 0x2C
    GZ1 = 0x2D


    INITIALIZE the i2c device
    gyro.init(gyro_L3G4200D)
    accel.init(accel_ADXL345B)

  ENDFUNCTION



  FUNCTION SensorModuleUpdate(sharedMem: &mut SharedMemory)
      INPUTS: address of shared memory
      OUTPUTS: Returns void


    SET i2c slave address for accelerometer to accel_ADXL345B

    READ from i2c at address AX0
    WRITE to SharedMemory in AcX

    READ from i2c at address AY0
    WRITE to SharedMemory in AcY

    READ from i2c at address AZ0
    WRITE to SharedMemory in AcZ


    SET i2c slave address for gyro to gyro_L3G4200D

    READ from i2c at address GX0
    WRITE to SharedMemory in GcX

    READ from i2c at address GY0
    WRITE to SharedMemory in GcY

    READ from i2c at address GZ0
    WRITE to SharedMemory in GcZ


  ENDFUNCTION

}
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

    RETURN 0
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

struct gpio
  cw = Pin,
  ccw = Pin,
  estop = Pin,
}

impl gpio
  FUNCTION init(pin: u64, dir: Direction) -> Option<gpio>
	INITIALIZE jsbsim
    INITIALIZE cw, ccw, estop with a new Pin for each
    SET the gpo pin directions
    RETURN okay if try did not fail
  END FUNCTION

  FUNCTION set_direction()
	Log event
  END FUNCTION
  
  FUNCTION set_value(value: u8) -> Option<()>
	cw.get_value()
	ccw.get_value()
	buffer_to_jsbsim({value, cw, ccw})
    RETURN okay if try did not fail
  END FUNCTION

  FUNCTION get_value() -> Option<u8>
    RETURN the value of the pin, wrapper around library calls
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
  FUNCTION init(bus: u8) -> Option<i2c>
    INITIALIZE the proxy I2CDevice
    RETURN okay if try did not fail
  END FUNCTION

  FUNCTION read_value(address: u8) -> Option<u16>
    accel, gyro <- buffer_from_jsbsim()
    data <- Convert to MPU-6050 format {accel, gyro}
	buffer_to_jsbsim(data)
  END FUNCTION
  
  FUNCTION write_value(address: u8) -> Option<u16>
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

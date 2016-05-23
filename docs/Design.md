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
CALL InitializeSensorModule with address of shared memory structure
CALL InitializeControlModule with address of shared memory structure
INIT variables/class/struct containing means of using high-precision time constructs
     to perform a fixed timestep loop
     
WHILE Running EQUAL true
    SET TimeConstruct.CurrentTime TO CALL RustLibraryGetCurrentTime
    INCREMENT TimeConstruct.TimeSinceLastUpdate BY TimeConstruct.CurrentTime 
                                                - TimeConstruct.PreviousTime
    SET TimeConstruct.PreviousTime TO TimeConstruct.CurrentTime;
    WHILE TimeConstruct.TimeSinceLastUpdate >= constant_time_step
        IF CALL SensorModuleUpdate EQUAL 1 THEN
            THROW emergency_sensor_exception
        ENDIF
        IF CALL ControlModuleUpdate EQUAL 1 THEN
            THROW emergency_control_exception
        ENDIF
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

    Store address of shared memory

    state <- 0 // control state variable

    // initialize gpio pins
    // Use pin 53 as clockwise (CW)
    // Use pin 54 as counter clockwise (CCW)
    // Use pin 0 as emergency stop (ESTOP)
    CALL libs::gpio::init(CW pin, DIR_OUT)
    CALL libs::gpio::init(CCW pin, DIR_OUT)
    CALL libs::gpio::init(ESTOP pin, DIR_IN)

    // turn gpio pins off
    CALL libs::gpio::set_value(CW pin, state)
    CALL libs::gpio::set_value(CCW pin, state)

    // initialize data formatter
    CALL data_fmt::init()

END FUNCTION


FUNCTION update()
    INPUTS: None
    OUTPUTS: Returns 0 -- all is well
                     1 -- Shut Down! (HW asserted the emergency stop pin)

    CALL libs::gpio::read_value(ESTOP pin)
    IF ESTOP pin is 1
        RETURN 1
    END IF

    Retrieve GyX from shared memory

    rateX <- GyX/114.3 // degrees per second
                       // (/114.3 when sensitivity is set to 250 dps)
    IF rateX GE 0.175
        CALL state_update()
        CALL libs::gpio::set_value(CW pin, state)
    ELSE IF rateX LE -0.175
        CALL state_update()
        CALL libs::gpio::set_value(CCW pin, state)
    ELSE
        // turn off both gpio pins
        CALL libs::gpio::set_value(CW pin, 0)
        CALL libs::gpio::set_value(CCW pin, 0)
    END IF

    // send info to data formatter
    CALL data_fmt::update(TBD)

    RETURN 0
END FUNCTION


FUNCTION state_update(rateX)
    INPUTS:  rateX -- rotational rate about the x axis
    OUTPUTS: Returns void

    kp <- 0.25           // proportional gain for duty cycle
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
```

#### Sensor Module
The sensor module retrieves sensor data and stores it in shared memory.  The sensor module provides:

* An initialization function that receives the location of shared memory and sets up the sensor hardware
* An update function that reads sensor data from hardware and stores it in shared memory

```

struct SensorModule {

  gyro: i2c,
  accel: i2c,
  magneto: i2c,

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


  //Magnetometer addresses
  mag_HMC5883L: u8, 
  //mag data is the same as the rest
  MX0: u8,
  MX1: u8,
  MY0: u8,
  MY1: u8,
  MZ0: u8,
  MZ1: u8,

  //magnetometer mode selection register
  Mag_Mode_Reg: u8,

  //barometer address
  bar_BMP085: u8,
}


impl SensorModule {

  Function InitializeSensorModule(sharedMem: &mut SharedMemory) {
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

    //Magnetometer addresses
    mag_HMC5883L = 0x1E
    //mag data is the same as the rest
    MX0 = 0x03
    MX1 = 0x04
    MY0 = 0x07
    MY1 = 0x08
    MZ0 = 0x05
    MZ1 = 0x06

    Mag_Mode_Reg = 0x02 //magnetometer mode selection register

    //Not quite sure what to do with these
    //x.address(mag_HMC5883L)
    //x.writeReg(Mag_Mode_Reg, 0x00) //0x00 == continuous measurements, default is 0x01 == single measurement

    //barometer address
    bar_BMP085 = 0xEE


    INITIALIZE the i2c device 
    gyro.init(accel_ADXL345B)
    accel.init(gyro_L3G4200D)
    magneto.init(mag_HMC5883L)


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


    SET i2c slave address for magnetometer to mag_HMC5883L

    READ from i2c at address MX0 
    WRITE to SharedMemory in McX

    READ from i2c at address MY0 
    WRITE to SharedMemory in McY

    READ from i2c at address MZ0 
    WRITE to SharedMemory in McZ


  ENDFUNCTION

}
```




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

```
INPUT:	sim_actuator_output			
OUTPUT:	sim_sensor_output


///this function initializes the JSBSim Binder
FUNCTION INITIALIZE
     //set up data buffers
     SET buffer_to_jsbsim				          	//data in csv format		
     SET buffer_from_jsbsim				          	//data in csv format

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
     WHILE (testing)
          GET actuator response from sim_actuator_output
          PARSE actuator response into buffer_to_jsbsim	//collapse structured data into csv
          SEND buffer_to_jsbsim to jsbsim

          WHILE (script)
	     	RUN script object’s runscript()         	//will need to know how data is to be blended
	     	RUN fgfdmexec’s run method              	// … between script & sim actuator output
          ENDWHILE

          PUT data from jsbsim into buffer_from_jsbsim      //structure csv data
          PARSE buffer_from_jsbsim
          SET data into sim_sensor_input
     ENDWHILE
ENDFUNCTION

///this function closes out the JSBSim Binder
FUNCTION TERMINATE:
     CLOSE buffer_to_jsbsim				
     CLOSE jsbsim output
     CLOSE jsbsim
     CLOSE buffer_from_jsbsim
ENDFUNCTION
```

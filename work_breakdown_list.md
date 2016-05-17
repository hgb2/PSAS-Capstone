## Learn Rust (Before summer) (All)
* write hello world
* learn about Multirust
* learn about Types
* learn about structs
* learn about traits
* learn about impl
* learn about rust testing framework (Cargo)
* learn about rust documentation (Cargo)

## Common Components
### Shared memory
* Determine rust best practices for sharing memory
* Incorporate memory sharing using rust built in types
* Build struct for memory sharing

### Data formatter
* Understand format PSAS wants to see
* Determine incoming telemetry data
* Convert incoming format to logging format
* Create logging function

### Sensor Module
* Plug into flight mode I2C library 
* Plug into Sensor interface
* Allow for redirection of Sensor data from either
    * Test mode (Sensor interface)
    * Flight mode (I2C)

### Controller Module
* Plug into flight mode GPIO library
* Plug into controller interface
* Allow for redirection of Controller data to either
    * Test mode (Controller interface)
    * Flight mode (GPIO library)

### Main / Core
* Task Resources
* plug into Controller
* test Controller
* plug into Sensor
* test Sensor
* iterate on control theory

## Test Mode 2 weeks, 3 people
### JSBSim
* Determine how JSBSim gives us data
* Research JSBSim rust library
* Determine how to configure JSBSim for reproducible tests
* Write up clockwise/counter-clockwise tests

### Sensor Interface
* Convert JSB clockwise data to Sensor module data
* Convert JSB counter-clockwise data to Sensor module data

### Controller Interface
* Convert Control module clockwise data to JSBSim data
* Convert Control module counter-clockwise data to JSBSim data

## Flight Mode
### Rust Libraries
1. GPIO for control
    * Research rust GPIO library
    * Determine api call for GPIO controls
    * Wrap calls as high level objects
    * function: clockwise call
    * function: counter-clockwise call
    * Stretch:
        * Stretch: Call up
        * Stretch: Call down
        * Stretch: Call forward
        * Stretch: Call back
        * Stretch: Call left
        * Stretch: Call right 
        * Stretch: Call yaw left
        * Stretch: Call yaw right


1. I2C for sensors
    * Research rust I2C library
    * Determine api calls for recording data
    * Wrap incoming data to our data stream
    * Determine roll speed
    * Determine roll direction (clockwise or counter-clockwise)
    * Stretch:
        * Stretch: Determine up
        * Stretch: Determine down
        * Stretch: Determine forward
        * Stretch: Determine back
        * Stretch: Determine left
        * Stretch: Determine right 
        * Stretch: Determine yaw direction
        * Stretch: Determine yaw speed







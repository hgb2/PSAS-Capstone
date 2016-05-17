## Learn Rust
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
* Determine struct used for shared (single threaded) memory
    * Mimic old Python code
* implement the traits on the structure for data modification
    * write data
    * read data

### Data formatter
* Understand telemetry format PSAS wants
* Determine incoming telemetry data format
* Convert incoming format to telemetry format
* Create logging function

### Sensor Module
* Plug into flight mode I2C library 
* Plug into Sensor interface
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

### Control Module
* Plug into flight mode GPIO library
* Plug into controller interface
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

### Main
* Initialize Shared Memory
* Initialize Sensor
* Initialize Control
* Loop with:
    * Read from sensor
    * Write to controller
* Iterate with information about control theory from PSAS team

## Test Mode
### JSBSim
* Determine how JSBSim gives us data
* Connect c/c++ JSBSim libraries to Rust
* Determine how to configure JSBSim for reproducible tests
* Write up clockwise/counter-clockwise tests

### Sensor Interface
* Convert JSB clockwise data to Sensor module data
* Convert JSB counter-clockwise data to Sensor module data
* Cross reference this with I2C libraries

### Controller Interface
* Convert Control module clockwise data to JSBSim data
* Convert Control module counter-clockwise data to JSBSim data
* Cross reference this with GPIO libraries

## Learn about flight Mode for Controller and Sensor modules
### Rust library GPIO for control (June 20) (1)
    * Research rust GPIO library
    * Determine api call for GPIO controls

### Rust library I2C for sensors
    * Research rust I2C library
    * Determine api calls for recording data

## Demonstrations
* Must: Compile on linux (debian, ubuntu, etc.)
* Stretch: Compile on windows (10, 7)
* Jamey's test board
* RCS prototype
* Stretch: Cubesat

## Mutitarget builds
* Determine configuration for test mode vs flight mode
* Determine configuration for platform target
* Hardware agnostic to test hardware vs on rocket hardware

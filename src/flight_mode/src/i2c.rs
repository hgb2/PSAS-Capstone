extern crate i2cdev;
//use i2cdev::LinuxI2cDevice;

pub fn init(path: String, slave_address: u16) { // -> i2cdev::linux::LinuxI2CDevice {
    //i2cdev::linux::LinuxI2CDevice::new(path, slave_address)
}

pub fn update(x: i32) -> i32 {
    println!("flight mode i2c update received {}", x);
    x
}

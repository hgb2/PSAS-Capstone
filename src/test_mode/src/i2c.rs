use std::io;

use wrapper;

pub struct Myi2c {

}

impl Drop for Myi2c {
    fn drop(&mut self) {
        wrapper::wrapper_close();
    }
}


impl Myi2c {
    pub fn init() -> Result<Myi2c, io::Error> {
        wrapper::wrapper_init();
        let i2c = Myi2c{};
        return Ok(i2c);
    }


    pub fn get_gyro(&mut self) -> Result<(f32, f32, f32), io::Error> {
        let gyro = wrapper::get_from_jsbsim();
        return Ok(gyro);
    }
}

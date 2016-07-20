use std::io;
use std::fmt;
use wrapper;


pub enum Error {
    Unexpected(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sensor error")
    }
}

pub enum I2CError {
    NotSupported,
    Other(&'static str),
}

pub type I2CResult<T> = Result<T, I2CError>;


pub struct Myi2c {}

impl Myi2c {

    pub fn init() -> Result<Myi2c, io::Error> {
        // Initialize JSBSim FFI binder interface
        let fdm = wrapper::wrapper_init();

        return Ok(Myi2c{});
    }

    pub fn write (&mut self, reg: &[u8]) -> Result<(), io::Error> {
        return Ok(());
    }

    pub fn read(&mut self, mut buf: &mut [u8]) -> Result<(), io::Error> {
        return Ok(());
    }
}

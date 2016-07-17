use std::io;



pub struct Myi2c {

}

impl Myi2c {

    pub fn init() -> Result<Myi2c, io::Error> {
        return Ok(Myi2c{});
    }

    pub fn write (&mut self, reg: &[u8]) -> Result<(), io::Error> {
        return Ok(());
    }

    pub fn read(&mut self, mut buf: &mut [u8]) -> Result<(), io::Error> {
        return Ok(());
    }
}

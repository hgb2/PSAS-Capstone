pub fn init() {
    println!("test mode i2c init");
}

pub fn update(x: i32) -> i32 {
    println!("test mode i2c update received {}", x);
    x
}

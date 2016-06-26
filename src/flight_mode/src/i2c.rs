pub fn init() {
    println!("flight mode i2c init");
}

pub fn update(x: i32) -> i32 {
    println!("flight mode i2c update received {}", x);
    x
}

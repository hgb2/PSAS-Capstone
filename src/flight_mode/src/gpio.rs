use sysfs_gpio::Pin;

pub fn init(pin_num: u64) -> sysfs_gpio::Pin {
    sysfs_gpio::Pin::new(pin_num)
}

pub fn update(x: i32) {
    println!("flight mode gpio update received {}", x);
}


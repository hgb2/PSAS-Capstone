use sysfs_gpio::Pin;

pub fn init(pin_num: u64) -> Pin {
    Pin::new(pin_num)
}

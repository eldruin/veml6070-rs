extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate veml6070;

use linux_embedded_hal::I2cdev;
use veml6070::VEML6070;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut uv_light_sensor = VEML6070::new(dev);
    // initialization step is necessary
    uv_light_sensor.init().unwrap();
    uv_light_sensor.enable().unwrap();
    let reading = uv_light_sensor.read_uv().unwrap();
    println!("UV reading: {}", reading);
}

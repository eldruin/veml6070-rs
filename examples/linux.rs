use linux_embedded_hal::I2cdev;
use veml6070::Veml6070;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut uv_light_sensor = Veml6070::new(dev);
    // initialization step is necessary
    uv_light_sensor.init().unwrap();
    uv_light_sensor.enable().unwrap();
    let reading = uv_light_sensor.read_uv().unwrap();
    println!("UV reading: {}", reading);
}

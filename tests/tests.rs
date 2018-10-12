extern crate veml6070;
extern crate embedded_hal_mock as hal;
use veml6070::{ VEML6070, IntegrationTime, AckThreshold };

struct Address;

impl Address {
    const ARA      : u8 = 0x0C;
    const COMMAND  : u8 = 0x38;
    const DATA_LSB : u8 = 0x38;
}
const DEFAULT_CMD  : u8 = 0x02;


fn setup<'a>(data: &'a[u8]) -> VEML6070<hal::I2cMock<'a>> {
    let mut dev = hal::I2cMock::new();
    dev.set_read_data(&data);
    VEML6070::new(dev)
}

fn check_sent_data(sensor: VEML6070<hal::I2cMock>, address: u8, data: &[u8]) {
    let dev = sensor.destroy();
    assert_eq!(dev.get_last_address(), Some(address));
    assert_eq!(dev.get_write_data(), &data[..]);
}

#[test]
fn can_clear_ack() {
    let mut dev = setup(&[0]);
    dev.clear_ack().unwrap();
    let i2c = dev.destroy();
    assert_eq!(i2c.get_last_address(), Some(Address::ARA));
}

#[test]
fn can_init() {
    let mut dev = setup(&[0]);
    dev.init().unwrap();
    check_sent_data(dev, Address::COMMAND, &[DEFAULT_CMD]);
}

#[test]
fn can_enable() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    check_sent_data(dev, Address::COMMAND, &[DEFAULT_CMD]);
}

#[test]
fn can_disable() {
    let mut dev = setup(&[0]);
    dev.disable().unwrap();
    check_sent_data(dev, Address::COMMAND, &[DEFAULT_CMD | 1]);
}

#[test]
fn can_read_uv() {
    let mut dev = setup(&[0xAB, 0xCD]);
    let reading = dev.read_uv().unwrap();
    assert_eq!(0xABCD, reading);
    let i2c = dev.destroy();
    assert_eq!(i2c.get_last_address(), Some(Address::DATA_LSB));
}

macro_rules! it_test {
    ( $test_name:ident, $it:expr, $expected:expr ) => {
        #[test]
        fn $test_name() {
            let mut dev = setup(&[0]);
            dev.set_integration_time($it).unwrap();
            check_sent_data(dev, Address::COMMAND, &[DEFAULT_CMD | $expected << 2]);
        }
    }
}

it_test!(can_set_integration_time_half_t, IntegrationTime::HalfT, 0);
it_test!(can_set_integration_time_1_t,    IntegrationTime::T1,    1);
it_test!(can_set_integration_time_2_t,    IntegrationTime::T2,    2);
it_test!(can_set_integration_time_4_t,    IntegrationTime::T4,    3);

#[test]
fn can_enable_ack() {
    let mut dev = setup(&[0]);
    dev.enable_ack().unwrap();
    check_sent_data(dev, Address::COMMAND, &[DEFAULT_CMD | 0b0010_0000]);
}

#[test]
fn can_disable_ack() {
    let mut dev = setup(&[0]);
    dev.disable_ack().unwrap();
    check_sent_data(dev, Address::COMMAND, &[DEFAULT_CMD]);
}


#[test]
fn can_set_ack_threshold_102_steps() {
    let mut dev = setup(&[0]);
    dev.set_ack_threshold(AckThreshold::Steps102).unwrap();
    check_sent_data(dev, Address::COMMAND, &[DEFAULT_CMD]);
}

#[test]
fn can_set_ack_threshold_145_steps() {
    let mut dev = setup(&[0]);
    dev.set_ack_threshold(AckThreshold::Steps145).unwrap();
    check_sent_data(dev, Address::COMMAND, &[DEFAULT_CMD | 0b0001_0000]);
}

#[test]
fn can_enable_ack_with_threshold_145_steps() {
    let mut dev = setup(&[0]);
    dev.enable_ack_with_threshold(AckThreshold::Steps145).unwrap();
    check_sent_data(dev, Address::COMMAND, &[DEFAULT_CMD | 0b0011_0000]);
}
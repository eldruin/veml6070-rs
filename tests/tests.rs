use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use veml6070::{AckThreshold, IntegrationTime, Veml6070};

struct Address;

impl Address {
    const ARA: u8 = 0x0C;
    const COMMAND: u8 = 0x38;
    const DATA_MSB: u8 = 0x39;
    const DATA_LSB: u8 = 0x38;
}
const DEFAULT_CMD: u8 = 0x02;

fn new(transactions: &[I2cTrans]) -> Veml6070<I2cMock> {
    Veml6070::new(I2cMock::new(transactions))
}

fn destroy(dev: Veml6070<I2cMock>) {
    dev.destroy().done();
}

#[test]
fn can_clear_ack() {
    let mut dev = new(&[I2cTrans::read(Address::ARA, vec![0])]);
    dev.clear_ack().unwrap();
    destroy(dev);
}

#[test]
fn can_init() {
    let mut dev = new(&[
        I2cTrans::read(Address::ARA, vec![0]),
        I2cTrans::write(Address::COMMAND, vec![DEFAULT_CMD]),
    ]);
    dev.init().unwrap();
    destroy(dev);
}

#[test]
fn can_enable() {
    let mut dev = new(&[I2cTrans::write(Address::COMMAND, vec![DEFAULT_CMD])]);
    dev.enable().unwrap();
    destroy(dev);
}

#[test]
fn can_disable() {
    let mut dev = new(&[I2cTrans::write(Address::COMMAND, vec![DEFAULT_CMD | 1])]);
    dev.disable().unwrap();
    destroy(dev);
}

#[test]
fn can_read_uv() {
    let mut dev = new(&[
        I2cTrans::read(Address::DATA_MSB, vec![0xAB]),
        I2cTrans::read(Address::DATA_LSB, vec![0xCD]),
    ]);
    let reading = dev.read_uv().unwrap();
    assert_eq!(0xABCD, reading);
    destroy(dev);
}

macro_rules! it_test {
    ( $test_name:ident, $it:expr, $expected:expr ) => {
        #[test]
        fn $test_name() {
            let mut dev = new(&[I2cTrans::write(
                Address::COMMAND,
                vec![DEFAULT_CMD | $expected << 2],
            )]);
            dev.set_integration_time($it).unwrap();
            destroy(dev);
        }
    };
}

it_test!(can_set_integration_time_half_t, IntegrationTime::HalfT, 0);
it_test!(can_set_integration_time_1_t, IntegrationTime::T1, 1);
it_test!(can_set_integration_time_2_t, IntegrationTime::T2, 2);
it_test!(can_set_integration_time_4_t, IntegrationTime::T4, 3);

#[test]
fn can_enable_ack() {
    let mut dev = new(&[I2cTrans::write(
        Address::COMMAND,
        vec![DEFAULT_CMD | 0b0010_0000],
    )]);
    dev.enable_ack().unwrap();
    destroy(dev);
}

#[test]
fn can_disable_ack() {
    let mut dev = new(&[I2cTrans::write(Address::COMMAND, vec![DEFAULT_CMD])]);
    dev.disable_ack().unwrap();
    destroy(dev);
}

#[test]
fn can_set_ack_threshold_102_steps() {
    let mut dev = new(&[I2cTrans::write(Address::COMMAND, vec![DEFAULT_CMD])]);
    dev.set_ack_threshold(AckThreshold::Steps102).unwrap();
    destroy(dev);
}

#[test]
fn can_set_ack_threshold_145_steps() {
    let mut dev = new(&[I2cTrans::write(
        Address::COMMAND,
        vec![DEFAULT_CMD | 0b0001_0000],
    )]);
    dev.set_ack_threshold(AckThreshold::Steps145).unwrap();
    destroy(dev);
}

#[test]
fn can_enable_ack_with_threshold_145_steps() {
    let mut dev = new(&[I2cTrans::write(
        Address::COMMAND,
        vec![DEFAULT_CMD | 0b0011_0000],
    )]);
    dev.enable_ack_with_threshold(AckThreshold::Steps145)
        .unwrap();
    destroy(dev);
}

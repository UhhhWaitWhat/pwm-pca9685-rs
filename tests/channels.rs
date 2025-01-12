use embedded_hal_mock::i2c::Transaction as I2cTrans;
use pwm_pca9685::Channel;
use std::convert::TryFrom;

mod common;
use self::common::{assert_invalid_input_data, destroy, new, Register, DEV_ADDR, MODE1_AI};

macro_rules! can_convert_channel {
    ($t:ty, $($value:expr, $channel:ident),*) => {
        $(
            assert_eq!(Channel::$channel, Channel::try_from($value as $t).unwrap());
        )*
    };
}

#[test]
fn can_convert_channel_u8() {
    can_convert_channel!(
        u8, 0, C0, 1, C1, 2, C2, 3, C3, 4, C4, 5, C5, 6, C6, 7, C7, 8, C8, 9, C9, 10, C10, 11, C11,
        12, C12, 13, C13, 14, C14, 15, C15
    );
}

#[test]
fn can_convert_channel_u16() {
    can_convert_channel!(
        u16, 0, C0, 1, C1, 2, C2, 3, C3, 4, C4, 5, C5, 6, C6, 7, C7, 8, C8, 9, C9, 10, C10, 11,
        C11, 12, C12, 13, C13, 14, C14, 15, C15
    );
}

#[test]
fn can_convert_channel_usize() {
    can_convert_channel!(
        usize, 0, C0, 1, C1, 2, C2, 3, C3, 4, C4, 5, C5, 6, C6, 7, C7, 8, C8, 9, C9, 10, C10, 11,
        C11, 12, C12, 13, C13, 14, C14, 15, C15
    );
}

#[test]
fn convert_channel_out_of_bounds() {
    assert_eq!(Err(()), Channel::try_from(16_u8));
    assert_eq!(Err(()), Channel::try_from(16_u16));
    assert_eq!(Err(()), Channel::try_from(16_usize));
}

invalid_test!(
    cannot_set_channel_on_invalid_value,
    set_channel_on,
    Channel::C0,
    4096
);

invalid_test!(
    cannot_set_channel_full_on_invalid_value,
    set_channel_full_on,
    Channel::C0,
    4096
);

invalid_test!(
    cannot_set_channel_off_invalid_value,
    set_channel_off,
    Channel::C0,
    4096
);

invalid_test!(
    cannot_set_channel_on_off_invalid_value_on,
    set_channel_on_off,
    Channel::C0,
    4096,
    0
);

invalid_test!(
    cannot_set_channel_on_off_invalid_value_off,
    set_channel_on_off,
    Channel::C0,
    0,
    4096
);

invalid_test!(
    cannot_set_all_on_off_invalid_value_on,
    set_all_on_off,
    &[4096; 16],
    &[0; 16]
);

invalid_test!(
    cannot_set_all_on_off_invalid_value_off,
    set_all_on_off,
    &[0; 16],
    &[4096; 16]
);

#[test]
fn sets_autoincrement_just_once() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::ALL_C_ON_L, 0b1111_1111, 0b0000_1111],
        ),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::ALL_C_ON_L, 0b1111_1111, 0b0000_1111],
        ),
    ];
    let mut pwm = new(&trans);
    pwm.set_channel_on(Channel::All, 4095).unwrap();
    pwm.set_channel_on(Channel::All, 4095).unwrap();
    destroy(pwm);
}

macro_rules! channels_test {
    ($($channel:ident, $reg_on:ident, $reg_off:ident),*) => {
        $(
            #[allow(non_snake_case)]
            mod $channel {
                use super::*;
                #[test]

                fn can_set_channel_on_min() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 0, 0])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_on(Channel::$channel, 0).unwrap();
                    destroy(pwm);
                }

                #[test]
                fn can_get_channel_on_min() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write_read(DEV_ADDR, vec![Register::$reg_on], vec![0, 0])
                    ];
                    let mut pwm = new(&trans);
                    assert_eq!(pwm.get_channel_on(Channel::$channel).unwrap(), 0);
                    destroy(pwm);
                }

                #[test]

                fn can_set_channel_on_max() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 0b1111_1111, 0b0000_1111])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_on(Channel::$channel, 4095).unwrap();
                    destroy(pwm);
                }

                #[test]
                fn can_get_channel_on_max() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write_read(DEV_ADDR, vec![Register::$reg_on], vec![255, 255])
                    ];
                    let mut pwm = new(&trans);
                    assert_eq!(pwm.get_channel_on(Channel::$channel).unwrap(), 4095);
                    destroy(pwm);
                }

                #[test]
                fn can_set_channel_off_min() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_off, 0, 0])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_off(Channel::$channel, 0).unwrap();
                    destroy(pwm);
                }

                #[test]
                fn can_get_channel_off_min() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write_read(DEV_ADDR, vec![Register::$reg_off], vec![0, 0])
                    ];
                    let mut pwm = new(&trans);
                    assert_eq!(pwm.get_channel_off(Channel::$channel).unwrap(), 0);
                    destroy(pwm);
                }

                #[test]
                fn can_set_channel_off_max() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_off, 0b1111_1111, 0b0000_1111])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_off(Channel::$channel, 4095).unwrap();
                    destroy(pwm);
                }

                #[test]
                fn can_get_channel_off_max() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write_read(DEV_ADDR, vec![Register::$reg_off], vec![255, 255])
                    ];
                    let mut pwm = new(&trans);
                    assert_eq!(pwm.get_channel_off(Channel::$channel).unwrap(), 4095);
                    destroy(pwm);
                }

                #[test]

                fn can_set_channel_full_on_min() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 0, 0b0001_0000])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_full_on(Channel::$channel, 0).unwrap();
                    destroy(pwm);
                }

                #[test]

                fn can_set_channel_full_on_max() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 0b1111_1111, 0b0001_1111])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_full_on(Channel::$channel, 4095).unwrap();
                    destroy(pwm);
                }

                #[test]
                fn can_get_channel_full_on() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write_read(DEV_ADDR, vec![Register::$reg_on], vec![0, 0b0001_0000])
                    ];
                    let mut pwm = new(&trans);
                    assert!(pwm.get_channel_full_on(Channel::$channel).unwrap());
                    destroy(pwm);
                }

                #[test]

                fn can_set_channel_full_off() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_off, 0, 0b0001_0000])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_full_off(Channel::$channel).unwrap();
                    destroy(pwm);
                }

                #[test]

                fn can_set_channel_on_off() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write(DEV_ADDR, vec![Register::$reg_on, 2, 1, 4, 3])
                    ];
                    let mut pwm = new(&trans);
                    pwm.set_channel_on_off(Channel::$channel, 0x102, 0x304).unwrap();
                    destroy(pwm);
                }

                #[test]
                fn can_get_channel_full_off() {
                    let trans = [
                        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
                        I2cTrans::write_read(DEV_ADDR, vec![Register::$reg_off], vec![0, 0b0001_0000])
                    ];
                    let mut pwm = new(&trans);
                    assert!(pwm.get_channel_full_off(Channel::$channel).unwrap());
                    destroy(pwm);
                }
            }
        )*
    };
}

channels_test!(
    C0,
    C0_ON_L,
    C0_OFF_L,
    C1,
    C1_ON_L,
    C1_OFF_L,
    C2,
    C2_ON_L,
    C2_OFF_L,
    C3,
    C3_ON_L,
    C3_OFF_L,
    C4,
    C4_ON_L,
    C4_OFF_L,
    C5,
    C5_ON_L,
    C5_OFF_L,
    C6,
    C6_ON_L,
    C6_OFF_L,
    C7,
    C7_ON_L,
    C7_OFF_L,
    C8,
    C8_ON_L,
    C8_OFF_L,
    C9,
    C9_ON_L,
    C9_OFF_L,
    C10,
    C10_ON_L,
    C10_OFF_L,
    C11,
    C11_ON_L,
    C11_OFF_L,
    C12,
    C12_ON_L,
    C12_OFF_L,
    C13,
    C13_ON_L,
    C13_OFF_L,
    C14,
    C14_ON_L,
    C14_OFF_L,
    C15,
    C15_ON_L,
    C15_OFF_L,
    All,
    ALL_C_ON_L,
    ALL_C_OFF_L
);

#[test]

fn can_set_all_on_off() {
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
        I2cTrans::write(
            DEV_ADDR,
            vec![
                Register::C0_ON_L,
                1,
                1,
                3,
                3,
                2,
                1,
                4,
                3,
                3,
                1,
                5,
                3,
                4,
                1,
                6,
                3,
                5,
                1,
                7,
                3,
                6,
                1,
                8,
                3,
                7,
                1,
                9,
                3,
                8,
                1,
                0,
                4,
                9,
                1,
                1,
                4,
                0,
                2,
                2,
                4,
                1,
                2,
                3,
                4,
                2,
                2,
                4,
                4,
                3,
                2,
                5,
                4,
                4,
                2,
                6,
                4,
                5,
                2,
                7,
                4,
                6,
                2,
                8,
                4,
            ],
        ),
    ];
    let mut pwm = new(&trans);
    let on = [
        0x101, 0x102, 0x103, 0x104, 0x105, 0x106, 0x107, 0x108, 0x109, 0x200, 0x201, 0x202, 0x203,
        0x204, 0x205, 0x206,
    ];
    let off = [
        0x303, 0x304, 0x305, 0x306, 0x307, 0x308, 0x309, 0x400, 0x401, 0x402, 0x403, 0x404, 0x405,
        0x406, 0x407, 0x408,
    ];
    pwm.set_all_on_off(&on, &off).unwrap();
    destroy(pwm);
}

#[test]
fn can_get_all_on_off() {
    #[rustfmt::skip]
    let trans = [
        I2cTrans::write(DEV_ADDR, vec![Register::MODE1, MODE1_AI]),
        I2cTrans::write_read(
            DEV_ADDR,
            vec![Register::C0_ON_L],
            vec![
                  0,   0, 255, 255,
                  1,   0,   0,   0,
                  0,   0,   1,   0,
                255, 255,   0,   0,
                  0,   0, 255, 255,
                  0,   0,   0,   0,
                  0,   0,   1,   0,
                  0,   0,   0,   0,
                255, 255,   0,   0,
                  0,   0,   0,   0,
                  0,   0,   0,   0,
                  0,   0,   0,   0,
                  0,   0,   0,   0,
                  0,   0,   0,   0,
                  0,   0,   0,   0,
                  0,   0,   0,   0,
            ],
        ),
    ];

    let mut pwm = new(&trans);

    #[rustfmt::skip]
    assert_eq!(
        pwm.get_all_on_off().unwrap(),
        (
            [   0,    1,    0, 4095,    0,    0,    0,    0, 4095, 0, 0, 0, 0, 0, 0, 0],
            [4095,    0,    1,    0, 4095,    0,    1,    0,    0, 0, 0, 0, 0, 0, 0, 0]
        )
    );

    destroy(pwm);
}

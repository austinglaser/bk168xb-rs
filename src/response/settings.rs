use crate::{
    psu,
    psu::ArgFormat,
    response::{Response, Result},
};

/// The power supply's output settings.
///
/// This is the response format used by the
/// [`GetSettings`](crate::command::GetSettings) command.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Settings {
    /// Maximum output voltage.
    pub voltage: f32,

    /// Maximum output current.
    pub current: f32,
}

impl Response for Settings {
    fn arg_bytes() -> usize {
        6
    }

    fn parse_args(raw: &[u8], psu: &psu::Info) -> Result<Self> {
        let volt_fmt = ArgFormat {
            decimals: psu.voltage_decimals,
            digits: 3,
        };
        let curr_fmt = ArgFormat {
            decimals: psu.current_decimals,
            digits: 3,
        };

        let (volt_raw, curr_raw) = raw.split_at(volt_fmt.digits);
        let voltage = volt_fmt.parse(volt_raw).unwrap();
        let current = curr_fmt.parse(curr_raw).unwrap();

        Ok(Settings { voltage, current })
    }
}

#[cfg(test)]
galvanic_test::test_suite! {
    name test;

    use super::*;

    use crate::{
        psu::test_util::{high_voltage_psu, low_voltage_psu},
        response::test_util::expect_deserializes_to,
    };

    test can_parse_for_low_voltage(low_voltage_psu) {
        let psu = low_voltage_psu.val;

        let _e = expect_deserializes_to(
            "000000\rOK\r",
            Settings {
                voltage: 0.0,
                current: 0.0,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "999000\rOK\r",
            Settings {
                voltage: 99.9,
                current: 0.0,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "000999\rOK\r",
            Settings {
                voltage: 0.0,
                current: 99.9,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "123456\rOK\r",
            Settings {
                voltage: 12.3,
                current: 45.6,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "654321\rOK\r",
            Settings {
                voltage: 65.4,
                current: 32.1,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "025051\rOK\r",
            Settings {
                voltage: 2.5,
                current: 5.1,
            },
            psu
        );
    }

    test can_parse_for_high_voltage(high_voltage_psu) {
        let psu = high_voltage_psu.val;

        let _e = expect_deserializes_to(
            "000000\rOK\r",
            Settings {
                voltage: 0.0,
                current: 0.0,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "999000\rOK\r",
            Settings {
                voltage: 99.9,
                current: 0.0,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "000999\rOK\r",
            Settings {
                voltage: 0.0,
                current: 9.99,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "123456\rOK\r",
            Settings {
                voltage: 12.3,
                current: 4.56,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "654321\rOK\r",
            Settings {
                voltage: 65.4,
                current: 3.21,
            },
            psu
        );
        let _e = expect_deserializes_to(
            "025051\rOK\r",
            Settings {
                voltage: 2.5,
                current: 0.51,
            },
            psu
        );
    }
}

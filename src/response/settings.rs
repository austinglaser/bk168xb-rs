use crate::{
    response::{Response, Result},
    ArgFormat, SupplyVariant,
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

    fn parse_args(raw: &[u8], variant: &SupplyVariant) -> Result<Self> {
        let volt_fmt = ArgFormat {
            decimals: variant.voltage_decimals,
            digits: 3,
        };
        let curr_fmt = ArgFormat {
            decimals: variant.current_decimals,
            digits: 3,
        };

        let (volt_raw, curr_raw) = raw.split_at(volt_fmt.digits);
        let voltage = volt_fmt.parse(volt_raw)?;
        let current = curr_fmt.parse(curr_raw)?;

        Ok(Settings { voltage, current })
    }
}

#[cfg(test)]
galvanic_test::test_suite! {
    name test;

    use super::*;

    use crate::{
        response::{
            test_util::{expect_deserialize_error, expect_deserializes_to},
            Error::MalformedResponse,
        },
        test_util::{any_psu, high_voltage_psu, low_voltage_psu},
    };

    test can_parse_for_low_voltage(low_voltage_psu) {
        let variant = low_voltage_psu.val;

        let _e = expect_deserializes_to(
            "000000\rOK\r",
            Settings {
                voltage: 0.0,
                current: 0.0,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "999000\rOK\r",
            Settings {
                voltage: 99.9,
                current: 0.0,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "000999\rOK\r",
            Settings {
                voltage: 0.0,
                current: 99.9,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "123456\rOK\r",
            Settings {
                voltage: 12.3,
                current: 45.6,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "654321\rOK\r",
            Settings {
                voltage: 65.4,
                current: 32.1,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "025051\rOK\r",
            Settings {
                voltage: 2.5,
                current: 5.1,
            },
            variant
        );
    }

    test can_parse_for_high_voltage(high_voltage_psu) {
        let variant = high_voltage_psu.val;

        let _e = expect_deserializes_to(
            "000000\rOK\r",
            Settings {
                voltage: 0.0,
                current: 0.0,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "999000\rOK\r",
            Settings {
                voltage: 99.9,
                current: 0.0,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "000999\rOK\r",
            Settings {
                voltage: 0.0,
                current: 9.99,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "123456\rOK\r",
            Settings {
                voltage: 12.3,
                current: 4.56,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "654321\rOK\r",
            Settings {
                voltage: 65.4,
                current: 3.21,
            },
            variant
        );
        let _e = expect_deserializes_to(
            "025051\rOK\r",
            Settings {
                voltage: 2.5,
                current: 0.51,
            },
            variant
        );
    }

    test fails_to_parse_invalid_settings(any_psu) {
        let _e = expect_deserialize_error::<Settings>(
            "x00000\rOK\r",
            MalformedResponse,
            any_psu.val,
        );

        let _e = expect_deserialize_error::<Settings>(
            "000x00\rOK\r",
            MalformedResponse,
            any_psu.val,
        );

        let _e = expect_deserialize_error::<Settings>(
            "1234567\rOK\r",
            MalformedResponse,
            any_psu.val,
        );
    }
}

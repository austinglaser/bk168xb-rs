use crate::{
    response::{Error::MalformedResponse, Response, Result},
    ArgFormat, OutputMode, SupplyVariant,
};

/// The supply's instantaneous state.
///
/// This is the response format used by the
/// [`GetStatus`](crate::command::GetStatus) command.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Status {
    /// Output voltage \[v\].
    pub voltage: f32,

    /// Output curent \[A\].
    pub current: f32,

    /// Supply's output-limiting mode.
    pub mode: OutputMode,
}

impl Response for Status {
    fn arg_bytes() -> usize {
        9
    }

    fn parse_args(raw: &[u8], _variant: &SupplyVariant) -> Result<Self> {
        // n.b.: For both current and voltage, this response has a different
        // format than others. Firstly, it uses four digits. Secondly, each
        // field is specified to two decimal places -- regardless of the power
        // supply model.
        let arg_fmt = ArgFormat {
            decimals: 2,
            digits: 4,
        };

        let (&mode_raw, args_raw) =
            raw.split_last().ok_or(MalformedResponse)?;
        let (volt_raw, curr_raw) = args_raw.split_at(arg_fmt.digits);

        let voltage = arg_fmt.parse(volt_raw)?;
        let current = arg_fmt.parse(curr_raw)?;
        let mode = match mode_raw {
            b'0' => OutputMode::ConstantVoltage,
            b'1' => OutputMode::ConstantCurrent,
            _ => return Err(MalformedResponse),
        };

        Ok(Status {
            voltage,
            current,
            mode,
        })
    }
}

#[cfg(test)]
galvanic_test::test_suite! {
    name test;

    use super::*;

    use crate::{
        response::test_util::{
            expect_deserialize_error, expect_deserializes_to,
        },
        test_util::any_psu,
    };

    test can_parse(any_psu) {
        let _e = expect_deserializes_to(
            "000000000\rOK\r",
            Status {
                voltage: 0.0,
                current: 0.0,
                mode: OutputMode::ConstantVoltage,
            },
            any_psu.val
        );
        let _e = expect_deserializes_to(
            "999900000\rOK\r",
            Status {
                voltage: 99.99,
                current: 0.0,
                mode: OutputMode::ConstantVoltage,
            },
            any_psu.val
        );
        let _e = expect_deserializes_to(
            "000099990\rOK\r",
            Status {
                voltage: 0.0,
                current: 99.99,
                mode: OutputMode::ConstantVoltage,
            },
            any_psu.val
        );
        let _e = expect_deserializes_to(
            "000000001\rOK\r",
            Status {
                voltage: 0.0,
                current: 0.0,
                mode: OutputMode::ConstantCurrent,
            },
            any_psu.val
        );
        let _e = expect_deserializes_to(
            "123456780\rOK\r",
            Status {
                voltage: 12.34,
                current: 56.78,
                mode: OutputMode::ConstantVoltage,
            },
            any_psu.val
        );
        let _e = expect_deserializes_to(
            "987654321\rOK\r",
            Status {
                voltage: 98.76,
                current: 54.32,
                mode: OutputMode::ConstantCurrent,
            },
            any_psu.val
        );
        let _e = expect_deserializes_to(
            "030201450\rOK\r",
            Status {
                voltage: 3.02,
                current: 1.45,
                mode: OutputMode::ConstantVoltage,
            },
            any_psu.val
        );
    }

    test fails_to_parse_bad_param(any_psu) {
        let _e = expect_deserialize_error::<Status>(
            "foo000000\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "----00001\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "OK0000000\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "0000foo00\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "0000----1\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "0000OK000\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "0000blah1\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "123456782\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "888776543\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "000000009\rOK\r",
            MalformedResponse,
            any_psu.val
        );
        let _e = expect_deserialize_error::<Status>(
            "00000000X\rOK\r",
            MalformedResponse,
            any_psu.val
        );
    }
}

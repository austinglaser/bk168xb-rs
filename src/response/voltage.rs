use crate::{
    psu,
    response::{Response, Result},
};
use psu::ArgFormat;

/// A supply voltage.
///
/// This is the response format used by the
/// [`GetVoltageLimit`](crate::command::GetVoltageLimit) command.
#[derive(Debug, PartialEq)]
pub struct Voltage(f32);

impl Response for Voltage {
    fn arg_bytes() -> usize {
        3
    }

    fn parse_args(raw: &[u8], psu: &psu::Info) -> Result<Self> {
        let voltage_fmt = ArgFormat {
            decimals: psu.voltage_decimals,
            digits: Self::arg_bytes(),
        };

        let voltage = voltage_fmt.parse(raw)?;

        Ok(Voltage(voltage))
    }
}

#[cfg(test)]
galvanic_test::test_suite! {
    name test;

    use super::*;

    use crate::{
        psu::test_util::any_psu,
        response::{
            test_util::{
                assert_deserialize_error, assert_deserializes_to, invalid_num,
                valid_ack, valid_num, valid_sep,
            },
            ResponseError,
        },
    };

    test can_parse(any_psu, valid_num, valid_sep, valid_ack) {
        let psu = any_psu.val;
        let arg = valid_num.val;

        let mut resp = arg.raw.to_owned();
        resp.push(valid_sep.val);
        resp.push_str(valid_ack.val);

        assert_deserializes_to(&resp, Voltage(arg.one_decimal), psu);
    }

    test fails_to_parse_with_malformed_param(
        any_psu,
        invalid_num,
        valid_ack
    ) {
        let mut resp = invalid_num.val.to_owned();
        resp.push('\r');
        resp.push_str(valid_ack.val);
        assert_deserialize_error::<Voltage>(
            &resp,
            ResponseError::MalformedResponse,
            any_psu.val
        );
    }
}

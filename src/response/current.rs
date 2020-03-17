use crate::{
    response::{Response, Result},
    ArgFormat, SupplyVariant,
};

/// A power-supply response for a single current value.
///
/// This is the response format used by the
/// (`GetCurrentLimit`)[crate::command::GetCurrentLimit] command.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Current(pub f32);

impl Response for Current {
    fn arg_bytes() -> usize {
        3
    }

    fn parse_args(raw: &[u8], variant: &SupplyVariant) -> Result<Self> {
        let current_fmt = ArgFormat {
            decimals: variant.current_decimals,
            digits: Self::arg_bytes(),
        };

        let current = current_fmt.parse(raw)?;

        Ok(Current(current))
    }
}

impl From<f32> for Current {
    fn from(i: f32) -> Self {
        Current(i)
    }
}

#[cfg(test)]
galvanic_test::test_suite! {
    name test;

    use super::*;

    use crate::{
        response::{
            test_util::{
                assert_deserialize_error, assert_deserializes_to, invalid_num,
                valid_ack, valid_num, valid_sep,
            },
            Error,
        },
        test_util::{any_psu, high_voltage_psu, low_voltage_psu},
    };

    test can_parse_for_low_voltage(
        low_voltage_psu,
        valid_num,
        valid_sep,
        valid_ack
    ) {
        let variant = low_voltage_psu.val;
        let arg = valid_num.val;

        let mut resp = arg.raw.to_owned();
        resp.push(valid_sep.val);
        resp.push_str(valid_ack.val);

        assert_deserializes_to(&resp, Current(arg.one_decimal), variant);
    }

    test can_parse_for_high_voltage(
        high_voltage_psu,
        valid_num,
        valid_sep,
        valid_ack
    ) {
        let variant = high_voltage_psu.val;
        let arg = valid_num.val;

        let mut resp = arg.raw.to_owned();
        resp.push(valid_sep.val);
        resp.push_str(valid_ack.val);

        assert_deserializes_to(&resp, Current(arg.two_decimals), variant);
    }

    test fails_to_parse_with_malformed_param(
        any_psu,
        invalid_num,
        valid_ack
    ) {
        let mut resp = invalid_num.val.to_owned();
        resp.push('\r');
        resp.push_str(valid_ack.val);
        assert_deserialize_error::<Current>(
            &resp,
            Error::MalformedResponse,
            any_psu.val
        );
    }
}

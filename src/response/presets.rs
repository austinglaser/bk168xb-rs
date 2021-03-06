use crate::{
    response::{Error::MalformedResponse, Response, Result},
    ArgFormat, OperatingPoint, PresetIndex, SupplyVariant,
};
use std::ops::{Index, IndexMut};

/// The supply's pre-configured operating points.
///
/// This is the response format used by the
/// [`GetPresets`](crate::command::GetPresets) command.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Presets(pub OperatingPoint, pub OperatingPoint, pub OperatingPoint);

impl Response for Presets {
    fn arg_bytes() -> usize {
        // three 6-byte fields, two carriage-return separators
        6 * 3 + 2
    }

    fn parse_args(raw: &[u8], variant: &SupplyVariant) -> Result<Self> {
        let v_fmt = ArgFormat {
            decimals: variant.voltage_decimals,
            digits: 3,
        };
        let i_fmt = ArgFormat {
            decimals: variant.current_decimals,
            digits: 3,
        };

        let mut op_points = raw.split(|&b| b == b'\r');

        let p0_raw = op_points.next().ok_or(MalformedResponse)?;
        let p0 = Self::parse_operating_point(p0_raw, &v_fmt, &i_fmt)?;

        let p1_raw = op_points.next().ok_or(MalformedResponse)?;
        let p1 = Self::parse_operating_point(p1_raw, &v_fmt, &i_fmt)?;

        let p2_raw = op_points.next().ok_or(MalformedResponse)?;
        let p2 = Self::parse_operating_point(p2_raw, &v_fmt, &i_fmt)?;

        if op_points.next().is_some() {
            return Err(MalformedResponse);
        }

        Ok(Presets(p0, p1, p2))
    }
}

impl Presets {
    fn parse_operating_point(
        raw: &[u8],
        v_fmt: &ArgFormat,
        i_fmt: &ArgFormat,
    ) -> Result<OperatingPoint> {
        let (v_raw, i_raw) = raw.split_at(v_fmt.digits);
        let voltage = v_fmt.parse(v_raw)?;
        let current = i_fmt.parse(i_raw)?;

        Ok(OperatingPoint { voltage, current })
    }
}

impl Index<PresetIndex> for Presets {
    type Output = OperatingPoint;

    fn index(&self, i: PresetIndex) -> &Self::Output {
        match i {
            PresetIndex::One => &self.0,
            PresetIndex::Two => &self.1,
            PresetIndex::Three => &self.2,
        }
    }
}

impl IndexMut<PresetIndex> for Presets {
    fn index_mut(&mut self, i: PresetIndex) -> &mut Self::Output {
        match i {
            PresetIndex::One => &mut self.0,
            PresetIndex::Two => &mut self.1,
            PresetIndex::Three => &mut self.2,
        }
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

    test can_parse_for_low_voltage_psu(low_voltage_psu) {
        let variant = low_voltage_psu.val;
        let _e = expect_deserializes_to(
            "000000\r000000\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );

        let _e = expect_deserializes_to(
            "111000\r000000\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 11.1, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );
        let _e = expect_deserializes_to(
            "000222\r000000\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 22.2 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );

        let _e = expect_deserializes_to(
            "000000\r333000\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 33.3, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );
        let _e = expect_deserializes_to(
            "000000\r000444\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 44.4 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );

        let _e = expect_deserializes_to(
            "000000\r000000\r555000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 55.5, current: 0.0 },
            ),
            variant,
        );
        let _e = expect_deserializes_to(
            "000000\r000000\r000666\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 66.6 },
            ),
            variant,
        );

        let _e = expect_deserializes_to(
            "015015\r025025\r035035\rOK\r",
            Presets(
                OperatingPoint { voltage: 1.5, current: 1.5 },
                OperatingPoint { voltage: 2.5, current: 2.5 },
                OperatingPoint { voltage: 3.5, current: 3.5 },
            ),
            variant,
        );
    }

    test can_parse_for_high_voltage_psu(high_voltage_psu) {
        let variant = high_voltage_psu.val;
        let _e = expect_deserializes_to(
            "000000\r000000\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );

        let _e = expect_deserializes_to(
            "111000\r000000\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 11.1, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );
        let _e = expect_deserializes_to(
            "000222\r000000\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 2.22 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );

        let _e = expect_deserializes_to(
            "000000\r333000\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 33.3, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );
        let _e = expect_deserializes_to(
            "000000\r000444\r000000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 4.44 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
            ),
            variant,
        );

        let _e = expect_deserializes_to(
            "000000\r000000\r555000\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 55.5, current: 0.0 },
            ),
            variant,
        );
        let _e = expect_deserializes_to(
            "000000\r000000\r000666\rOK\r",
            Presets(
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 0.0 },
                OperatingPoint { voltage: 0.0, current: 6.66 },
            ),
            variant,
        );

        let _e = expect_deserializes_to(
            "015015\r025025\r035035\rOK\r",
            Presets(
                OperatingPoint { voltage: 1.5, current: 0.15 },
                OperatingPoint { voltage: 2.5, current: 0.25 },
                OperatingPoint { voltage: 3.5, current: 0.35 },
            ),
            variant,
        );
    }

    test fails_to_parse_with_missing_fields(any_psu) {
        let _e = expect_deserialize_error::<Presets>(
            "015015\r025025\rOK\r",
            MalformedResponse,
            any_psu.val,
        );

        // Correct character count => should exercise the parse_args function
        let _e = expect_deserialize_error::<Presets>(
            "015000015\r0250000025\rOK\r",
            MalformedResponse,
            any_psu.val,
        );
    }

    test fails_to_parse_with_extra_fields(any_psu) {
        let _e = expect_deserialize_error::<Presets>(
            "111111\r222222\r333333\r444444\rOK\r",
            MalformedResponse,
            any_psu.val,
        );

        // Correct character count => should exercise the parse_args function
        let _e = expect_deserialize_error::<Presets>(
            "111111\r22222\r33333\r9\rOK\r",
            MalformedResponse,
            any_psu.val,
        );
    }
}

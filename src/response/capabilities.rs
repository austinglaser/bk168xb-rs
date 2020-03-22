use crate::{
    response::{Response, Result},
    variant_for_max_voltage, ArgFormat, SupplyVariant,
};

/// The maximum output this hardware is capable of.
///
/// These are fixed unlike the "soft" limits
/// imposed by [`SetVoltageLimit`](crate::command::SetVoltageLimit) and
/// [`SetCurrentLimit`](crate::command::SetCurrentLimit).
///
/// *Note:* When parsed, this command ignores the supplied `variant`. Instead,
/// it determines the variant based on the maximum voltage the supply reports.
/// This behavior is important in order to support variant autodetect.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Capabilities {
    /// Maximum voltage that can be supplied.
    pub max_voltage: f32,

    /// Maximum current that can be supplied.
    ///
    /// *Note:* Currents > 5A can only be supplied through the rear terminals
    /// of the device, regardless of variant.
    pub max_current: f32,
}

impl Capabilities {
    /// Determine the variant suggested by these capabilities.
    ///
    /// If `None`, this means the capabilities do not match one of the known
    /// supplies in this crate. This should be unlikely when actually working
    /// with a supply.
    pub fn variant(self) -> Option<&'static SupplyVariant> {
        variant_for_max_voltage(self.max_voltage)
    }
}

impl Response for Capabilities {
    fn arg_bytes() -> usize {
        6
    }

    fn parse_args(raw: &[u8], _variant: &SupplyVariant) -> Result<Self> {
        // All supported supplies one decimal place in their voltage format
        let volt_fmt = ArgFormat {
            decimals: 1,
            digits: 3,
        };
        let (volt_raw, curr_raw) = raw.split_at(volt_fmt.digits);
        let voltage = volt_fmt.parse(volt_raw)?;

        // If we can't identify the supply (unlikely), fall back to the most
        // common case of using two decimal places.
        //
        // TODO: Should we just return an error here instead?
        let current_decimals = variant_for_max_voltage(voltage)
            .map(|v| v.current_decimals)
            .unwrap_or(1);

        let curr_fmt = ArgFormat {
            decimals: current_decimals,
            digits: 3,
        };

        let current = curr_fmt.parse(curr_raw)?;

        Ok(Capabilities {
            max_voltage: voltage,
            max_current: current,
        })
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
        test_util::any_psu,
        BK1685B, BK1687B, BK1688B,
    };
    use galvanic_assert::{
        expect_that, get_expectation_for, has_structure, matchers::*, structure,
    };

    test parses_for_60v_supply(any_psu) {
        let _e = expect_deserializes_to(
            "600500\rOK\r",
            Capabilities {
                max_voltage: 60.0,
                max_current: 5.0,
            },
            any_psu.val
        );

        let _e = expect_deserializes_to(
            "601501\rOK\r",
            Capabilities {
                max_voltage: 60.1,
                max_current: 5.01,
            },
            any_psu.val
        );

        let _e = expect_deserializes_to(
            "623579\rOK\r",
            Capabilities {
                max_voltage: 62.3,
                max_current: 5.79,
            },
            any_psu.val
        );
    }

    test reports_60v_supply() {
        let caps_nominal = Capabilities {
            max_voltage: 60.0,
            max_current: 5.0,
        };
        expect_that!(
            &caps_nominal.variant(),
            has_structure!(Some [eq(BK1685B)])
        );

        let caps_imprecise = Capabilities {
            max_voltage: 60.1,
            max_current: 5.01,
        };
        expect_that!(
            &caps_imprecise.variant(),
            has_structure!(Some [eq(BK1685B)])
        );

        let caps_high = Capabilities {
            max_voltage: 62.3,
            max_current: 5.79,
        };
        expect_that!(
            &caps_high.variant(),
            has_structure!(Some [eq(BK1685B)])
        );
    }

    test parses_for_36v_supply(any_psu) {
        let _e = expect_deserializes_to(
            "360100\rOK\r",
            Capabilities {
                max_voltage: 36.0,
                max_current: 10.0,
            },
            any_psu.val
        );

        let _e = expect_deserializes_to(
            "361101\rOK\r",
            Capabilities {
                max_voltage: 36.1,
                max_current: 10.1,
            },
            any_psu.val
        );

        let _e = expect_deserializes_to(
            "383109\rOK\r",
            Capabilities {
                max_voltage: 38.3,
                max_current: 10.9,
            },
            any_psu.val
        );
    }

    test reports_36v_supply() {
        let caps_nominal = Capabilities {
            max_voltage: 36.0,
            max_current: 10.0,
        };
        expect_that!(
            &caps_nominal.variant(),
            has_structure!(Some [eq(BK1687B)])
        );

        let caps_imprecise = Capabilities {
            max_voltage: 36.1,
            max_current: 10.1,
        };
        expect_that!(
            &caps_imprecise.variant(),
            has_structure!(Some [eq(BK1687B)])
        );

        let caps_high = Capabilities {
            max_voltage: 38.3,
            max_current: 10.9,
        };
        expect_that!(
            &caps_high.variant(),
            has_structure!(Some [eq(BK1687B)])
        );
    }

    test parses_for_18v_supply(any_psu) {
        let _e = expect_deserializes_to(
            "180200\rOK\r",
            Capabilities {
                max_voltage: 18.0,
                max_current: 20.0,
            },
            any_psu.val
        );

        let _e = expect_deserializes_to(
            "181201\rOK\r",
            Capabilities {
                max_voltage: 18.1,
                max_current: 20.1,
            },
            any_psu.val
        );

        let _e = expect_deserializes_to(
            "193209\rOK\r",
            Capabilities {
                max_voltage: 19.3,
                max_current: 20.9,
            },
            any_psu.val
        );
    }

    test reports_18v_supply() {
        let caps_nominal = Capabilities {
            max_voltage: 18.0,
            max_current: 20.0,
        };
        expect_that!(
            &caps_nominal.variant(),
            has_structure!(Some [eq(BK1688B)])
        );

        let caps_imprecise = Capabilities {
            max_voltage: 18.1,
            max_current: 20.1,
        };
        expect_that!(
            &caps_imprecise.variant(),
            has_structure!(Some [eq(BK1688B)])
        );

        let caps_high = Capabilities {
            max_voltage: 19.3,
            max_current: 20.9,
        };
        expect_that!(
            &caps_high.variant(),
            has_structure!(Some [eq(BK1688B)])
        );
    }

    test only_reports_in_range() {
        let zero = Capabilities {
            max_voltage: 0.0,
            max_current: 0.0,
        };
        expect_that!(&zero.variant(), eq(None));

        let below_low = Capabilities {
            max_voltage: 17.0,
            max_current: 20.0,
        };
        expect_that!(&below_low.variant(), eq(None));

        let above_low = Capabilities {
            max_voltage: 29.9,
            max_current: 10.9,
        };
        expect_that!(&above_low.variant(), eq(None));

        let below_med = Capabilities {
            max_voltage: 35.0,
            max_current: 10.9,
        };
        expect_that!(&below_med.variant(), eq(None));

        let above_med = Capabilities {
            max_voltage: 47.2,
            max_current: 5.3,
        };
        expect_that!(&above_med.variant(), eq(None));

        let below_hi = Capabilities {
            max_voltage: 58.7,
            max_current: 5.0,
        };
        expect_that!(&below_hi.variant(), eq(None));

        let above_hi = Capabilities {
            max_voltage: 72.1,
            max_current: 5.0,
        };
        expect_that!(&above_hi.variant(), eq(None));

        let ninetynine = Capabilities {
            max_voltage: 99.9,
            max_current: 99.9,
        };
        expect_that!(&ninetynine.variant(), eq(None));
    }

    test fails_to_parse_invalid_settings(any_psu) {
        let _e = expect_deserialize_error::<Capabilities>(
            "x00000\rOK\r",
            MalformedResponse,
            any_psu.val,
        );

        let _e = expect_deserialize_error::<Capabilities>(
            "000x00\rOK\r",
            MalformedResponse,
            any_psu.val,
        );

        let _e = expect_deserialize_error::<Capabilities>(
            "1234567\rOK\r",
            MalformedResponse,
            any_psu.val,
        );
    }
}

//! Command for setting supply operating current

use crate::{
    command::{self, Command},
    response::Current,
    ArgFormat, SupplyVariant,
};

use std::io;

/// Set the supply's operating current.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SetCurrent(pub f32);

impl Command for SetCurrent {
    const FUNCTION: &'static str = "CURR";

    fn serialize_args<S: io::Write>(
        &self,
        sink: &mut S,
        variant: &SupplyVariant,
    ) -> command::Result<()> {
        let fmt = ArgFormat {
            decimals: variant.current_decimals,
            digits: 3,
        };

        fmt.serialize_arg(sink, self.0)
    }
}

impl From<Current> for SetCurrent {
    fn from(c: Current) -> Self {
        SetCurrent(c.0)
    }
}

impl From<f32> for SetCurrent {
    fn from(i: f32) -> Self {
        SetCurrent(i)
    }
}

#[cfg(test)]
use galvanic_test::test_suite;

#[cfg(test)]
test_suite! {
    name test;

    use super::*;

    use crate::{
        command::test_util::{assert_cant_serialize, expect_serializes_to},
        test_util::{
            high_voltage_psu, invalid_current_high_voltage,
            invalid_current_low_voltage, low_voltage_psu,
        },
    };

    test serialize_for_low_v_psu(low_voltage_psu) {
        let variant = low_voltage_psu.val;
        let _e = expect_serializes_to(SetCurrent(0.), "CURR000\r", variant);
        let _e = expect_serializes_to(SetCurrent(0.5), "CURR005\r", variant);
        let _e = expect_serializes_to(SetCurrent(1.5), "CURR015\r", variant);
        let _e = expect_serializes_to(SetCurrent(1.5), "CURR015\r", variant);
        let _e = expect_serializes_to(SetCurrent(12.3), "CURR123\r", variant);
        let _e = expect_serializes_to(SetCurrent(99.9), "CURR999\r", variant);
        let _e = expect_serializes_to(SetCurrent(8.21), "CURR082\r", variant);
        let _e = expect_serializes_to(SetCurrent(12.99), "CURR130\r", variant);
    }

    test cant_serialize_if_unrepresentable_on_low_v(
        low_voltage_psu,
        invalid_current_low_voltage
    ) {
        assert_cant_serialize(
            SetCurrent(invalid_current_low_voltage.val),
            low_voltage_psu.val
        );
    }

    test serialize_for_high_v_psu(high_voltage_psu) {
        let variant = high_voltage_psu.val;
        let _e = expect_serializes_to(SetCurrent(0.), "CURR000\r", variant);
        let _e = expect_serializes_to(SetCurrent(0.5), "CURR050\r", variant);
        let _e = expect_serializes_to(SetCurrent(1.55), "CURR155\r", variant);
        let _e = expect_serializes_to(SetCurrent(2.31), "CURR231\r", variant);
        let _e = expect_serializes_to(SetCurrent(9.99), "CURR999\r", variant);
        let _e = expect_serializes_to(SetCurrent(0.821), "CURR082\r", variant);
        let _e = expect_serializes_to(SetCurrent(1.299), "CURR130\r", variant);
    }

    test cant_serialize_if_unrepresentable_on_high_v(
        high_voltage_psu,
        invalid_current_high_voltage
    ) {
        assert_cant_serialize(
            SetCurrent(invalid_current_high_voltage.val),
            high_voltage_psu.val
        );
    }
}

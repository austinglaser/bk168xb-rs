//! Command for setting supply operating voltage.

use crate::{
    command::{self, Command},
    response::Voltage,
    ArgFormat, SupplyVariant,
};

use std::io;

/// Set the supply's operating voltage.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SetVoltage(pub f32);

impl Command for SetVoltage {
    const FUNCTION: &'static str = "VOLT";

    fn serialize_args<S: io::Write>(
        &self,
        sink: &mut S,
        variant: &SupplyVariant,
    ) -> command::Result<()> {
        let fmt = ArgFormat {
            decimals: variant.voltage_decimals,
            digits: 3,
        };

        fmt.serialize_arg(sink, self.0)
    }
}

impl From<Voltage> for SetVoltage {
    fn from(v: Voltage) -> Self {
        SetVoltage(v.0)
    }
}

impl From<f32> for SetVoltage {
    fn from(v: f32) -> Self {
        SetVoltage(v)
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
        test_util::{any_psu, invalid_voltage},
    };

    test can_serialize(any_psu) {
        let variant = any_psu.val;
        let _e = expect_serializes_to(SetVoltage(12.3), "VOLT123\r", variant);
        let _e = expect_serializes_to(SetVoltage(0.), "VOLT000\r", variant);
        let _e = expect_serializes_to(SetVoltage(0.1), "VOLT001\r", variant);
        let _e = expect_serializes_to(SetVoltage(99.9), "VOLT999\r", variant);
        let _e = expect_serializes_to(SetVoltage(8.21), "VOLT082\r", variant);
        let _e = expect_serializes_to(SetVoltage(12.99), "VOLT130\r", variant);
    }

    test cant_serialize_if_unrepresentable(any_psu, invalid_voltage) {
        assert_cant_serialize(SetVoltage(invalid_voltage.val), any_psu.val);
    }
}

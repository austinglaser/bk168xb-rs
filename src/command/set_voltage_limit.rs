//! Command for setting a "soft" voltage limit.
use crate::{
    command::{self, Command},
    response::Voltage,
    ArgFormat, SupplyVariant,
};

use std::io;

/// Set a "soft" limit on programmable voltage.
///
/// This limit applies to settings via the front panel, but can be lifted via
/// USB-serial control.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SetVoltageLimit(pub f32);

impl Command for SetVoltageLimit {
    const FUNCTION: &'static str = "SOVP";

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

impl From<Voltage> for SetVoltageLimit {
    fn from(v: Voltage) -> Self {
        SetVoltageLimit(v.0)
    }
}

impl From<f32> for SetVoltageLimit {
    fn from(v: f32) -> Self {
        SetVoltageLimit(v)
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
        let _e = expect_serializes_to(SetVoltageLimit(12.3), "SOVP123\r", variant);
        let _e = expect_serializes_to(SetVoltageLimit(0.), "SOVP000\r", variant);
        let _e = expect_serializes_to(SetVoltageLimit(0.1), "SOVP001\r", variant);
        let _e = expect_serializes_to(SetVoltageLimit(99.9), "SOVP999\r", variant);
        let _e = expect_serializes_to(SetVoltageLimit(8.21), "SOVP082\r", variant);
        let _e = expect_serializes_to(SetVoltageLimit(12.99), "SOVP130\r", variant);
    }

    test cant_serialize_if_unrepresentable(any_psu, invalid_voltage) {
        let variant = any_psu.val;
        assert_cant_serialize(SetVoltageLimit(invalid_voltage.val), variant);
    }
}

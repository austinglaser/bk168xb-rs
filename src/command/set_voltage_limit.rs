///! Command for setting a "soft" voltage limit.
use crate::command::{self, ArgFormat, Command};
use crate::psu;

use std::io;

/// Set a "soft" limit on programmable voltage.
///
/// This limit applies to settings via the front panel, but can be lifted via
/// USB-serial control.
pub struct SetVoltageLimit(f32);

impl Command for SetVoltageLimit {
    const FUNCTION: &'static str = "SOVP";

    fn serialize_args<S: io::Write>(
        &self,
        mut sink: S,
        psu: &psu::Info,
    ) -> command::Result<()> {
        let fmt = ArgFormat {
            decimals: psu.voltage_decimals,
            digits: 3,
        };

        fmt.serialize_arg(&mut sink, self.0)
    }
}

#[cfg(test)]
use galvanic_test::test_suite;

#[cfg(test)]
test_suite! {
    name test;

    use super::*;

    use crate::command::test_util::{
        expect_serializes_to,
        assert_cant_serialize,
    };
    use crate::psu::test_util::{any_psu, invalid_voltage};

    test can_serialize(any_psu) {
        let psu = any_psu.val;
        let _e = expect_serializes_to(SetVoltageLimit(12.3), "SOVP123\r", psu);
        let _e = expect_serializes_to(SetVoltageLimit(0.), "SOVP000\r", psu);
        let _e = expect_serializes_to(SetVoltageLimit(0.1), "SOVP001\r", psu);
        let _e = expect_serializes_to(SetVoltageLimit(99.9), "SOVP999\r", psu);
        let _e = expect_serializes_to(SetVoltageLimit(8.21), "SOVP082\r", psu);
        let _e = expect_serializes_to(SetVoltageLimit(12.99), "SOVP130\r", psu);
    }

    test cant_serialize_if_unrepresentable(any_psu, invalid_voltage) {
        let psu = any_psu.val;
        assert_cant_serialize(SetVoltageLimit(invalid_voltage.val), psu);
    }
}

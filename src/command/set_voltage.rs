//! Command for setting supply operating voltage.

use crate::{
    command::{self, ArgFormat, Command},
    psu,
};

use std::io;

/// Set the supply's operating voltage.
#[derive(Debug, PartialEq)]
pub struct SetVoltage(f32);

impl Command for SetVoltage {
    const FUNCTION: &'static str = "VOLT";

    fn serialize_args<S: io::Write>(
        &self,
        sink: &mut S,
        psu: &psu::Info,
    ) -> command::Result<()> {
        let fmt = ArgFormat {
            decimals: psu.voltage_decimals,
            digits: 3,
        };

        fmt.serialize_arg(sink, self.0)
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
        let _e = expect_serializes_to(SetVoltage(12.3), "VOLT123\r", psu);
        let _e = expect_serializes_to(SetVoltage(0.), "VOLT000\r", psu);
        let _e = expect_serializes_to(SetVoltage(0.1), "VOLT001\r", psu);
        let _e = expect_serializes_to(SetVoltage(99.9), "VOLT999\r", psu);
        let _e = expect_serializes_to(SetVoltage(8.21), "VOLT082\r", psu);
        let _e = expect_serializes_to(SetVoltage(12.99), "VOLT130\r", psu);
    }

    test cant_serialize_if_unrepresentable(any_psu, invalid_voltage) {
        assert_cant_serialize(SetVoltage(invalid_voltage.val), any_psu.val);
    }
}

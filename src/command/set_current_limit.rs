///! Command for setting a "soft" current limit.
use crate::command::{ArgFormat, Command};
use crate::psu;

use std::io;

/// Set a "soft" limit on programmable current.
///
/// This limit applies to settings via the front panel, but can be lifted via
/// USB-serial control.
pub struct SetCurrentLimit(f32);

impl Command for SetCurrentLimit {
    const FUNCTION: &'static str = "SOCP";

    fn serialize_args<S: io::Write>(
        &self,
        mut sink: S,
        psu: &psu::Info,
    ) -> io::Result<()> {
        let fmt = ArgFormat {
            decimals: psu.current_decimals(),
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
    use crate::psu::test::{high_voltage_psu, low_voltage_psu};
    use crate::psu::test::{
        invalid_current_high_voltage,
        invalid_current_low_voltage
    };

    test serialize_for_low_v_psu(low_voltage_psu) {
        let psu = low_voltage_psu.val;
        let _e = expect_serializes_to(SetCurrentLimit(0.), "SOCP000\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(0.5), "SOCP005\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(1.5), "SOCP015\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(12.3), "SOCP123\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(99.9), "SOCP999\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(8.21), "SOCP082\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(12.99), "SOCP130\r", psu);
    }

    test cant_serialize_if_unrepresentable_on_low_v(
        low_voltage_psu,
        invalid_current_low_voltage
    ) {
        assert_cant_serialize(
            SetCurrentLimit(invalid_current_low_voltage.val),
            low_voltage_psu.val
        );
    }

    test serialize_for_high_v_psu(high_voltage_psu) {
        let psu = high_voltage_psu.val;
        let _e = expect_serializes_to(SetCurrentLimit(0.), "SOCP000\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(0.5), "SOCP050\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(1.55), "SOCP155\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(2.31), "SOCP231\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(9.99), "SOCP999\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(0.821), "SOCP082\r", psu);
        let _e = expect_serializes_to(SetCurrentLimit(1.299), "SOCP130\r", psu);
    }

    test cant_serialize_if_unrepresentable_on_high_v(
        high_voltage_psu,
        invalid_current_high_voltage
    ) {
        assert_cant_serialize(
            SetCurrentLimit(invalid_current_high_voltage.val),
            high_voltage_psu.val
        );
    }
}

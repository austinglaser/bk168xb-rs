//! Command for setting supply operating current

use crate::command::{self, ArgFormat, Command};
use crate::psu;

use std::io;

/// Set the supply's operating current.
pub struct SetCurrent(f32);

impl Command for SetCurrent {
    const FUNCTION: &'static str = "CURR";

    fn serialize_args<S: io::Write>(
        &self,
        mut sink: S,
        psu: &psu::Info,
    ) -> command::Result<()> {
        let fmt = ArgFormat {
            decimals: psu.current_decimals,
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
    use crate::psu::test_util::{high_voltage_psu, low_voltage_psu};
    use crate::psu::test_util::{
        invalid_current_high_voltage,
        invalid_current_low_voltage
    };

    test serialize_for_low_v_psu(low_voltage_psu) {
        let psu = low_voltage_psu.val;
        let _e = expect_serializes_to(SetCurrent(0.), "CURR000\r", psu);
        let _e = expect_serializes_to(SetCurrent(0.5), "CURR005\r", psu);
        let _e = expect_serializes_to(SetCurrent(1.5), "CURR015\r", psu);
        let _e = expect_serializes_to(SetCurrent(1.5), "CURR015\r", psu);
        let _e = expect_serializes_to(SetCurrent(12.3), "CURR123\r", psu);
        let _e = expect_serializes_to(SetCurrent(99.9), "CURR999\r", psu);
        let _e = expect_serializes_to(SetCurrent(8.21), "CURR082\r", psu);
        let _e = expect_serializes_to(SetCurrent(12.99), "CURR130\r", psu);
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
        let psu = high_voltage_psu.val;
        let _e = expect_serializes_to(SetCurrent(0.), "CURR000\r", psu);
        let _e = expect_serializes_to(SetCurrent(0.5), "CURR050\r", psu);
        let _e = expect_serializes_to(SetCurrent(1.55), "CURR155\r", psu);
        let _e = expect_serializes_to(SetCurrent(2.31), "CURR231\r", psu);
        let _e = expect_serializes_to(SetCurrent(9.99), "CURR999\r", psu);
        let _e = expect_serializes_to(SetCurrent(0.821), "CURR082\r", psu);
        let _e = expect_serializes_to(SetCurrent(1.299), "CURR130\r", psu);
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

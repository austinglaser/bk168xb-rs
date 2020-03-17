///! Commands for getting values from the supply.
use crate::command::Command;
use crate::response::{Ack, Current, Presets, Settings, Status, Voltage};

/// Get the current output voltage and current
///
/// Set through `SetVoltage` and `SetCurrent`
#[derive(Debug, PartialEq)]
pub struct GetSettings;

impl Command for GetSettings {
    type Response = Settings;

    const FUNCTION: &'static str = "GETS";
}

/// Get the current supply status, as displayed on the front panel.
///
/// This consists of:
///
/// - Actual output voltage
/// - Actual output current
/// - Output mode (constant current or constant voltage)
#[derive(Debug, PartialEq)]
pub struct GetStatus;

impl Command for GetStatus {
    type Response = Status;

    const FUNCTION: &'static str = "GETD";
}

/// Get the maximum acceptable supply voltage.
///
/// Set through `SetVoltageLimit`
#[derive(Debug, PartialEq)]
pub struct GetVoltageLimit;

impl Command for GetVoltageLimit {
    type Response = Voltage;

    const FUNCTION: &'static str = "GOVP";
}

/// Get the maximum acceptable supply current.
///
/// Set through `SetCurrentLimit`
#[derive(Debug, PartialEq)]
pub struct GetCurrentLimit;

impl Command for GetCurrentLimit {
    type Response = Current;

    const FUNCTION: &'static str = "GOCP";
}

/// Determine the supply's absolute maximum voltage/current limits.
///
/// This is unaffected by the "soft" limits imposed by `SetVoltageLimit`
/// and `SetCurrentLimit`.
#[derive(Debug, PartialEq)]
pub struct GetCapabilities;

impl Command for GetCapabilities {
    // XXX: This is wrong -- response not implemented yet
    type Response = Ack;

    const FUNCTION: &'static str = "GMAX";
}

/// Get a list of the pre-set operating points.
pub struct GetPresets;

impl Command for GetPresets {
    type Response = Presets;

    const FUNCTION: &'static str = "GETM";
}

#[cfg(test)]
use galvanic_test::test_suite;

#[cfg(test)]
test_suite! {
    name test;

    use super::*;

    use crate::command::test_util::*;
    use crate::psu::test_util::any_psu;

    test serialize_get_settings(any_psu) {
        assert_serializes_to(GetSettings, "GETS\r", any_psu.val);
    }

    test serialize_get_status(any_psu) {
        assert_serializes_to(GetStatus, "GETD\r", any_psu.val);
    }

    test serialize_get_voltage_limit(any_psu) {
        assert_serializes_to(GetVoltageLimit, "GOVP\r", any_psu.val);
    }

    test serialize_get_current_limit(any_psu) {
        assert_serializes_to(GetCurrentLimit, "GOCP\r", any_psu.val);
    }

    test serialize_get_capabilities(any_psu) {
        assert_serializes_to(GetCapabilities, "GMAX\r", any_psu.val);
    }

    test serialize_get_presets(any_psu) {
        assert_serializes_to(GetPresets, "GETM\r", any_psu.val);
    }

}

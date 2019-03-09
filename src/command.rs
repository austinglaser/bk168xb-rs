//! Power supply command set

use crate::psu;
use std::io;

/// A PSU command.
pub trait Command {
    /// Function-discrimination part of a command.
    ///
    /// Each command starts with a four-character "function." This describes
    /// what operation is being performed.
    const FUNCTION: &'static str;

    /// Write a command's arguments to the specified sink.
    ///
    /// The default implementation of this function serializes no arguments.
    ///
    /// # Arguments
    ///
    /// - `psu`: Provides information about per-supply serialization quirks
    /// - `sink`: Where to write arguments
    fn serialize_args<S: io::Write>(
        &self,
        mut _sink: S,
        _psu: &psu::Info,
    ) -> io::Result<()> {
        Ok(())
    }
}

/// A command that can be serialized.
pub trait Serialize {
    /// Write the command to the specified sink.
    ///
    /// # Arguments
    ///
    /// - `psu`: Provides information about per-supply serialization quirks
    /// - `sink`: Where to write arguments
    fn serialize<S: io::Write>(
        &self,
        sink: S,
        psu: &psu::Info,
    ) -> io::Result<()>;
}

impl<C> Serialize for C
where
    C: Command,
{
    fn serialize<S: io::Write>(
        &self,
        mut sink: S,
        psu: &psu::Info,
    ) -> io::Result<()> {
        write!(&mut sink, "{}", C::FUNCTION)?;
        self.serialize_args(&mut sink, psu)?;
        write!(&mut sink, "\r")?;

        Ok(())
    }
}

/// Set the supply's operating voltage.
pub struct SetVoltage(f32);

impl Command for SetVoltage {
    const FUNCTION: &'static str = "VOLT";

    fn serialize_args<S: io::Write>(
        &self,
        mut sink: S,
        psu: &psu::Info,
    ) -> io::Result<()> {
        let fmt = ArgFormat {
            decimals: psu.voltage_decimals(),
            digits: 3,
        };

        fmt.serialize_arg(&mut sink, self.0)
    }
}

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
    ) -> io::Result<()> {
        let fmt = ArgFormat {
            decimals: psu.voltage_decimals(),
            digits: 3,
        };

        fmt.serialize_arg(&mut sink, self.0)
    }
}

/// Set the supply's operating current.
pub struct SetCurrent(f32);

impl Command for SetCurrent {
    const FUNCTION: &'static str = "CURR";

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

/// Control whether the supply is supplying power.
pub struct SetOutput(psu::OutputState);

impl Command for SetOutput {
    const FUNCTION: &'static str = "SOUT";

    fn serialize_args<S: io::Write>(
        &self,
        mut sink: S,
        _psu: &psu::Info,
    ) -> io::Result<()> {
        let fmt = ArgFormat {
            decimals: 0,
            digits: 1,
        };

        fmt.serialize_arg(&mut sink, self.0.arg_val() as f32)
    }
}

/// Configure the supply's pre-set operating points.
pub struct SetPresets(
    psu::OperatingPoint,
    psu::OperatingPoint,
    psu::OperatingPoint,
);

impl Command for SetPresets {
    const FUNCTION: &'static str = "PROM";

    fn serialize_args<S: io::Write>(
        &self,
        mut sink: S,
        psu: &psu::Info,
    ) -> io::Result<()> {
        let v_fmt = ArgFormat {
            decimals: psu.voltage_decimals(),
            digits: 3,
        };

        let i_fmt = ArgFormat {
            decimals: psu.current_decimals(),
            digits: 3,
        };

        v_fmt.serialize_arg(&mut sink, self.0.voltage)?;
        i_fmt.serialize_arg(&mut sink, self.0.current)?;
        v_fmt.serialize_arg(&mut sink, self.1.voltage)?;
        i_fmt.serialize_arg(&mut sink, self.1.current)?;
        v_fmt.serialize_arg(&mut sink, self.2.voltage)?;
        i_fmt.serialize_arg(&mut sink, self.2.current)?;

        Ok(())
    }
}

/// Select a preset previously set with `SetPresets`
pub struct SelectPreset(psu::PresetIndex);

impl Command for SelectPreset {
    const FUNCTION: &'static str = "RUNM";

    fn serialize_args<S: io::Write>(
        &self,
        mut sink: S,
        _psu: &psu::Info,
    ) -> io::Result<()> {
        let fmt = ArgFormat {
            decimals: 0,
            digits: 1,
        };

        fmt.serialize_arg(&mut sink, self.0.arg_val() as f32)
    }
}

/// Get the current output voltage and current
///
/// Set through `SetVoltage` and `SetCurrent`
pub struct GetSettings;

impl Command for GetSettings {
    const FUNCTION: &'static str = "GETS";
}

/// Get the current supply status, as displayed on the front panel.
///
/// This consists of:
///
/// - Actual output voltage
/// - Actual output current
/// - Output mode (constant current or constant voltage)
pub struct GetStatus;

impl Command for GetStatus {
    const FUNCTION: &'static str = "GETD";
}

/// Get the maximum acceptable supply voltage.
///
/// Set through `SetVoltageLimit`
pub struct GetVoltageLimit;

impl Command for GetVoltageLimit {
    const FUNCTION: &'static str = "GOVP";
}

/// Get the maximum acceptable supply current.
///
/// Set through `SetCurrentLimit`
pub struct GetCurrentLimit;

impl Command for GetCurrentLimit {
    const FUNCTION: &'static str = "GOCP";
}

/// Determine the supply's absolute maximum voltage/current limits.
///
/// This is unaffected by the "soft" limits imposed by `SetVoltageLimit`
/// and `SetCurrentLimit`.
pub struct GetCapabilities;

impl Command for GetCapabilities {
    const FUNCTION: &'static str = "GMAX";
}

/// Get a list of the pre-set operating points.
pub struct GetPresets;

impl Command for GetPresets {
    const FUNCTION: &'static str = "GETM";
}

struct ArgFormat {
    pub decimals: usize,
    pub digits: usize,
}

impl ArgFormat {
    fn serialize_arg<S: io::Write>(
        &self,
        mut sink: S,
        val: f32,
    ) -> io::Result<()> {
        if let Some(value) = self.output_val(val) {
            write!(&mut sink, "{arg:0width$}", arg = value, width = self.digits)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "unrepresentable arg"))
        }
    }

    fn output_val(&self, val: f32) -> Option<u32> {
        let multiplier = f32::powi(10., self.decimals as i32);
        let max = (f32::powi(10., self.digits as i32) - 1.) / multiplier;

        if !val.is_finite() || val < 0. || val > max {
            return None;
        }

        let output_val = (val * multiplier).round() as u32;

        Some(output_val)
    }
}

#[cfg(test)]
use galvanic_test::test_suite;

#[cfg(test)]
test_suite! {
    name test;

    use super::*;

    use crate::psu::{self, BK1685B, BK1687B, BK1688B, OperatingPoint, OutputState, PresetIndex};

    use std::io::Cursor;
    use std::str;

    use galvanic_assert::{Expectation, get_expectation_for, matchers::*};

    test serialize_set_voltage(any_psu) {
        let _e = expect_serializes_to(SetVoltage(12.3), "VOLT123\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltage(0.), "VOLT000\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltage(0.1), "VOLT001\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltage(99.9), "VOLT999\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltage(8.21), "VOLT082\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltage(12.99), "VOLT130\r", any_psu.val);
    }

    test cant_serialize_unrepresentable_set_voltage(any_psu, invalid_voltage) {
        assert_cant_serialize(SetVoltage(invalid_voltage.val), any_psu.val);
    }

    test serialize_set_voltage_limit(any_psu) {
        let _e = expect_serializes_to(SetVoltageLimit(12.3), "SOVP123\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltageLimit(0.), "SOVP000\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltageLimit(0.1), "SOVP001\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltageLimit(99.9), "SOVP999\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltageLimit(8.21), "SOVP082\r", any_psu.val);
        let _e = expect_serializes_to(SetVoltageLimit(12.99), "SOVP130\r", any_psu.val);
    }

    test cant_serialize_unrepresentable_set_voltage_limit(any_psu, invalid_voltage) {
        assert_cant_serialize(SetVoltageLimit(invalid_voltage.val), any_psu.val);
    }

    test serialize_set_current_low_voltage(low_voltage_psu) {
        let _e = expect_serializes_to(SetCurrent(0.), "CURR000\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(0.5), "CURR005\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(1.5), "CURR015\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(1.5), "CURR015\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(12.3), "CURR123\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(99.9), "CURR999\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(8.21), "CURR082\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(12.99), "CURR130\r", low_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_current_low_voltage(
        low_voltage_psu,
        invalid_current_low_voltage
    ) {
        assert_cant_serialize(
            SetCurrent(invalid_current_low_voltage.val),
            low_voltage_psu.val
        );
    }

    test serialize_set_current_high_voltage(high_voltage_psu) {
        let _e = expect_serializes_to(SetCurrent(0.), "CURR000\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(0.5), "CURR050\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(1.55), "CURR155\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(2.31), "CURR231\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(9.99), "CURR999\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(0.821), "CURR082\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrent(1.299), "CURR130\r", high_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_current_high_voltage(
        high_voltage_psu,
        invalid_current_high_voltage
    ) {
        assert_cant_serialize(
            SetCurrent(invalid_current_high_voltage.val),
            high_voltage_psu.val
        );
    }

    test serialize_set_current_limit_low_voltage(low_voltage_psu) {
        let _e = expect_serializes_to(SetCurrentLimit(0.), "SOCP000\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(0.5), "SOCP005\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(1.5), "SOCP015\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(1.5), "SOCP015\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(12.3), "SOCP123\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(99.9), "SOCP999\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(8.21), "SOCP082\r", low_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(12.99), "SOCP130\r", low_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_current_limit_low_voltage(
        low_voltage_psu,
        invalid_current_low_voltage
    ) {
        assert_cant_serialize(
            SetCurrentLimit(invalid_current_low_voltage.val),
            low_voltage_psu.val
        );
    }

    test serialize_set_current_limit_high_voltage(high_voltage_psu) {
        let _e = expect_serializes_to(SetCurrentLimit(0.), "SOCP000\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(0.5), "SOCP050\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(1.55), "SOCP155\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(2.31), "SOCP231\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(9.99), "SOCP999\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(0.821), "SOCP082\r", high_voltage_psu.val);
        let _e = expect_serializes_to(SetCurrentLimit(1.299), "SOCP130\r", high_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_current_limit_high_voltage(
        high_voltage_psu,
        invalid_current_high_voltage
    ) {
        assert_cant_serialize(
            SetCurrentLimit(invalid_current_high_voltage.val),
            high_voltage_psu.val
        );
    }

    test serialize_set_output(any_psu) {
        let _e = expect_serializes_to(SetOutput(OutputState::On), "SOUT0\r", any_psu.val);
        let _e = expect_serializes_to(SetOutput(OutputState::Off), "SOUT1\r", any_psu.val);
    }

    test serialize_set_presets_low_voltage(low_voltage_psu) {
        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 1.1,
                    current: 2.2,
                },
                OperatingPoint {
                    voltage: 3.3,
                    current: 4.4,
                },
                OperatingPoint {
                    voltage: 5.5,
                    current: 6.6,
                },
            ),
            "PROM011022033044055066\r",
            low_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 5.0,
                    current: 1.0,
                },
                OperatingPoint {
                    voltage: 12.5,
                    current: 3.0,
                },
                OperatingPoint {
                    voltage: 15.0,
                    current: 0.5,
                },
            ),
            "PROM050010125030150005\r",
            low_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
            ),
            "PROM000000000000000000\r",
            low_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 99.9,
                    current: 99.9,
                },
                OperatingPoint {
                    voltage: 99.9,
                    current: 99.9,
                },
                OperatingPoint {
                    voltage: 99.9,
                    current: 99.9,
                },
            ),
            "PROM999999999999999999\r",
            low_voltage_psu.val,
        );
    }

    test cant_serialize_unrepresentable_set_preset_low_voltage(
        low_voltage_psu,
        invalid_current_low_voltage
    ) {
        let c = invalid_current_low_voltage.val;
        let invalid_p1 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p2 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p3 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
        );
        let _e = expect_cant_serialize(invalid_p1, low_voltage_psu.val);
        let _e = expect_cant_serialize(invalid_p2, low_voltage_psu.val);
        let _e = expect_cant_serialize(invalid_p3, low_voltage_psu.val);
    }

    test serialize_set_presets_high_voltage(high_voltage_psu) {
        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 1.1,
                    current: 0.22,
                },
                OperatingPoint {
                    voltage: 3.3,
                    current: 0.44,
                },
                OperatingPoint {
                    voltage: 5.5,
                    current: 0.66,
                },
            ),
            "PROM011022033044055066\r",
            high_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 5.0,
                    current: 1.0,
                },
                OperatingPoint {
                    voltage: 12.5,
                    current: 3.0,
                },
                OperatingPoint {
                    voltage: 15.0,
                    current: 0.55,
                },
            ),
            "PROM050100125300150055\r",
            high_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
            ),
            "PROM000000000000000000\r",
            high_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 99.9,
                    current: 9.99,
                },
                OperatingPoint {
                    voltage: 99.9,
                    current: 9.99,
                },
                OperatingPoint {
                    voltage: 99.9,
                    current: 9.99,
                },
            ),
            "PROM999999999999999999\r",
            high_voltage_psu.val,
        );
    }

    test cant_serialize_unrepresentable_set_preset_high_voltage(
        high_voltage_psu,
        invalid_current_high_voltage
    ) {
        let c = invalid_current_high_voltage.val;
        let invalid_p1 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p2 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p3 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
        );
        let _e = expect_cant_serialize(invalid_p1, high_voltage_psu.val);
        let _e = expect_cant_serialize(invalid_p2, high_voltage_psu.val);
        let _e = expect_cant_serialize(invalid_p3, high_voltage_psu.val);
    }

    test serialize_select_preset(any_psu) {
        let _e = expect_serializes_to(SelectPreset(PresetIndex::One), "RUNM0\r", any_psu.val);
        let _e = expect_serializes_to(SelectPreset(PresetIndex::Two), "RUNM1\r", any_psu.val);
        let _e = expect_serializes_to(SelectPreset(PresetIndex::Three), "RUNM2\r", any_psu.val);
    }

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

    fn assert_cant_serialize<C: Command>(command: C, psu: &psu::Info) {
        expect_cant_serialize(command, psu).verify();
    }

    fn expect_cant_serialize<C: Command>(command: C, psu: &psu::Info) -> Expectation {
        let mut sink = Cursor::new(Vec::new());

        get_expectation_for!(&command.serialize(&mut sink, psu).is_err(), eq(true))
    }

    fn assert_serializes_to<C: Command>(command: C, result: &str, psu: &psu::Info) {
        expect_serializes_to(command, result, psu).verify();
    }

    fn expect_serializes_to<C: Command>(command: C, result: &str, psu: &psu::Info) -> Expectation {
        let mut sink = Cursor::new(Vec::new());

        command.serialize(&mut sink, psu).unwrap();
        let written = str::from_utf8(sink.get_ref()).unwrap();

        get_expectation_for!(&written, eq(result))
    }

    fixture any_psu(psu: &'static psu::Info) -> &'static psu::Info {
        params {
            vec![
                &BK1685B as &psu::Info,
                &BK1687B as &psu::Info,
                &BK1688B as &psu::Info,
            ].into_iter()
        }
        setup(&mut self) {
            *self.psu
        }
    }

    fixture low_voltage_psu(psu: &'static psu::Info) -> &'static psu::Info {
        params {
            vec![
                &BK1687B as &psu::Info,
                &BK1688B as &psu::Info
            ].into_iter()
        }
        setup(&mut self) {
            *self.psu
        }
    }

    fixture high_voltage_psu() -> &'static psu::Info {
        setup(&mut self) {
            &BK1685B as &psu::Info
        }
    }

    fixture invalid_voltage(voltage: f32) -> f32 {
        params {
            vec![
                -1.,
                100.,
                101.,
                std::f32::NAN,
                std::f32::INFINITY,
                std::f32::NEG_INFINITY,
            ].into_iter()
        }
        setup (&mut self) {
            *self.voltage
        }
    }

    fixture invalid_current_low_voltage(current: f32) -> f32 {
        params {
            vec![
                -1.,
                100.,
                101.,
                std::f32::NAN,
                std::f32::INFINITY,
                std::f32::NEG_INFINITY,
            ].into_iter()
        }
        setup (&mut self) {
            *self.current
        }
    }

    fixture invalid_current_high_voltage(current: f32) -> f32 {
        params {
            vec![
                -1.,
                10.0,
                10.1,
                std::f32::NAN,
                std::f32::INFINITY,
                std::f32::NEG_INFINITY,
            ].into_iter()
        }
        setup (&mut self) {
            *self.current
        }
    }
}

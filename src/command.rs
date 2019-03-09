//! Power supply command set
//!
//! The `Command` enum is used to encode downstream commands to a supply, and
//! suports encoding those command for various supply types.

use crate::psu;

/// Various commands that can be sent to a supply.
///
/// This is currently just a subset, omitting all commands to do with
/// programmable presets.
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Set the supply's output voltage.
    SetVoltage(f32),

    /// Set the maximum acceptable voltage.
    ///
    /// Applies both via the control protocol, and through the front panel.
    SetVoltageLimit(f32),

    /// Set the supply's current limit.
    SetCurrent(f32),

    /// Set the maximum acceptable current limit.
    ///
    /// Applies both via the control protocol, and through the front panel.
    SetCurrentLimit(f32),

    /// Turn the supply's output on or off.
    SetOutput(psu::OutputState),

    /// Configure predefined power supply operating points
    SetPresets(
        psu::OperatingPoint,
        psu::OperatingPoint,
        psu::OperatingPoint,
    ),

    /// Set current and voltage based on a predefined operating point.
    SelectPreset(psu::PresetIndex),

    /// Get the current output voltage and current
    ///
    /// Set through `SetVoltage` and `SetCurrent`
    GetSettings,

    /// Get the current supply status, as displayed on the front panel.
    ///
    /// This consists of:
    ///
    /// - Actual output voltage
    /// - Actual output current
    /// - Output mode (constant current or constant voltage)
    GetStatus,

    /// Get the maximum acceptable supply voltage.
    ///
    /// Set through `SetVoltageLimit`
    GetVoltageLimit,

    /// Get the maximum acceptable supply current.
    ///
    /// Set through `SetCurrentLimit`
    GetCurrentLimit,

    /// Determine the supply's absolute maximum voltage/current limits.
    ///
    /// This is unaffected by the "soft" limits imposed by `SetVoltageLimit`
    /// and `SetCurrentLimit`.
    GetCapabilities,

    /// Get a list of the current presets.
    GetPresets,
}

impl Command {
    /// Get a string representation of a command.
    ///
    /// Trying to serialize a command that's unrepresentable (i.e. has a value
    /// that's negative, or that would have too many digits) in the PSUs' format
    /// will result in a panic. Note that this doesn't have anything to do with
    /// the supply's functional limits -- only the control protocol.
    ///
    /// # Arguments
    ///
    /// - `psu`: Provides information about per-supply serialization quirks
    pub fn serialize(&self, psu: &psu::Info) -> String {
        let mut serialized = self.function().to_owned();

        for arg in self.args(psu) {
            serialized.push_str(&arg.serialize());
        }

        serialized.push_str("\r");

        serialized
    }
}

impl Command {
    /// Get the function-discrimination part of the command.
    ///
    /// Each command starts with a four-character "function."
    fn function(&self) -> &'static str {
        use Command::*;

        match *self {
            SetVoltage(..) => "VOLT",
            SetVoltageLimit(..) => "SOVP",
            SetCurrent(..) => "CURR",
            SetCurrentLimit(..) => "SOCP",
            SetOutput(..) => "SOUT",
            SetPresets(..) => "PROM",
            SelectPreset(..) => "RUNM",
            GetSettings => "GETS",
            GetStatus => "GETD",
            GetVoltageLimit => "GOVP",
            GetCurrentLimit => "GOCP",
            GetCapabilities => "GMAX",
            GetPresets => "GETM",
        }
    }

    /// Get the argument for a command.
    ///
    /// Each supported command has some number of arguments, each consisting of
    /// a variable number (1 or 3) of numeric digits.
    ///
    /// # Arguments
    ///
    /// - `psu`: Provides information about per-supply serialization quirks
    fn args(&self, psu: &psu::Info) -> Vec<Arg> {
        use Command::*;

        match *self {
            SetVoltage(volts) | SetVoltageLimit(volts) => {
                vec![Arg::new(volts, psu.voltage_decimals(), 3)]
            }
            SetCurrent(amps) | SetCurrentLimit(amps) => {
                vec![Arg::new(amps, psu.current_decimals(), 3)]
            }
            SetOutput(ref state) => {
                vec![Arg::new(state.arg_val() as f32, 0, 1)]
            }
            SetPresets(ref p1, ref p2, ref p3) => {
                let mut args = Vec::with_capacity(6);

                for preset in [p1, p2, p3].iter() {
                    args.push(Arg::new(
                        preset.voltage,
                        psu.voltage_decimals(),
                        3,
                    ));
                    args.push(Arg::new(
                        preset.current,
                        psu.current_decimals(),
                        3,
                    ));
                }

                args
            }
            SelectPreset(ref preset) => {
                vec![Arg::new(preset.arg_val() as f32, 0, 1)]
            }
            GetSettings | GetStatus | GetVoltageLimit | GetCurrentLimit
            | GetCapabilities | GetPresets => vec![],
        }
    }
}

/// A command parameter.
struct Arg {
    /// The command value.
    ///
    /// When serialized, this is converted to an integer and rounded. No
    /// further scaling is applied -- that's the responsibility of the
    /// higher-level serializer.
    ///
    /// Must be >= 0, and <= 10^`digits` - 1
    val: f32,

    /// How many decimal places the argument has.
    ///
    /// For instance, if the value 12.3 is represented as `123`, this should
    /// be 1. 0 is a valid value.
    decimals: usize,

    /// How many digits are used to represent this argument.
    digits: usize,
}

impl Arg {
    /// Construct a command argument.
    ///
    /// Panics if `val` is out of range (less than 0, or greater than
    /// 10^`digits` - 1).
    fn new(val: f32, decimals: usize, digits: usize) -> Arg {
        let arg = Arg {
            val,
            decimals,
            digits,
        };

        assert!(arg.is_representable());

        arg
    }

    /// Get a string representation of the argument
    fn serialize(&self) -> String {
        format!(
            "{arg:0width$}",
            arg = self.output_val(),
            width = self.digits
        )
    }

    /// Determine whether the value can fit in the specified number of digits.
    fn is_representable(&self) -> bool {
        self.val.is_finite()
            && self.val >= 0.
            && self.output_val() <= 10u32.pow(self.digits as u32) - 1
    }

    fn output_val(&self) -> u32 {
        (self.val * f32::powi(10., self.decimals as i32)).round() as u32
    }
}

#[cfg(test)]
use galvanic_test::test_suite;

#[cfg(test)]
test_suite! {
    name serialize;

    use super::*;

    use psu::{BK1685B, BK1687B, BK1688B, OperatingPoint, OutputState, PresetIndex};

    use galvanic_assert::assert_that;
    use galvanic_assert::matchers::*;
    use std;

    test serialize_set_voltage(any_psu) {
        let pairs = vec![
            CommandPair { command: Command::SetVoltage(12.3), serialized: "VOLT123\r" },
            CommandPair { command: Command::SetVoltage(0.), serialized: "VOLT000\r" },
            CommandPair { command: Command::SetVoltage(0.1), serialized: "VOLT001\r" },
            CommandPair { command: Command::SetVoltage(99.9), serialized: "VOLT999\r" },
            CommandPair { command: Command::SetVoltage(8.21), serialized: "VOLT082\r" },
            CommandPair { command: Command::SetVoltage(12.99), serialized: "VOLT130\r" },
        ];

        assert_pairs_match(pairs, any_psu.val);
    }

    test cant_serialize_unrepresentable_set_voltage(any_psu, invalid_voltage) {
        let command = Command::SetVoltage(invalid_voltage.val);
        assert_that!(command.serialize(any_psu.val), panics);
    }

    test serialize_set_voltage_limit(any_psu) {
        let pairs = vec![
            CommandPair { command: Command::SetVoltageLimit(12.3), serialized: "SOVP123\r" },
            CommandPair { command: Command::SetVoltageLimit(0.), serialized: "SOVP000\r" },
            CommandPair { command: Command::SetVoltageLimit(0.1), serialized: "SOVP001\r" },
            CommandPair { command: Command::SetVoltageLimit(99.9), serialized: "SOVP999\r" },
            CommandPair { command: Command::SetVoltageLimit(8.21), serialized: "SOVP082\r" },
            CommandPair { command: Command::SetVoltageLimit(12.99), serialized: "SOVP130\r" },
        ];

        assert_pairs_match(pairs, any_psu.val);
    }

    test cant_serialize_unrepresentable_set_voltage_limit(any_psu, invalid_voltage) {
        let command = Command::SetVoltageLimit(invalid_voltage.val);
        assert_that!(command.serialize(any_psu.val), panics);
    }

    test serialize_set_current_low_voltage(low_voltage_psu) {
        let pairs = vec![
            CommandPair { command: Command::SetCurrent(0.), serialized: "CURR000\r" },
            CommandPair { command: Command::SetCurrent(0.5), serialized: "CURR005\r" },
            CommandPair { command: Command::SetCurrent(1.5), serialized: "CURR015\r" },
            CommandPair { command: Command::SetCurrent(1.5), serialized: "CURR015\r" },
            CommandPair { command: Command::SetCurrent(12.3), serialized: "CURR123\r" },
            CommandPair { command: Command::SetCurrent(99.9), serialized: "CURR999\r" },
            CommandPair { command: Command::SetCurrent(8.21), serialized: "CURR082\r" },
            CommandPair { command: Command::SetCurrent(12.99), serialized: "CURR130\r" },
        ];

        assert_pairs_match(pairs, low_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_current_low_voltage(
        low_voltage_psu,
        invalid_current_low_voltage
    ) {
        let command = Command::SetCurrent(invalid_current_low_voltage.val);
        assert_that!(command.serialize(low_voltage_psu.val), panics);
    }

    test serialize_set_current_high_voltage(high_voltage_psu) {
        let pairs = vec![
            CommandPair { command: Command::SetCurrent(0.), serialized: "CURR000\r" },
            CommandPair { command: Command::SetCurrent(0.5), serialized: "CURR050\r" },
            CommandPair { command: Command::SetCurrent(1.55), serialized: "CURR155\r" },
            CommandPair { command: Command::SetCurrent(2.31), serialized: "CURR231\r" },
            CommandPair { command: Command::SetCurrent(9.99), serialized: "CURR999\r" },
            CommandPair { command: Command::SetCurrent(0.821), serialized: "CURR082\r" },
            CommandPair { command: Command::SetCurrent(1.299), serialized: "CURR130\r" },
        ];

        assert_pairs_match(pairs, high_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_current_high_voltage(
        high_voltage_psu,
        invalid_current_high_voltage
    ) {
        let command = Command::SetCurrent(invalid_current_high_voltage.val);
        assert_that!(command.serialize(high_voltage_psu.val), panics);
    }

    test serialize_set_current_limit_low_voltage(low_voltage_psu) {
        let pairs = vec![
            CommandPair { command: Command::SetCurrentLimit(0.), serialized: "SOCP000\r" },
            CommandPair { command: Command::SetCurrentLimit(0.5), serialized: "SOCP005\r" },
            CommandPair { command: Command::SetCurrentLimit(1.5), serialized: "SOCP015\r" },
            CommandPair { command: Command::SetCurrentLimit(1.5), serialized: "SOCP015\r" },
            CommandPair { command: Command::SetCurrentLimit(12.3), serialized: "SOCP123\r" },
            CommandPair { command: Command::SetCurrentLimit(99.9), serialized: "SOCP999\r" },
            CommandPair { command: Command::SetCurrentLimit(8.21), serialized: "SOCP082\r" },
            CommandPair { command: Command::SetCurrentLimit(12.99), serialized: "SOCP130\r" },
        ];

        assert_pairs_match(pairs, low_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_current_limit_low_voltage(
        low_voltage_psu,
        invalid_current_low_voltage
    ) {
        let command = Command::SetCurrentLimit(invalid_current_low_voltage.val);
        assert_that!(command.serialize(low_voltage_psu.val), panics);
    }

    test serialize_set_current_limit_high_voltage(high_voltage_psu) {
        let pairs = vec![
            CommandPair { command: Command::SetCurrentLimit(0.), serialized: "SOCP000\r" },
            CommandPair { command: Command::SetCurrentLimit(0.5), serialized: "SOCP050\r" },
            CommandPair { command: Command::SetCurrentLimit(1.55), serialized: "SOCP155\r" },
            CommandPair { command: Command::SetCurrentLimit(2.31), serialized: "SOCP231\r" },
            CommandPair { command: Command::SetCurrentLimit(9.99), serialized: "SOCP999\r" },
            CommandPair { command: Command::SetCurrentLimit(0.821), serialized: "SOCP082\r" },
            CommandPair { command: Command::SetCurrentLimit(1.299), serialized: "SOCP130\r" },
        ];

        assert_pairs_match(pairs, high_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_current_limit_high_voltage(
        high_voltage_psu,
        invalid_current_high_voltage
    ) {
        let command = Command::SetCurrentLimit(invalid_current_high_voltage.val);
        assert_that!(command.serialize(high_voltage_psu.val), panics);
    }

    test serialize_set_output(any_psu) {
        let pairs = vec![
            CommandPair { command: Command::SetOutput(OutputState::On), serialized: "SOUT0\r" },
            CommandPair { command: Command::SetOutput(OutputState::Off), serialized: "SOUT1\r" },
        ];

        assert_pairs_match(pairs, any_psu.val);
    }

    test serialize_set_presets_low_voltage(low_voltage_psu) {
        let pairs = vec![
            CommandPair {
                command: Command::SetPresets(
                    OperatingPoint { voltage: 1.1, current: 2.2 },
                    OperatingPoint { voltage: 3.3, current: 4.4 },
                    OperatingPoint { voltage: 5.5, current: 6.6 },
                ),
                serialized: "PROM011022033044055066\r",
            },
            CommandPair {
                command: Command::SetPresets(
                    OperatingPoint { voltage: 5.0, current: 1.0 },
                    OperatingPoint { voltage: 12.5, current: 3.0 },
                    OperatingPoint { voltage: 15.0, current: 0.5 },
                ),
                serialized: "PROM050010125030150005\r",
            },
            CommandPair {
                command: Command::SetPresets(
                    OperatingPoint { voltage: 0., current: 0. },
                    OperatingPoint { voltage: 0., current: 0. },
                    OperatingPoint { voltage: 0., current: 0. },
                ),
                serialized: "PROM000000000000000000\r",
            },
            CommandPair {
                command: Command::SetPresets(
                    OperatingPoint { voltage: 99.9, current: 99.9 },
                    OperatingPoint { voltage: 99.9, current: 99.9 },
                    OperatingPoint { voltage: 99.9, current: 99.9 },
                ),
                serialized: "PROM999999999999999999\r",
            },
        ];

        assert_pairs_match(pairs, low_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_preset_low_voltage(
        low_voltage_psu,
        invalid_current_low_voltage
    ) {
        let c = invalid_current_low_voltage.val;
        let commands = vec![
            Command::SetPresets(
                OperatingPoint { voltage: 0., current: c },
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: 0. },
            ),
            Command::SetPresets(
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: c },
                OperatingPoint { voltage: 0., current: 0. },
            ),
            Command::SetPresets(
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: c },
            ),
        ];

        for command in commands {
            assert_that!(command.serialize(low_voltage_psu.val), panics);
        }
    }

    test serialize_set_presets_high_voltage(high_voltage_psu) {
        let pairs = vec![
            CommandPair {
                command: Command::SetPresets(
                    OperatingPoint { voltage: 1.1, current: 0.22 },
                    OperatingPoint { voltage: 3.3, current: 0.44 },
                    OperatingPoint { voltage: 5.5, current: 0.66 },
                ),
                serialized: "PROM011022033044055066\r",
            },
            CommandPair {
                command: Command::SetPresets(
                    OperatingPoint { voltage: 5.0, current: 1.0 },
                    OperatingPoint { voltage: 12.5, current: 3.0 },
                    OperatingPoint { voltage: 15.0, current: 0.55 },
                ),
                serialized: "PROM050100125300150055\r",
            },
            CommandPair {
                command: Command::SetPresets(
                    OperatingPoint { voltage: 0., current: 0. },
                    OperatingPoint { voltage: 0., current: 0. },
                    OperatingPoint { voltage: 0., current: 0. },
                ),
                serialized: "PROM000000000000000000\r",
            },
            CommandPair {
                command: Command::SetPresets(
                    OperatingPoint { voltage: 99.9, current: 9.99 },
                    OperatingPoint { voltage: 99.9, current: 9.99 },
                    OperatingPoint { voltage: 99.9, current: 9.99 },
                ),
                serialized: "PROM999999999999999999\r",
            },
        ];

        assert_pairs_match(pairs, high_voltage_psu.val);
    }

    test cant_serialize_unrepresentable_set_preset_high_voltage(
        high_voltage_psu,
        invalid_current_high_voltage
    ) {
        let c = invalid_current_high_voltage.val;
        let commands = vec![
            Command::SetPresets(
                OperatingPoint { voltage: 0., current: c },
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: 0. },
            ),
            Command::SetPresets(
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: c },
                OperatingPoint { voltage: 0., current: 0. },
            ),
            Command::SetPresets(
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: c },
            ),
        ];

        for command in commands {
            assert_that!(command.serialize(high_voltage_psu.val), panics);
        }
    }

    test cant_serialize_unrepresentable_set_preset_voltage(any_psu, invalid_voltage) {
        let v = invalid_voltage.val;
        let commands = vec![
            Command::SetPresets(
                OperatingPoint { voltage: v, current: 0. },
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: 0. },
            ),
            Command::SetPresets(
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: v, current: 0. },
                OperatingPoint { voltage: 0., current: 0. },
            ),
            Command::SetPresets(
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: 0., current: 0. },
                OperatingPoint { voltage: v, current: 0. },
            ),
        ];

        for command in commands {
            assert_that!(command.serialize(any_psu.val), panics);
        }
    }

    test serialize_select_preset(any_psu) {
        let pairs = vec![
            CommandPair { command: Command::SelectPreset(PresetIndex::One), serialized: "RUNM0\r" },
            CommandPair { command: Command::SelectPreset(PresetIndex::Two), serialized: "RUNM1\r" },
            CommandPair { command: Command::SelectPreset(PresetIndex::Three), serialized: "RUNM2\r" },
        ];

        assert_pairs_match(pairs, any_psu.val);
    }

    test serialize_get_settings(any_psu) {
        let command = Command::GetSettings;
        assert_that!(&command.serialize(any_psu.val), eq("GETS\r".to_owned()));
    }

    test serialize_get_status(any_psu) {
        let command = Command::GetStatus;
        assert_that!(&command.serialize(any_psu.val), eq("GETD\r".to_owned()));
    }

    test serialize_get_voltage_limit(any_psu) {
        let command = Command::GetVoltageLimit;
        assert_that!(&command.serialize(any_psu.val), eq("GOVP\r".to_owned()));
    }

    test serialize_get_current_limit(any_psu) {
        let command = Command::GetCurrentLimit;
        assert_that!(&command.serialize(any_psu.val), eq("GOCP\r".to_owned()));
    }

    test serialize_get_capabilities(any_psu) {
        let command = Command::GetCapabilities;
        assert_that!(&command.serialize(any_psu.val), eq("GMAX\r".to_owned()));
    }

    test serialize_get_presets(any_psu) {
        let command = Command::GetPresets;
        assert_that!(&command.serialize(any_psu.val), eq("GETM\r".to_owned()));
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

    struct CommandPair {
        command: Command,
        serialized: &'static str,
    }

    fn assert_pairs_match(pairs: Vec<CommandPair>, psu: &psu::Info) {
        for pair in pairs {
            assert_that!(&pair.command.serialize(psu), eq(pair.serialized.to_owned()));
        }
    }
}

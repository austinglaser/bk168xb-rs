//! Power supply command set

use crate::psu;
use std::io;

mod error;
mod get;
mod select_preset;
mod set_current;
mod set_current_limit;
mod set_output;
mod set_presets;
mod set_voltage;
mod set_voltage_limit;

pub use error::*;
pub use get::{
    GetCapabilities, GetCurrentLimit, GetPresets, GetSettings, GetStatus,
    GetVoltageLimit,
};
pub use select_preset::SelectPreset;
pub use set_current::SetCurrent;
pub use set_current_limit::SetCurrentLimit;
pub use set_output::SetOutput;
pub use set_presets::SetPresets;
pub use set_voltage::SetVoltage;
pub use set_voltage_limit::SetVoltageLimit;

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
    ) -> Result<()> {
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
    fn serialize<S: io::Write>(&self, sink: S, psu: &psu::Info) -> Result<()>;
}

impl<C> Serialize for C
where
    C: Command,
{
    fn serialize<S: io::Write>(
        &self,
        mut sink: S,
        psu: &psu::Info,
    ) -> Result<()> {
        write!(&mut sink, "{}", C::FUNCTION)?;
        self.serialize_args(&mut sink, psu)?;
        write!(&mut sink, "\r")?;

        Ok(())
    }
}

struct ArgFormat {
    pub decimals: usize,
    pub digits: usize,
}

impl ArgFormat {
    fn serialize_arg<S: io::Write>(&self, mut sink: S, val: f32) -> Result<()> {
        if let Some(value) = self.output_val(val) {
            write!(
                &mut sink,
                "{arg:0width$}",
                arg = value,
                width = self.digits,
            )?;

            Ok(())
        } else {
            Err(Error::ValueUnrepresentable(val))
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
pub mod test_util {
    use super::*;

    use crate::psu;

    use galvanic_assert::{
        get_expectation_for, matchers::*, Expectation, MatchResult,
        MatchResultBuilder,
    };

    use std::io::Cursor;
    use std::str;

    pub fn assert_cant_serialize<C: Command>(command: C, psu: &psu::Info) {
        expect_cant_serialize(command, psu).verify();
    }

    pub fn expect_cant_serialize<C: Command>(
        command: C,
        psu: &psu::Info,
    ) -> Expectation {
        let mut sink = Cursor::new(Vec::new());

        get_expectation_for!(
            &command.serialize(&mut sink, psu),
            is_unrepresentable_val_error
        )
    }

    fn is_unrepresentable_val_error<T>(res: &Result<T>) -> MatchResult {
        let builder =
            MatchResultBuilder::for_("is unrepresentable value error");

        if let Err(ref e) = *res {
            if let Error::ValueUnrepresentable(_) = *e {
                builder.matched()
            } else {
                builder.failed_because("wrong type of error")
            }
        } else {
            builder.failed_because("not an error")
        }
    }

    pub fn assert_serializes_to<C: Command>(
        command: C,
        result: &str,
        psu: &psu::Info,
    ) {
        expect_serializes_to(command, result, psu).verify();
    }

    pub fn expect_serializes_to<C: Command>(
        command: C,
        result: &str,
        psu: &psu::Info,
    ) -> Expectation {
        let mut sink = Cursor::new(Vec::new());

        command.serialize(&mut sink, psu).unwrap();
        let written = str::from_utf8(sink.get_ref()).unwrap();

        get_expectation_for!(&written, eq(result))
    }
}

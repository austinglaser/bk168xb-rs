//! Power supply command set

use crate::psu;
use std::io;

mod get;
mod select_preset;
mod set_current;
mod set_current_limit;
mod set_output;
mod set_presets;
mod set_voltage;
mod set_voltage_limit;

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
pub mod test_util {
    use super::*;

    use crate::psu;

    use galvanic_assert::{get_expectation_for, matchers::*, Expectation};

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
            &command.serialize(&mut sink, psu).is_err(),
            eq(true)
        )
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

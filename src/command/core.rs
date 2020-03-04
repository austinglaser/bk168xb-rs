use crate::{
    command::{CommandError, Result},
    psu,
};

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

pub(crate) struct ArgFormat {
    pub decimals: usize,
    pub digits: usize,
}

impl ArgFormat {
    pub(crate) fn serialize_arg<S: io::Write>(
        &self,
        mut sink: S,
        val: f32,
    ) -> Result<()> {
        use CommandError::ValueUnrepresentable;

        let value = self.output_val(val).ok_or(ValueUnrepresentable(val))?;
        write!(&mut sink, "{arg:0width$}", arg = value, width = self.digits)?;

        Ok(())
    }

    pub(crate) fn output_val(&self, val: f32) -> Option<u32> {
        let multiplier = f32::powi(10., self.decimals as i32);
        let max = (f32::powi(10., self.digits as i32) - 1.) / multiplier;

        if !val.is_finite() || val < 0. || val > max {
            return None;
        }

        let output_val = (val * multiplier).round() as u32;

        Some(output_val)
    }
}

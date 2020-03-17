use crate::{command::Result, SupplyVariant};

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
    /// - `variant`: Provides information about per-supply serialization quirks
    /// - `sink`: Where to write arguments
    fn serialize_args<S: io::Write>(
        &self,
        sink: &mut S,
        variant: &SupplyVariant,
    ) -> Result<()> {
        let _ = (sink, variant);

        Ok(())
    }
}

/// A target for command serialization.
pub(crate) trait CommandSink {
    /// Write the command to a sink.
    ///
    /// # Arguments
    ///
    /// - `command`: Command to send
    /// - `variant`: Provides information about per-supply serialization quirks
    fn send_command<C: Command>(
        &mut self,
        command: &C,
        variant: &SupplyVariant,
    ) -> Result<()>;
}

impl<S: io::Write> CommandSink for S {
    fn send_command<C: Command>(
        &mut self,
        command: &C,
        variant: &SupplyVariant,
    ) -> Result<()> {
        write!(self, "{}", C::FUNCTION)?;
        command.serialize_args(self, variant)?;
        write!(self, "\r")?;

        Ok(())
    }
}

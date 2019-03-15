//! Error handling for BK commands
use err_derive::Error;

use std::io;

/// Errors that can arise from `Command` functions.
#[derive(Debug, Error)]
pub enum CommandError {
    /// The command contained a value which is invalid for its format.
    #[error(display = "unrepresentable value in command: {}", _0)]
    ValueUnrepresentable(f32),

    /// The sink returned an error while writing the command.
    #[error(display = "failed to write command")]
    WriteFailure(#[error(cause)] io::Error),
}

impl From<io::Error> for CommandError {
    fn from(io: io::Error) -> Self {
        CommandError::WriteFailure(io)
    }
}

/// A specialeized `Result` type for `Command` operations.
pub type Result<T> = std::result::Result<T, CommandError>;

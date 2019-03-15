///! Error handling for BK commands
use std::error;
use std::fmt;
use std::io;

/// Errors that can arise from `Command` functions.
#[derive(Debug)]
pub enum CommandError {
    /// The command contained a value which is invalid for its format.
    ValueUnrepresentable(f32),

    /// The sink returned an error while writing the command.
    WriteFailure(io::Error),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CommandError::*;

        match *self {
            ValueUnrepresentable(v) => {
                write!(f, "unrepresentable value: {}", v)
            }
            WriteFailure(_) => write!(f, "could not serialize command"),
        }
    }
}

impl error::Error for CommandError {
    fn description(&self) -> &str {
        use CommandError::*;

        match *self {
            ValueUnrepresentable(_) => "unrepresentable value",
            WriteFailure(_) => "serialization falure",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use CommandError::*;

        match *self {
            ValueUnrepresentable(_) => None,
            WriteFailure(ref io) => Some(io),
        }
    }
}

impl From<io::Error> for CommandError {
    fn from(io: io::Error) -> Self {
        CommandError::WriteFailure(io)
    }
}

/// A specialeized `Result` type for `Command` operations.
pub type Result<T> = std::result::Result<T, CommandError>;

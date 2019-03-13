///! Error handling for BK commands

use std::error;
use std::fmt;
use std::io;

/// Errors that can arise from `Command` functions.
#[derive(Debug)]
pub enum Error {
    /// The command contained a value which is invalid for its format.
    ValueUnrepresentable(f32),

    /// The sink returned an error while writing the command.
    SerializationFailure(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            ValueUnrepresentable(v) => {
                write!(f, "unrepresentable value: {}", v)
            }
            SerializationFailure(_) => write!(f, "could not serialize command"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use Error::*;

        match *self {
            ValueUnrepresentable(_) => "unrepresentable value",
            SerializationFailure(_) => "serialization falure",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use Error::*;

        match *self {
            ValueUnrepresentable(_) => None,
            SerializationFailure(ref io) => Some(io),
        }
    }
}

impl From<io::Error> for Error {
    fn from(io: io::Error) -> Self {
        Error::SerializationFailure(io)
    }
}

/// A specialeized `Result` type for `Command` operations.
pub type Result<T> = std::result::Result<T, Error>;

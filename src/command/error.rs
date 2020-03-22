//! Error handling for BK commands

use std::io;

/// Errors that can arise from `Command` functions.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The command contained a value which is invalid for its format.
    #[error("unrepresentable value in command: {0}")]
    ValueUnrepresentable(f32),

    /// The sink returned an error while writing the command.
    #[error("failed to write command")]
    WriteFailure(#[from] io::Error),
}

/// A specialized `Result` type for `Command` operations.
pub type Result<T> = std::result::Result<T, Error>;

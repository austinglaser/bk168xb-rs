//! Errors that can arise from parsing BK responses

use std::io;

/// Errors that can arise from `Response` functions.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The PSU returned data, but it didn't match the expected format.
    #[error("malformed command response")]
    MalformedResponse,

    /// The PSU returned no data.
    ///
    /// Note: this is also returned if a timeout error occurred.
    #[error("no command response")]
    NoResponse,

    /// The source returned an error when reading data.
    #[error("failed to read response")]
    ReadFailure(#[from] io::Error),
}

/// A specialized `Result` type for `Response` operations.
pub type Result<T> = std::result::Result<T, Error>;

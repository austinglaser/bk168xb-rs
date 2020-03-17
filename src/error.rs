//! Error handling for power supply interface

use thiserror;

use crate::{command, response};

/// Errors that can arise interfacing with BK168xB supplies.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The command could not be written to the device.
    #[error("could not send command: {0}")]
    WritingCommand(#[from] command::Error),

    /// There was a problem getting a response back from the device.
    #[error("error receiving response: {0}")]
    ReadingResponse(#[from] response::Error),
}

/// A specialized `Result` type for supply methods.
pub type Result<T> = std::result::Result<T, Error>;

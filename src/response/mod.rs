//! Type-safe BK168xB response parsing

use self::error::{Error, Result};
use crate::psu;

use std::io;

pub trait Response {
}

/// A response indicating success, but carrying no data.
pub struct Ack;

#[cfg(test)]
use galvanic_test::test_suite;

#[cfg(test)]
test_suite! {
    name test;
}

pub mod error {
    use std::error;
    use std::fmt;
    use std::io;

    /// Errors that can arise from `Response` functions.
    #[derive(Debug)]
    pub enum Error {
        MalformedResponse,
        NoResponse,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use Error::*;

            write!(f, "{}", (self as &error::Error).description())
        }
    }

    impl error::Error for Error {
        fn description(&self) -> &str {
            use Error::*;

            match *self {
                MalformedResponse => "malformed command response",
                NoResponse => "no command response",
            }
        }
    }

    pub type Result<T> = std::result::Result<T, Error>;
}

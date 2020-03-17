//! Rust bindings for BK precision 168xB supplies
//!
//! This crate aims to provide functional bindings for this family of supplies,
//! including:
//!
//! - BK1685B (60V/5A)
//! - BK1687B (36V/10A)
//! - BK1688B (18V/20A)

#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod core;
#[cfg(test)]
pub(crate) mod test_util;

pub mod command;
pub mod response;

pub use crate::core::*;

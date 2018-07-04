//! Rust bindings for BK precision 168xB supplies
//!
//! This crate aims to provide functional bindings for this family of supplies,
//! including:
//!
//! - BK1685B (60V/5A)
//! - BK1687B (36V/10A)
//! - BK1688B (18V/20A)

#[warn(missing_docs)]
#[cfg(test)]
#[macro_use]
extern crate galvanic_test;

#[cfg(test)]
#[macro_use]
extern crate galvanic_assert;

pub mod command;
pub mod psu;

pub use command::Command;

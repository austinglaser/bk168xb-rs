//! Power supply information
//!
//! # Constant definitions
//!
//! This module exports constants and common types relating to power supply
//! state, control points, etc.
//!
//! # Protocol Quirks
//!
//! There are various pieces of data which vary between supported power
//! supplies. These include the precision of certain commands, nominal
//! current/voltage limits, etc.
//!
//! All that information is encompassed in the information structs here.

use std::{fmt::Debug, panic::RefUnwindSafe};

/// Output state of the supply.
#[derive(Debug)]
pub enum OutputState {
    /// The supply is actively providing power.
    On,

    /// The supply is not providing power.
    Off,
}

/// Used to select a single preset
#[derive(Debug)]
pub enum PresetIndex {
    /// First preset
    Preset1,

    /// Second preset
    Preset2,

    /// Third preset
    Preset3,
}

/// A power-supply operating point
#[derive(Debug)]
pub struct OperatingPoint {
    /// Voltage setpoint.
    pub voltage: f32,

    /// Current limit.
    pub current: f32,
}

/// Power supply information for the 1685B (60V / 5A) model
pub const BK1685B: Bk1685b = Bk1685b {};

/// Power supply information for the 1687B (36V / 10A) model
pub const BK1687B: Bk1687b = Bk1687b {};

/// Power supply information for the 1687B (18V / 20A) model
pub const BK1688B: Bk1688b = Bk1688b {};

/// Info type for BK1685B supplies.
///
/// Avoid constructing values of this type; instead, use the appropriate
/// constant (`BK1685B`).
#[derive(Debug)]
pub struct Bk1685b {}

/// Info type for BK1687B supplies
///
/// Avoid constructing values of this type; instead, use the appropriate
/// constant (`BK1687B`).
#[derive(Debug)]
pub struct Bk1687b {}

/// Info type for BK1689B supplies
///
/// Avoid constructing values of this type; instead, use the appropriate
/// constant (`BK1688B`).
#[derive(Debug)]
pub struct Bk1688b {}

/// Interface for types providing power supply information.
///
/// Do not implement this trait! Use one of the constants for which it is
/// pre-implemented.
pub unsafe trait Info: Debug + RefUnwindSafe {
    /// Decimal places used in current arguments
    fn current_decimals(&self) -> usize;

    /// Decimal places used in voltage arguments
    fn voltage_decimals(&self) -> usize;
}

unsafe impl Info for Bk1685b {
    fn current_decimals(&self) -> usize {
        2
    }

    fn voltage_decimals(&self) -> usize {
        1
    }
}

unsafe impl Info for Bk1687b {
    fn current_decimals(&self) -> usize {
        1
    }

    fn voltage_decimals(&self) -> usize {
        1
    }
}

unsafe impl Info for Bk1688b {
    fn current_decimals(&self) -> usize {
        1
    }

    fn voltage_decimals(&self) -> usize {
        1
    }
}

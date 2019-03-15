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

#[cfg(test)]
pub mod test_util;

/// Output state of the supply.
#[derive(Debug, PartialEq)]
pub enum OutputState {
    /// The supply is actively providing power.
    On,

    /// The supply is not providing power.
    Off,
}

impl OutputState {
    /// Get a value appropriate for using in a command.
    ///
    /// N.B.: This field has inverted logic in commands -- it uses 0 for on, and
    /// 1 for off.
    pub(crate) fn arg_val(&self) -> usize {
        match *self {
            OutputState::On => 0,
            OutputState::Off => 1,
        }
    }
}

/// Used to select a single preset
#[derive(Debug, PartialEq)]
pub enum PresetIndex {
    /// First preset
    One,

    /// Second preset
    Two,

    /// Third preset
    Three,
}

impl PresetIndex {
    /// Get a concrete index integer for this preset.
    ///
    /// Appropriate for use in commands, or for indexing preset arrays.
    pub(crate) fn arg_val(&self) -> usize {
        match *self {
            PresetIndex::One => 0,
            PresetIndex::Two => 1,
            PresetIndex::Three => 2,
        }
    }
}

/// A power-supply operating point
#[derive(Debug, PartialEq)]
pub struct OperatingPoint {
    /// Voltage setpoint.
    pub voltage: f32,

    /// Current limit.
    pub current: f32,
}

/// Information of model-to-model supply variations.
#[derive(Debug, PartialEq, Eq)]
pub struct Info {
    /// The number of decimal places in commands encoding current.
    current_decimals: usize,

    /// The number of decimal places in commands encoding voltage.
    voltage_decimals: usize,
}

impl Info {
    /// Get the number of decimal places in commands encoding current.
    pub fn current_decimals(&self) -> usize {
        self.current_decimals
    }

    /// Get the number of decimal places in commands encoding voltage.
    pub fn voltage_decimals(&self) -> usize {
        self.voltage_decimals
    }
}

/// Power supply information for the 1685B (60V / 5A) model
pub const BK1685B: Info = Info {
    current_decimals: 2,
    voltage_decimals: 1,
};

/// Power supply information for the 1687B (36V / 10A) model
pub const BK1687B: Info = Info {
    current_decimals: 1,
    voltage_decimals: 1,
};

/// Power supply information for the 1687B (18V / 20A) model
pub const BK1688B: Info = Info {
    current_decimals: 1,
    voltage_decimals: 1,
};
use crate::{command, response};

use std::{io, str};

/// Output state of the supply.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    pub(crate) fn arg_val(self) -> usize {
        match self {
            OutputState::On => 0,
            OutputState::Off => 1,
        }
    }
}

/// A supply's output mode.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum OutputMode {
    /// Constant voltage mode.
    ///
    /// The supply is regulating its output current to maintain the configured
    /// voltage.
    ConstantVoltage,

    /// Constant current mode.
    ///
    /// The supply is regulating its output voltage to maintain the configured
    /// current.
    ConstantCurrent,
}

/// Used to select a single preset
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    pub(crate) fn arg_val(self) -> usize {
        match self {
            PresetIndex::One => 0,
            PresetIndex::Two => 1,
            PresetIndex::Three => 2,
        }
    }
}

/// A power-supply operating point
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OperatingPoint {
    /// Voltage setpoint.
    pub voltage: f32,

    /// Current limit.
    pub current: f32,
}

/// Information of model-to-model supply variations.
///
/// This type cannot be constructed outside this crate; instead, use one of the
/// pre-defined static instances for the supported PSU types:
///
/// - [`BK1685B`](crate::BK1685B)
/// - [`BK1687B`](crate::BK1687B)
/// - [`BK1688B`](crate::BK1688B)
///
/// # Cannot be constructed publicly
///
/// ```compile_fail
/// let _ = bk168xb::SupplyVariant { current_decimals: 3, voltage_decimals: 0 };
/// ```
///
/// # Cannot be copied and modified
///
/// ```compile_fail
/// let mut info = *bk168xb::BK1688B;
/// info.current_decimals = 3;
/// ```
#[derive(Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SupplyVariant {
    /// The supply's model number
    pub model: &'static str,

    /// The stated maximum voltage.
    ///
    /// Generally, a supply's reported capabilities are slightly above this.
    pub nominal_max_voltage: usize,

    /// The stated maximum current.
    ///
    /// Generally, a supply's reported capabilities are slightly above this.
    pub nominal_max_current: usize,

    /// The number of decimal places in commands encoding current.
    pub current_decimals: usize,

    /// The number of decimal places in commands encoding voltage.
    pub voltage_decimals: usize,
}

/// Power supply information for the 1685B (60V / 5A) model
pub const BK1685B: &SupplyVariant = &BK1685B_INST;

/// Power supply information for the 1687B (36V / 10A) model
pub const BK1687B: &SupplyVariant = &BK1687B_INST;

/// Power supply information for the 1688B (18V / 20A) model
pub const BK1688B: &SupplyVariant = &BK1688B_INST;

const BK1685B_INST: SupplyVariant = SupplyVariant {
    model: "BK1685B",
    nominal_max_voltage: 60,
    nominal_max_current: 5,
    current_decimals: 2,
    voltage_decimals: 1,
};

const BK1687B_INST: SupplyVariant = SupplyVariant {
    model: "BK1687B",
    nominal_max_voltage: 36,
    nominal_max_current: 10,
    current_decimals: 1,
    voltage_decimals: 1,
};

const BK1688B_INST: SupplyVariant = SupplyVariant {
    model: "BK1688B",
    nominal_max_voltage: 18,
    nominal_max_current: 20,
    current_decimals: 1,
    voltage_decimals: 1,
};

pub(crate) fn variant_for_max_voltage(
    voltage: f32,
) -> Option<&'static SupplyVariant> {
    for supply in &[BK1685B, BK1687B, BK1688B] {
        let nom_volt = supply.nominal_max_voltage as f32;

        // This band is pretty arbitrary. It's intentionally lax (I don't think
        // I've seen a supply ever report more than a couple volts higher than
        // its nominal value).
        let top_volt = nom_volt + 10.;
        if voltage >= nom_volt && voltage < top_volt {
            return Some(supply);
        }
    }

    None
}

pub(crate) struct ArgFormat {
    pub decimals: usize,
    pub digits: usize,
}

impl ArgFormat {
    pub(crate) fn serialize_arg<S: io::Write>(
        &self,
        sink: &mut S,
        val: f32,
    ) -> command::Result<()> {
        use command::Error::ValueUnrepresentable;

        let value = self.output_val(val).ok_or(ValueUnrepresentable(val))?;
        write!(sink, "{arg:0width$}", arg = value, width = self.digits)?;

        Ok(())
    }

    pub(crate) fn parse(&self, raw: &[u8]) -> response::Result<f32> {
        use response::Error::MalformedResponse;

        if raw.len() != self.digits {
            return Err(MalformedResponse);
        }

        let as_str = str::from_utf8(raw).map_err(|_| MalformedResponse)?;
        let as_int =
            usize::from_str_radix(as_str, 10).map_err(|_| MalformedResponse)?;
        let val = as_int as f32 / self.factor();

        Ok(val)
    }

    fn output_val(&self, val: f32) -> Option<u32> {
        if !val.is_finite() || val < 0. || val > self.max() {
            return None;
        }

        let output_val = (val * self.factor()).round() as u32;

        Some(output_val)
    }

    fn factor(&self) -> f32 {
        f32::powi(10., self.decimals as i32)
    }

    fn max(&self) -> f32 {
        (f32::powi(10., self.digits as i32) - 1.) / self.factor()
    }
}

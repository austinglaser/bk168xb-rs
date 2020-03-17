//! Command for selecting an operating point from memory.

use crate::{
    command::{self, Command},
    ArgFormat, PresetIndex, SupplyVariant,
};

use std::io;

/// Select a preset previously set with `SetPresets`
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct SelectPreset(pub PresetIndex);

impl Command for SelectPreset {
    const FUNCTION: &'static str = "RUNM";

    fn serialize_args<S: io::Write>(
        &self,
        sink: &mut S,
        _variant: &SupplyVariant,
    ) -> command::Result<()> {
        let fmt = ArgFormat {
            decimals: 0,
            digits: 1,
        };

        fmt.serialize_arg(sink, self.0.arg_val() as f32)
    }
}

#[cfg(test)]
use galvanic_test::test_suite;

#[cfg(test)]
test_suite! {
    name test;

    use super::*;

    use crate::command::test_util::expect_serializes_to;
    use crate::PresetIndex;
    use crate::test_util::any_psu;

    test serialize_select_preset(any_psu) {
        let _e = expect_serializes_to(
            SelectPreset(PresetIndex::One),
            "RUNM0\r",
            any_psu.val,
        );
        let _e = expect_serializes_to(
            SelectPreset(PresetIndex::Two),
            "RUNM1\r",
            any_psu.val,
        );
        let _e = expect_serializes_to(
            SelectPreset(PresetIndex::Three),
            "RUNM2\r",
            any_psu.val,
        );
    }
}

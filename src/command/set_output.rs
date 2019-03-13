//! Command for controlling supply output state.

use crate::command::{ArgFormat, Command};
use crate::psu;

use std::io;

/// Control whether the supply is supplying power.
pub struct SetOutput(psu::OutputState);

impl Command for SetOutput {
    const FUNCTION: &'static str = "SOUT";

    fn serialize_args<S: io::Write>(
        &self,
        mut sink: S,
        _psu: &psu::Info,
    ) -> io::Result<()> {
        let fmt = ArgFormat {
            decimals: 0,
            digits: 1,
        };

        fmt.serialize_arg(&mut sink, self.0.arg_val() as f32)
    }
}

#[cfg(test)]
use galvanic_test::test_suite;

#[cfg(test)]
test_suite! {
    name test;

    use super::*;

    use crate::command::test_util::expect_serializes_to;
    use crate::psu::OutputState;
    use crate::psu::test::any_psu;

    test can_serialize(any_psu) {
        let _e = expect_serializes_to(
            SetOutput(OutputState::On),
            "SOUT0\r",
            any_psu.val,
        );
        let _e = expect_serializes_to(
            SetOutput(OutputState::Off),
            "SOUT1\r",
            any_psu.val,
        );
    }
}
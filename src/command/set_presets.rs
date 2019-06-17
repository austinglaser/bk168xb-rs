//! Command for setting a predefined set of operating points.

use crate::command::{self, ArgFormat, Command};
use crate::psu;

use std::io;

/// Configure the supply's pre-set operating points.
pub struct SetPresets(
    psu::OperatingPoint,
    psu::OperatingPoint,
    psu::OperatingPoint,
);

impl Command for SetPresets {
    const FUNCTION: &'static str = "PROM";

    fn serialize_args<S: io::Write>(
        &self,
        mut sink: S,
        psu: &psu::Info,
    ) -> command::Result<()> {
        let v_fmt = ArgFormat {
            decimals: psu.voltage_decimals,
            digits: 3,
        };

        let i_fmt = ArgFormat {
            decimals: psu.current_decimals,
            digits: 3,
        };

        v_fmt.serialize_arg(&mut sink, self.0.voltage)?;
        i_fmt.serialize_arg(&mut sink, self.0.current)?;
        v_fmt.serialize_arg(&mut sink, self.1.voltage)?;
        i_fmt.serialize_arg(&mut sink, self.1.current)?;
        v_fmt.serialize_arg(&mut sink, self.2.voltage)?;
        i_fmt.serialize_arg(&mut sink, self.2.current)?;

        Ok(())
    }
}

#[cfg(test)]
use galvanic_test::test_suite;

#[cfg(test)]
test_suite! {
    name test;

    use super::*;

    use crate::command::test_util::{
        expect_serializes_to,
        expect_cant_serialize,
    };
    use crate::psu::OperatingPoint;
    use crate::psu::test_util::{any_psu, high_voltage_psu, low_voltage_psu};
    use crate::psu::test_util::{
        invalid_current_high_voltage,
        invalid_current_low_voltage,
        invalid_voltage,
    };

    test serialize_for_low_v_psu(low_voltage_psu) {
        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 1.1,
                    current: 2.2,
                },
                OperatingPoint {
                    voltage: 3.3,
                    current: 4.4,
                },
                OperatingPoint {
                    voltage: 5.5,
                    current: 6.6,
                },
            ),
            "PROM011022033044055066\r",
            low_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 5.0,
                    current: 1.0,
                },
                OperatingPoint {
                    voltage: 12.5,
                    current: 3.0,
                },
                OperatingPoint {
                    voltage: 15.0,
                    current: 0.5,
                },
            ),
            "PROM050010125030150005\r",
            low_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
            ),
            "PROM000000000000000000\r",
            low_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 99.9,
                    current: 99.9,
                },
                OperatingPoint {
                    voltage: 99.9,
                    current: 99.9,
                },
                OperatingPoint {
                    voltage: 99.9,
                    current: 99.9,
                },
            ),
            "PROM999999999999999999\r",
            low_voltage_psu.val,
        );
    }

    test cant_serialize_if_current_unrepresentable_on_low_v(
        low_voltage_psu,
        invalid_current_low_voltage
    ) {
        let c = invalid_current_low_voltage.val;
        let invalid_p1 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p2 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p3 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
        );
        let _e = expect_cant_serialize(invalid_p1, low_voltage_psu.val);
        let _e = expect_cant_serialize(invalid_p2, low_voltage_psu.val);
        let _e = expect_cant_serialize(invalid_p3, low_voltage_psu.val);
    }

    test serialize_for_high_v_psu(high_voltage_psu) {
        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 1.1,
                    current: 0.22,
                },
                OperatingPoint {
                    voltage: 3.3,
                    current: 0.44,
                },
                OperatingPoint {
                    voltage: 5.5,
                    current: 0.66,
                },
            ),
            "PROM011022033044055066\r",
            high_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 5.0,
                    current: 1.0,
                },
                OperatingPoint {
                    voltage: 12.5,
                    current: 3.0,
                },
                OperatingPoint {
                    voltage: 15.0,
                    current: 0.55,
                },
            ),
            "PROM050100125300150055\r",
            high_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
                OperatingPoint {
                    voltage: 0.,
                    current: 0.,
                },
            ),
            "PROM000000000000000000\r",
            high_voltage_psu.val,
        );

        let _e = expect_serializes_to(
            SetPresets(
                OperatingPoint {
                    voltage: 99.9,
                    current: 9.99,
                },
                OperatingPoint {
                    voltage: 99.9,
                    current: 9.99,
                },
                OperatingPoint {
                    voltage: 99.9,
                    current: 9.99,
                },
            ),
            "PROM999999999999999999\r",
            high_voltage_psu.val,
        );
    }

    test cant_serialize_if_current_unrepresentable_on_high_v(
        high_voltage_psu,
        invalid_current_high_voltage
    ) {
        let c = invalid_current_high_voltage.val;
        let invalid_p1 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p2 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p3 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: c,
            },
        );
        let _e = expect_cant_serialize(invalid_p1, high_voltage_psu.val);
        let _e = expect_cant_serialize(invalid_p2, high_voltage_psu.val);
        let _e = expect_cant_serialize(invalid_p3, high_voltage_psu.val);
    }

    test cant_serialize_if_voltage_unrepresentable(any_psu, invalid_voltage) {
        let v = invalid_voltage.val;
        let invalid_p1 = SetPresets(
            OperatingPoint {
                voltage: v,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p2 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: v,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
        );
        let invalid_p3 = SetPresets(
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: 0.,
                current: 0.,
            },
            OperatingPoint {
                voltage: v,
                current: 0.,
            },
        );
        let _e = expect_cant_serialize(invalid_p1, any_psu.val);
        let _e = expect_cant_serialize(invalid_p2, any_psu.val);
        let _e = expect_cant_serialize(invalid_p3, any_psu.val);
    }
}

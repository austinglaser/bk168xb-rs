use super::*;

use galvanic_test::fixture;

fixture! {
    any_psu(variant: &'static SupplyVariant) -> &'static SupplyVariant {
        params {
            vec![BK1685B, BK1687B, BK1688B].into_iter()
        }
        setup(&mut self) {
            *self.variant
        }
    }
}

fixture! {
    low_voltage_psu(variant: &'static SupplyVariant) -> &'static SupplyVariant {
        params {
            vec![BK1687B, BK1688B].into_iter()
        }
        setup(&mut self) {
            *self.variant
        }
    }
}

fixture! {
    high_voltage_psu() -> &'static SupplyVariant {
        setup(&mut self) {
            BK1685B
        }
    }
}

fixture! {
    invalid_voltage(voltage: f32) -> f32 {
        params {
            vec![
                -1.,
                100.,
                101.,
                std::f32::NAN,
                std::f32::INFINITY,
                std::f32::NEG_INFINITY,
            ].into_iter()
        }
        setup (&mut self) {
            *self.voltage
        }
    }
}

fixture! {
    invalid_current_low_voltage(current: f32) -> f32 {
        params {
            vec![
                -1.,
                100.,
                101.,
                std::f32::NAN,
                std::f32::INFINITY,
                std::f32::NEG_INFINITY,
            ].into_iter()
        }
        setup (&mut self) {
            *self.current
        }
    }
}

fixture! {
    invalid_current_high_voltage(current: f32) -> f32 {
        params {
            vec![
                -1.,
                10.0,
                10.1,
                std::f32::NAN,
                std::f32::INFINITY,
                std::f32::NEG_INFINITY,
            ].into_iter()
        }
        setup (&mut self) {
            *self.current
        }
    }
}

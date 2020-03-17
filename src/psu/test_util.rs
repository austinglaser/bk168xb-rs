use super::*;

use galvanic_test::fixture;

fixture! {
    any_psu(psu: &'static Info) -> &'static Info {
        params {
            vec![BK1685B, BK1687B, BK1688B].into_iter()
        }
        setup(&mut self) {
            *self.psu
        }
    }
}

fixture! {
    low_voltage_psu(psu: &'static Info) -> &'static Info {
        params {
            vec![BK1687B, BK1688B].into_iter()
        }
        setup(&mut self) {
            *self.psu
        }
    }
}

fixture! {
    high_voltage_psu() -> &'static Info {
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

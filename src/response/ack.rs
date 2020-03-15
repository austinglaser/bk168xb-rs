use crate::{
    psu,
    response::{Response, Result},
};

/// A response indicating success, but carrying no data.
#[derive(Debug, PartialEq)]
pub struct Ack;

impl Response for Ack {
    fn arg_bytes() -> usize {
        0
    }

    fn parse_args(_raw: &[u8], _psu: &psu::Info) -> Result<Self> {
        Ok(Ack)
    }
}

#[cfg(test)]
galvanic_test::test_suite! {
    name test;

    use super::*;

    use crate::{
        psu::test_util::any_psu,
        response::test_util::{assert_deserializes_to, valid_ack},
    };

    test can_parse(any_psu, valid_ack) {
        assert_deserializes_to(valid_ack.val, Ack, any_psu.val);
    }
}

use crate::{
    psu,
    response::{Error, Response, ResponseSource},
};

use galvanic_assert::{
    assert_that, get_expectation_for, is_variant, matchers::*, Expectation,
};
use galvanic_test::fixture;

use std::{fmt::Debug, io, io::Read};

pub fn expect_deserializes_to<R: Response + Debug>(
    resp: &str,
    expected_result: R,
    psu: &psu::Info,
) -> Expectation {
    let mut source = dbg!(resp).as_bytes();

    let result = dbg!(source.get_response(psu)).unwrap();

    let byte: &mut [u8] = &mut [0; 1];
    assert_that!(
        source.read(byte).unwrap() == 0, otherwise "Not all bytes parsed"
    );

    get_expectation_for!(&result, eq(expected_result))
}

pub fn assert_deserializes_to<R: Response + Debug>(
    resp: &str,
    expected_result: R,
    psu: &psu::Info,
) {
    expect_deserializes_to(resp, expected_result, psu).verify();
}

pub fn expect_deserialize_error_from<R: Response + Debug, S: io::Read>(
    source: &mut S,
    expected_error: Error,
    psu: &psu::Info,
) -> Expectation {
    let err = dbg!(source.get_response::<R>(psu)).unwrap_err();

    match expected_error {
        Error::MalformedResponse => {
            get_expectation_for!(&err, is_variant!(Error::MalformedResponse))
        }
        Error::NoResponse => {
            get_expectation_for!(&err, is_variant!(Error::NoResponse))
        }
        Error::ReadFailure(expected_inner) => {
            if let Error::ReadFailure(inner) = err {
                get_expectation_for!(&inner.kind(), eq(expected_inner.kind()))
            } else {
                get_expectation_for!(false, otherwise "not a read failure")
            }
        }
    }
}

pub fn expect_deserialize_error<R: Response + Debug>(
    resp: &str,
    expected_error: Error,
    psu: &psu::Info,
) -> Expectation {
    let mut source = dbg!(resp).as_bytes();

    expect_deserialize_error_from::<R, _>(&mut source, expected_error, psu)
}

pub fn assert_deserialize_error<R: Response + Debug>(
    resp: &str,
    expected_error: Error,
    psu: &psu::Info,
) {
    expect_deserialize_error::<R>(resp, expected_error, psu).verify();
}

fixture! {
    valid_ack() -> &'static str {
        setup(&mut self) {
            "OK\r"
        }
    }
}

fixture! {
    invalid_ack(string: &'static str) -> &'static str {
        params {
            vec![
                "foo",
                "OK",
                "ok",
                "\r",
                "NOK\r",
                "ERROR\r",
            ].into_iter()
        }
        setup(&mut self) {
            *self.string
        }
    }
}

pub struct Arg {
    pub raw: &'static str,
    pub one_decimal: f32,
    pub two_decimals: f32,
}

fixture! {
    valid_num(string: &'static str) -> Arg {
        params {
            vec![
                "000",
                "001",
                "010",
                "100",
                "998",
                "999",
            ].into_iter()
        }
        setup(&mut self) {
            let raw = *self.string;
            let num = usize::from_str_radix(raw, 10).unwrap();
            let one_decimal = (num as f32) / 10.;
            let two_decimals = (num as f32) / 100.;

            Arg { raw, one_decimal, two_decimals }
        }
    }
}

fixture! {
    invalid_num(string: &'static str) -> &'static str {
        params {
            vec![
                "10",
                "8924",
                "22.1",
                "foo",
                "22f",
                "22.",
                "OK",
                "NOK",
                "a32",
            ].into_iter()
        }
        setup(&mut self) {
            *self.string
        }
    }
}

fixture! {
    valid_sep() -> char {
        setup(&mut self) {
            '\r'
        }
    }
}

fixture! {
    invalid_sep(sep: char) -> char {
        params {
            vec![
                ' ',
                '\n',
                '#',
                '.',
                '0',
                '9',
            ].into_iter()
        }
        setup(&mut self) {
            *self.sep
        }
    }
}

fixture! {
    io_error(kind: io::ErrorKind) -> io::ErrorKind {
        params {
            vec![
                io::ErrorKind::NotFound,
                io::ErrorKind::PermissionDenied,
                io::ErrorKind::NotConnected,
                io::ErrorKind::BrokenPipe,
                io::ErrorKind::InvalidData,
                io::ErrorKind::Interrupted,
                io::ErrorKind::Other,
            ].into_iter()
        }
        setup(&mut self) {
            *self.kind
        }
    }
}

pub struct ErrorAfter<S> {
    kind: io::ErrorKind,
    source: S,
}

impl<S> ErrorAfter<S> {
    pub fn new(kind: io::ErrorKind, source: S) -> Self {
        ErrorAfter { kind, source }
    }
}

impl<S> Read for ErrorAfter<S>
where
    S: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf).and_then(|count| {
            if count == buf.len() {
                Ok(count)
            } else {
                Err(self.kind.into())
            }
        })
    }
}

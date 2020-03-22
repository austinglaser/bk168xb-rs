use crate::{
    psu,
    response::{Error, Result},
};

use std::{io, str};

/// A type which may be a supply's response to a command.
///
/// This forms the core of the receive end of a power supply interface. Because
/// the interface is not self-describing, this trait relies on a-priori
/// knowledge of the expected response. This is the core reason for the design
/// of the entire parsing structure, which is intended to allow compile-time
/// enforcement of command/response pairings.
///
/// Each response consists of a fixed-sized argument field, followed by a
/// literal `"OK\r"` string. The specifics for each particular format can be
/// found in the programming manual for the supplies in question, beginning on
/// page 11.
///
/// This trait abstracts over the parsing of the argument field. Commmon parsing
/// logic is implemented in [`ResponseSource`](crate::response::ResponseSource).
pub trait Response: Sized + PartialEq {
    /// How many bytes make up the argument for this response.
    ///
    /// This does not include the carriage-return separating the arguments from
    /// their `OK`, but does include any internal carriage-returns which the
    /// response format may contain.
    fn arg_bytes() -> usize;

    /// Parse the argument for this response.
    fn parse_args(raw: &[u8], psu: &psu::Info) -> Result<Self>;
}

impl Response for () {
    fn arg_bytes() -> usize {
        0
    }

    fn parse_args(_raw: &[u8], _psu: &psu::Info) -> Result<Self> {
        Ok(())
    }
}

/// The receiving side of a power-supply communication link.
pub trait ResponseSource {
    /// Receive a response from the power supply.
    ///
    /// Should almost always be paired with a call to
    /// [`send_command()`](crate::command::CommandSink::send_command).
    fn get_response<R: Response>(&mut self, psu: &psu::Info) -> Result<R>;
}

impl<S> ResponseSource for S
where
    S: io::Read,
{
    fn get_response<R: Response>(&mut self, psu: &psu::Info) -> Result<R> {
        use Error::*;

        let arg_bytes = R::arg_bytes();
        let before_ok_bytes = if arg_bytes != 0 {
            // one more for the separator
            arg_bytes + 1
        } else {
            arg_bytes
        };
        let total_bytes = before_ok_bytes + OK.len();

        let mut buf: Vec<_> = vec![0; total_bytes];
        let read = self.read(&mut buf)?;
        if read == 0 {
            return Err(NoResponse);
        } else if read != total_bytes {
            return Err(MalformedResponse);
        }

        let (before_ok, ok) = buf.split_at(before_ok_bytes);
        verify_ok(ok)?;

        let args = if arg_bytes != 0 {
            let (&sep, args) =
                before_ok.split_last().ok_or(MalformedResponse)?;
            verify_sep(sep)?;

            args
        } else {
            before_ok
        };
        let resp = R::parse_args(args, psu)?;

        Ok(resp)
    }
}

const OK: &str = "OK\r";

fn verify_ok(raw: &[u8]) -> Result<()> {
    if raw != OK.as_bytes() {
        return Err(Error::MalformedResponse);
    }

    Ok(())
}

fn verify_sep(sep: u8) -> Result<()> {
    if sep != b'\r' {
        return Err(Error::MalformedResponse);
    }

    Ok(())
}

#[cfg(test)]
galvanic_test::test_suite! {
    name test;

    use crate::{
        psu::{self, test_util::any_psu},
        response::{
            test_util::{
                assert_deserializes_to, expect_deserialize_error,
                expect_deserialize_error_from, invalid_ack, invalid_sep,
                io_error, valid_ack, valid_sep, ErrorAfter,
            },
            Current,
            Error::*,
            Presets, Response, Settings, Status, Voltage,
        },
    };

    use galvanic_assert::Expectation;

    use core::fmt::Debug;
    use std::io;
    use std::iter::repeat;

    test can_parse(any_psu, valid_ack) {
        assert_deserializes_to(valid_ack.val, (), any_psu.val);
    }

    test fails_to_parse_with_no_response(any_psu) {
        let psu = any_psu.val;

        let _e = expect_no_resp_parse_error::<()>(psu);
        let _e = expect_no_resp_parse_error::<Voltage>(psu);
        let _e = expect_no_resp_parse_error::<Current>(psu);
        let _e = expect_no_resp_parse_error::<Settings>(psu);
        let _e = expect_no_resp_parse_error::<Status>(psu);
        let _e = expect_no_resp_parse_error::<Presets>(psu);

        fn expect_no_resp_parse_error<R: Response + Debug>(
            psu: &psu::Info,
        ) -> Expectation {
            expect_deserialize_error::<R>("", NoResponse, psu)
        }
    }

    test fails_to_parse_with_no_value(any_psu, valid_ack) {
        let psu = any_psu.val;
        let ack = valid_ack.val;

        let _e = expect_no_val_parse_error::<Voltage>(psu, ack);
        let _e = expect_no_val_parse_error::<Current>(psu, ack);
        let _e = expect_no_val_parse_error::<Settings>(psu, ack);
        let _e = expect_no_val_parse_error::<Status>(psu, ack);
        let _e = expect_no_val_parse_error::<Presets>(psu, ack);

        fn expect_no_val_parse_error<R: Response + Debug>(
            psu: &psu::Info,
            ack: &str
        ) -> Expectation {
            expect_deserialize_error::<R>(ack, MalformedResponse, psu)
        }
    }

    test fails_to_parse_with_no_separator(any_psu, valid_ack) {
        let psu = any_psu.val;
        let ack = valid_ack.val;

        let _e = expect_no_sep_parse_error::<Voltage>(psu, ack);
        let _e = expect_no_sep_parse_error::<Current>(psu, ack);
        let _e = expect_no_sep_parse_error::<Settings>(psu, ack);
        let _e = expect_no_sep_parse_error::<Status>(psu, ack);
        let _e = expect_no_sep_parse_error::<Presets>(psu, ack);

        fn expect_no_sep_parse_error<R: Response + Debug>(
            psu: &psu::Info,
            ack: &str
        ) -> Expectation {
            let mut resp = dummy_arg_for::<R>();
            resp.push_str(ack);

            expect_deserialize_error::<R>(&resp, MalformedResponse, psu)
        }
    }

    test fails_to_parse_with_invalid_separator(
        any_psu,
        invalid_sep,
        valid_ack
    ) {
        let psu = any_psu.val;
        let ack = valid_ack.val;
        let sep = invalid_sep.val;

        let _e = expect_bad_sep_parse_error::<()>(psu, sep, ack);
        let _e = expect_bad_sep_parse_error::<Voltage>(psu, sep, ack);
        let _e = expect_bad_sep_parse_error::<Current>(psu, sep, ack);
        let _e = expect_bad_sep_parse_error::<Settings>(psu, sep, ack);
        let _e = expect_bad_sep_parse_error::<Status>(psu, sep, ack);
        let _e = expect_bad_sep_parse_error::<Presets>(psu, sep, ack);

        fn expect_bad_sep_parse_error<R: Response + Debug>(
            psu: &psu::Info,
            sep: char,
            ack: &str
        ) -> Expectation {
            let mut resp = dummy_arg_for::<R>();
            resp.push(sep);
            resp.push_str(ack);

            expect_deserialize_error::<R>(&resp, MalformedResponse, psu)
        }
    }

    test fails_to_parse_with_duplicate_separator(
        any_psu,
        valid_sep,
        valid_ack
    ) {
        let psu = any_psu.val;
        let ack = valid_ack.val;
        let sep = valid_sep.val;

        let _e = expect_dupe_sep_parse_error::<()>(psu, sep, ack);
        let _e = expect_dupe_sep_parse_error::<Voltage>(psu, sep, ack);
        let _e = expect_dupe_sep_parse_error::<Current>(psu, sep, ack);
        let _e = expect_dupe_sep_parse_error::<Settings>(psu, sep, ack);
        let _e = expect_dupe_sep_parse_error::<Status>(psu, sep, ack);
        let _e = expect_dupe_sep_parse_error::<Presets>(psu, sep, ack);

        fn expect_dupe_sep_parse_error<R: Response + Debug>(
            psu: &psu::Info,
            sep: char,
            ack: &str
        ) -> Expectation {
            let mut resp = dummy_arg_for::<R>();
            resp.push(sep);
            resp.push(sep);
            resp.push_str(ack);

            expect_deserialize_error::<R>(&resp, MalformedResponse, psu)
        }
    }

    test fails_to_parse_with_bad_ack(
        any_psu,
        valid_sep,
        invalid_ack
    ) {
        let psu = any_psu.val;
        let ack = invalid_ack.val;
        let sep = valid_sep.val;

        let _e = expect_invalid_ack_parse_error::<()>(psu, sep, ack);
        let _e = expect_invalid_ack_parse_error::<Voltage>(psu, sep, ack);
        let _e = expect_invalid_ack_parse_error::<Current>(psu, sep, ack);
        let _e = expect_invalid_ack_parse_error::<Settings>(psu, sep, ack);
        let _e = expect_invalid_ack_parse_error::<Status>(psu, sep, ack);
        let _e = expect_invalid_ack_parse_error::<Presets>(psu, sep, ack);

        fn expect_invalid_ack_parse_error<R: Response + Debug>(
            psu: &psu::Info,
            sep: char,
            ack: &str
        ) -> Expectation {
            let mut resp = dummy_arg_for::<R>();
            if resp.len() != 0 {
                resp.push(sep);
            }
            resp.push_str(ack);

            expect_deserialize_error::<R>(&resp, MalformedResponse, psu)
        }
    }

    test propogates_io_error(any_psu, io_error) {
        let psu = any_psu.val;
        let err = io_error.val;

        let _e = expect_catches_io_error::<()>(psu, err);
        let _e = expect_catches_io_error::<Voltage>(psu, err);
        let _e = expect_catches_io_error::<Current>(psu, err);
        let _e = expect_catches_io_error::<Settings>(psu, err);
        let _e = expect_catches_io_error::<Status>(psu, err);
        let _e = expect_catches_io_error::<Presets>(psu, err);

        fn expect_catches_io_error<R: Response + Debug>(
            psu: &psu::Info,
            err: io::ErrorKind,
        ) -> Expectation {
            let mut data = "".as_bytes();
            let mut source = ErrorAfter::new(err, &mut data);

            expect_deserialize_error_from::<R, _>(
                &mut source,
                ReadFailure(err.into()),
                psu
            )
        }
    }

    fn dummy_arg_for<R: Response>() -> String {
        repeat('0').take(R::arg_bytes()).collect()
    }
}

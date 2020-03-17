//! Assertions specific to dealing with BK168xB commands.

use crate::{
    command::{Command, CommandSink, Error, Result},
    SupplyVariant,
};

use galvanic_assert::{
    get_expectation_for, matchers::*, Expectation, MatchResult,
    MatchResultBuilder,
};

use std::str;

pub fn assert_cant_serialize<C: Command>(command: C, variant: &SupplyVariant) {
    expect_cant_serialize(command, variant).verify();
}

pub fn expect_cant_serialize<C: Command>(
    command: C,
    variant: &SupplyVariant,
) -> Expectation {
    let mut sink = Vec::new();

    get_expectation_for!(
        &sink.send_command(&command, variant),
        is_unrepresentable_val_error
    )
}

fn is_unrepresentable_val_error<T>(res: &Result<T>) -> MatchResult {
    let builder = MatchResultBuilder::for_("is unrepresentable value error");

    if let Err(ref e) = *res {
        if let Error::ValueUnrepresentable(_) = *e {
            builder.matched()
        } else {
            builder.failed_because("wrong type of error")
        }
    } else {
        builder.failed_because("not an error")
    }
}

pub fn assert_serializes_to<C: Command>(
    command: C,
    result: &str,
    variant: &SupplyVariant,
) {
    expect_serializes_to(command, result, variant).verify();
}

pub fn expect_serializes_to<C: Command>(
    command: C,
    result: &str,
    variant: &SupplyVariant,
) -> Expectation {
    let mut sink = Vec::new();

    sink.send_command(&command, variant).unwrap();
    let written = str::from_utf8(&sink).unwrap();

    get_expectation_for!(&written, eq(result))
}

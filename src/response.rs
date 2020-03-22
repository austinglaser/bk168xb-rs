//! Type-safe BK168xB response parsing

mod capabilities;
mod core;
mod current;
mod error;
mod presets;
mod settings;
mod status;
mod voltage;

#[cfg(test)]
pub(crate) mod test_util;

pub use self::{
    capabilities::*, core::*, current::*, error::*, presets::*, settings::*,
    status::*, voltage::*,
};

#![warn(clippy::disallowed_types)]

pub use metadata::kusama as runtime;
pub use metadata::kusama::api::runtime_types::staging_kusama_runtime::RuntimeCall;

pub mod dv;
pub mod err;
pub mod governance;
mod metadata;
pub mod substrate;
pub mod util;

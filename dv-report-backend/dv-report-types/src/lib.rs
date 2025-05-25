#![warn(clippy::disallowed_types)]

#[cfg(all(feature = "polkadot", feature = "kusama"))]
compile_error!("You must enable only one of the features: polkadot or kusama.");
#[cfg(feature = "kusama")]
pub use metadata::kusama as runtime;
#[cfg(feature = "kusama")]
pub use metadata::kusama::api::runtime_types::staging_kusama_runtime::RuntimeCall;
#[cfg(feature = "polkadot")]
pub use metadata::polkadot as runtime;
#[cfg(feature = "polkadot")]
pub use metadata::polkadot::api::runtime_types::polkadot_runtime::RuntimeCall;

pub mod dv;
pub mod err;
pub mod governance;
mod metadata;
pub mod substrate;
pub mod util;

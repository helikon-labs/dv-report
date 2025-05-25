#![warn(clippy::disallowed_types)]

#[cfg(all(feature = "polkadot", feature = "kusama"))]
compile_error!("You must enable only one of the features: polkadot or kusama.");
#[cfg(feature = "kusama")]
pub use metadata::kusama as runtime;
#[cfg(feature = "polkadot")]
pub use metadata::polkadot as runtime;

pub mod dv;
pub mod err;
pub mod governance;
mod metadata;
pub mod substrate;
pub mod util;

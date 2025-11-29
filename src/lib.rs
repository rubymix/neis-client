#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
mod error;
pub mod types;

#[cfg(feature = "client")]
pub use client::NeisClient;
#[cfg(feature = "client")]
pub use error::Error;

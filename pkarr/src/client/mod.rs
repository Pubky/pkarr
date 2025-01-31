//! Client implementation.

pub mod cache;

#[cfg(all(not(target_arch = "wasm32"), feature = "dht"))]
pub(crate) mod dht;
#[cfg(all(not(target_arch = "wasm32"), feature = "dht"))]
pub use dht::{resolvres_to_socket_addrs, Client, Config};

#[cfg(all(target_arch = "wasm32", feature = "relay"))]
pub(crate) mod relay;
#[cfg(all(target_arch = "wasm32", feature = "relay"))]
pub use relay::{Client, Config};

#[cfg(all(not(target_arch = "wasm32"), feature = "relay"))]
pub mod relay;

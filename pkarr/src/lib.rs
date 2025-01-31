#![doc = include_str!("../README.md")]
//! ## Feature flags
#![doc = document_features::document_features!()]
//!

// TODO: once we merge udp and relay clients, we should have a "client" feature

// Modules
#[cfg(any(target_arch = "wasm32", feature = "relay", feature = "dht"))]
pub mod client;
pub mod extra;
mod keys;
mod signed_packet;

/// Default minimum TTL: 5 minutes
pub const DEFAULT_MINIMUM_TTL: u32 = 300;
/// Default maximum TTL: 24 hours
pub const DEFAULT_MAXIMUM_TTL: u32 = 24 * 60 * 60;
/// Default cache size: 1000
pub const DEFAULT_CACHE_SIZE: usize = 1000;
/// Default [relay](https://pkarr.org/relays)s
pub const DEFAULT_RELAYS: [&str; 2] = ["https://relay.pkarr.org", "https://pkarr.pubky.org"];
/// Default [resolver](https://pkarr.org/resolvers)s
pub const DEFAULT_RESOLVERS: [&str; 2] = ["resolver.pkarr.org:6881", "pkarr.pubky.org:6881"];

// Exports
#[cfg(any(target_arch = "wasm32", feature = "relay", feature = "dht"))]
pub use client::cache::{Cache, CacheKey, InMemoryCache};
pub use keys::{Keypair, PublicKey};
pub use signed_packet::SignedPacket;

#[cfg(all(not(target_arch = "wasm32"), feature = "dht"))]
pub use client::dht::Info;
#[cfg(any(feature = "relay", feature = "dht"))]
pub use client::{Client, Config};

// Rexports
pub use simple_dns as dns;

pub mod errors {
    //! Exported errors

    #[cfg(all(not(target_arch = "wasm32"), feature = "dht"))]
    pub use super::client::dht::{ClientWasShutdown, PublishError};

    #[cfg(feature = "relay")]
    pub use super::client::relay::{EmptyListOfRelays, PublishToRelayError};

    pub use super::keys::PublicKeyError;
    pub use super::signed_packet::SignedPacketBuildError;
    pub use super::signed_packet::SignedPacketVerifyError;
}

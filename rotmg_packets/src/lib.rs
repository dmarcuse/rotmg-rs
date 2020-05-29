//! Types and utilities for encoding and decoding ROTMG packets.
//!
//! This crate provides functionality to represent and manipulate ROTMG packets.
//! It defines structs for every packet type known at the time of writing, as
//! well as traits and implementations to allow encoding/decoding the packets as
//! bytes, as they are in the official client and server.
//!
//! If desired, `serde` support can be enabled with the `serde` feature flag,
//! allowing packets to be serialized and deserialized to arbitrary formats.

#![warn(missing_docs)]

pub mod adapters;
mod mappings;
pub mod raw;
pub mod structured;

pub use mappings::PacketMappings;

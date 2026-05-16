//! Actix integration exports.
//!
//! The core client and Bridge validator are framework agnostic. A full Actix
//! middleware wrapper can be layered on top of `BridgeValidator` without
//! changing the public SDK API.

pub use crate::bridge::{BridgeValidator, SessionData, ValidationMode, ValidationOptions};

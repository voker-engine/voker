//! Core internals of the voker engine.
//!
//! This crate holds the engine implementation and acts as the central point for
//! **feature management**: the public [`voker`] facade forwards its features
//! (`std`, `debug`, `backtrace`, `dylib`) down to this crate, which in turn
//! enables the matching behavior across the engine. Downstream crates should
//! depend on `voker` rather than on this crate directly.
//!
//! [`voker`]: https://github.com/voker-engine/voker
#![no_std]

pub use voker_reg as reg;
pub use voker_utils as utils;

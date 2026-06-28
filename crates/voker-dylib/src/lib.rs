//! Dynamic-linking helper crate.
//!
//! This crate exists purely to speed up iterative development builds. It is
//! compiled as a `dylib` (`crate-type = ["dylib"]`) and force-links
//! [`voker_internal`], so the bulk of the engine is built into a single shared
//! object that does not need to be re-linked into every dependent binary.
//!
//! It contains no real logic — the lone `use voker_internal;` only keeps the
//! symbols from being stripped. It is gated behind the root crate's `dylib`
//! feature and excluded on `wasm` targets, where dynamic linking is unavailable.
#![no_std]

#[expect(unused_imports, reason = "Force linking to keep it from being stripped")]
#[expect(clippy::single_component_path_imports, reason = "Keep dylib linked.")]
use voker_internal;

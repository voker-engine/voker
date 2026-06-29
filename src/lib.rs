#![no_std]

pub use voker_internal::*;

#[cfg(all(feature = "dylib", not(target_family = "wasm")))]
#[expect(
    unused_imports,
    clippy::single_component_path_imports,
    reason = "Force linking to keep it from being stripped"
)]
use voker_dylib;

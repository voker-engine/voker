#![no_std]

// -----------------------------------------------------------------------------
// no_std support

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

// -----------------------------------------------------------------------------
// Modules

pub mod atomic;
pub mod sync;
pub mod thread;
pub mod time;
pub mod utils;

// -----------------------------------------------------------------------------
// Special platform support

#[doc(hidden)]
pub mod exports {
    #[cfg(target_family = "windows")]
    pub use windows_sys;

    #[cfg(target_os = "android")]
    pub use android_activity;

    #[cfg(target_family = "wasm")]
    pub use js_sys;

    #[cfg(target_family = "wasm")]
    pub use wasm_bindgen;

    #[cfg(target_family = "wasm")]
    pub use wasm_bindgen_futures;
}

# Platform related extensions

Cross-platform, `no_std`-first abstractions over essential OS functionality.

## Architecture Overview

Rust's standard library is organized into three layers for maximum portability:

- **core**: Language core functionality, independent of OS and allocator
- **alloc**: Memory allocation APIs and common containers (`String`, `Vec`, etc.)
- **std**: OS-level APIs (atomics, synchronization, threads, time, etc.)

While Rust provides extensive [cross-platform support] through `std`,
it cannot cover every possible target—especially custom embedded systems or
specialized game consoles. This crate provides a thin abstraction layer over the
OS facilities the engine relies on, with backends selected at compile time so the
same API works on `std`, WebAssembly, and bare-metal `no_std` targets.

[cross-platform support]: https://doc.rust-lang.org/nightly/rustc/platform-support.html

---

## Modules

- [`atomic`]: Atomic types, falling back to `portable_atomic` when the target lacks
  native atomics. Also provides the [`define_atomic_id!`] macro.
- [`sync`]: Synchronization primitives (`Mutex`, `RwLock`, `Once`, `OnceLock`,
  `LazyLock`, …) mirroring `std::sync`, plus a `no_std` [`SpinLock`](sync::SpinLock).
- [`time`]: Time measurement APIs (`Instant`, `SystemTime`, `Duration`).
- [`thread`]: Thread utilities (`sleep`, `available_parallelism`, `thread_hash`).
- [`utils`]: Lock-free / spin-based concurrent data structures (`ArrayQueue`,
  `SegQueue`, `ListQueue`, `OnceFlag`, `CachePadded`, `Backoff`).

---

## Design Philosophy

Each module exposes a single API and selects its implementation at compile time.
Wherever possible the fallbacks preserve the standard library's *stable* surface,
so code written against this crate behaves identically across backends (internal
details such as container sizes may differ).

### Standard Backend (Default)

- **Direct re-exports** of `std` APIs with zero runtime overhead.
- **Use case**: most desktop, mobile, and server targets.
- **Enabled by**: the default `std` feature.

### WebAssembly Backend

- Automatically selected on `target_family = "wasm"`.
- **Time** is provided by the [`web_time`] crate (browser-backed `Instant` /
  `SystemTime`); `available_parallelism` currently reports `1`.
- Relevant wasm bindings (`js-sys`, `wasm-bindgen`, `wasm-bindgen-futures`) are
  pulled in automatically for wasm targets.

### No-Std Backend

- Automatically selected when the `std` feature is **disabled**.
- **Synchronization** falls back to spinlock-based implementations.
- **Atomics** fall back to `portable_atomic` on targets without native support
  (platforms lacking atomic pointers are unsupported, since `Arc` requires them).
- **`Instant`** uses a built-in counter on `x86`, `x86_64`, and `aarch64`; on other
  architectures a monotonic timer must be supplied via `Instant::set_elapsed_getter`.
- **`SystemTime`** has no default and must always be configured via
  `SystemTime::set_elapsed_getter`.
- **`sleep`** is spin-based and `available_parallelism` reports `1`.
- A custom global allocator is required (the crate still uses `alloc`).

[`web_time`]: https://docs.rs/web-time

---

## Platform Support Crates

Beyond the backends above, the crate re-exports a few target-specific dependencies
under `exports` for internal use:

- **Windows**: `windows-sys`
- **Android**: `android-activity`
- **WebAssembly**: `js-sys`, `wasm-bindgen`, `wasm-bindgen-futures`

---

## Feature Flags

### `std` (enabled by default)

- Uses standard library implementations for every module.
- Provides full OS-level functionality.
- Disable it (`default-features = false`) to opt into the `no_std` fallback backend.

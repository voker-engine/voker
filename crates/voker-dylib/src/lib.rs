#![no_std]

#[expect(unused_imports, reason = "Force linking to keep it from being stripped")]
#[expect(clippy::single_component_path_imports, reason = "Keep dylib linked.")]
use voker_internal;

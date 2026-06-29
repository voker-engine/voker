#![expect(
    unsafe_code,
    reason = "This crate relies on many underlying operations."
)]
#![no_std]

extern crate alloc;

pub mod component;
pub mod entity;
pub mod script;

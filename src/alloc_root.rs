//! Testing `no_std`-compatible alloc-based code of [camigo_helpers].
#![no_std]

extern crate alloc as rust_alloc;

mod alloc;

#[path = "core.rs"]
mod core;
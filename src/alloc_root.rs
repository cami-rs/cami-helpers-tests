//! Testing `no_std`-compatible alloc-based code of [camigo_helpers].
#![no_std]

extern crate alloc as rust_alloc;

#[path = "alloc/alloc_mod.rs"]
mod alloc;

#[path = "core/core_mod.rs"]
mod core;

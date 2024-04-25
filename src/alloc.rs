//! Testing `no_std`-compatible alloc-based code of [camigo_helpers].

#![no_std]

extern crate alloc as rust_alloc;

// See <core.rs> about organization of these tests, and about why there is the following module.
mod alloc {
    mod macros_cami_tests_basic;
    mod macros_cami_tests_party;
    mod macros_core_tests_basic;
}

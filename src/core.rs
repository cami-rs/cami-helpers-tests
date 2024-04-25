//! Testing `no_std` & no-alloc-compatible code of [camigo_helpers].
//!
// Based on https://mozilla.github.io/application-services/book/design/test-faster.html.
//
// This extra module level is here, so that we have module directory navigation the same as in Rust
// libraries. Why is it needed? Because [[test]] makes this file the top of the generated test crate
// (one of the test crates). It makes it the top level (of such a generated test crate), above any
// modules), rather than make it a module (`core`) itself.

#![no_std]

mod core {}

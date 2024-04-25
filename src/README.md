## Test organization

Tests are under `src/` (instead of the usual `tests/`). That's based on https://mozilla.github.io/application-services/book/design/test-faster.html.

`*_top_level.rs` files exist, so that we have module directory navigation the same as in Rust
libraries. Why is it needed? Because [[test]] makes this file the top (root) of the generated test
crate (one of the test crates). But,

- we want the module hierarchy reflect Rust's `core`, `alloc` and `std`. And
- we want `std` to "include" (load in/refer to) both `core` and `alloc`, and we want `alloc` to "include" (load in/refer to) `core`. But, Rust/Cargo doesn't allow various test targets (and maybe any various targets) from the same package to depend on each other. So, instead we have to use `#[path = "core.rs"] mod core;` in both `alloc` and `std` (and similarly, use `#[path = "alloc.rs"] mod alloc;` in `std`). And/but
- we don't want extra directory levels (like `src/core/core/`...


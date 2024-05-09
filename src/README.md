# Test organization

## Tests are under src/

Tests are under `src/` - instead of the usual `tests/`. That's based on
[mozilla.github.io/application-services/book/design/test-faster.html](https://mozilla.github.io/application-services/book/design/test-faster.html).

## Why *_root.rs files

`*_root.rs` files exist, so that we have module directory navigation the same way as in Rust
libraries.

Why are those `*/*root.rs` files needed - in addition to `*/*_mod.rs`? Because [[test]] makes this
file the top (root) level of the generated test crate (one of the test crates). But,

- we want the module hierarchy reflect Rust's `core`, `alloc` and `std`. And
- we want `std` to "include" (load in) both `core` and `alloc`, and we want `alloc` to "include"
  (load in) `core`. We have to do that, because Rust/Cargo doesn't allow various `test` targets (and
  maybe any various targets) from the same package to depend on each other. So, instead we have to
  use `#[path = "core_md.rs"] mod core;` in both `alloc` and `std` (and similarly, use `#[path =
  "alloc.rs"] mod alloc;` in `std`). And/but
- we don't want extra directory levels (like `src/core/core/`).

## Why */*_mod.rs

We don't want `mod.rs` files (as per [cami > CONTRIBUTING > Internal
Conventions](https://github.com/peter-kehl/cami/blob/main/CONTRIBUTING.md#internal-conventions)).

Instead of `core/core_mod.rs` we could have `core/core.rs` (and similar for `alloc` and `std`). But
then `core` and `core_root` (and similar for `alloc` and `alloc_root`, `std` and `std_root`) would
be more ambiguous.

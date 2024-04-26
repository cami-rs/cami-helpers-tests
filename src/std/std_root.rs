// Not needed for `std/` tests themselves, but needed for `alloc/` module included below.
extern crate alloc as rust_alloc;

#[path = "../core/core_mod.rs"]
mod core;

#[path = "../alloc/alloc_mod.rs"]
mod alloc;

#[path = "std_mod.rs"]
mod std;

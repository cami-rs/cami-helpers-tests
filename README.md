# Testing camigo-helpers

## Why is this NOT a part of camigo-helpers?

This depends on both `camigo` and `camigo-helpers`. But, since `camigo` depends on `camigo-helpers`,
if we made these tests a part of `camigo-helpers`, it would fail: "circular dependencies."

## On Cargo.toml

This doesn't use [Cargo target
auto-discovery](https://doc.rust-lang.org/nightly/cargo/reference/cargo-targets.html#target-auto-discovery).
Why? Because standard [Integration
Tests](https://doc.rust-lang.org/nightly/book/ch11-03-test-organization.html#integration-tests)
can't have their own `Cargo.toml`.

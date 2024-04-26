# Integration Tests of camigo-helpers

## Why is this not a part of camigo-helpers?

It cannot. This depends on both `camigo` and `camigo-helpers`. But, `camigo` also depends on
`camigo-helpers`. If we moved these tests to `camigo-helpers` instead, it would fail: "circular
dependencies."

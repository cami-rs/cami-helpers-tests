# Integration Tests of camigo-helpers

## Why is this not a part of camigo-helpers?

This depends on both `camigo` and `camigo-helpers`. But, since `camigo` depends on `camigo-helpers`,
if we made these tests a part of `camigo-helpers`, it would fail: "circular dependencies."

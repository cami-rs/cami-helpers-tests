# Integration Tests of cami-helpers

## Why is this not a part of cami-helpers?

It cannot. This depends on both `cami` and `cami-helpers`. But, `cami` also depends on
`cami-helpers`. If we moved these tests to `cami-helpers` instead, it would fail: "circular
dependencies."

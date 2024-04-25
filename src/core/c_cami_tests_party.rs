use camigo_helpers::{cami_partial_eq, Locality};

#[test]
fn main() {}

struct Empty {}
cami_partial_eq! {
    Empty {
        Locality::Both
    }
    []
    []
    []
}

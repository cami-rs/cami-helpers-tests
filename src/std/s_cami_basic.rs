use ::std::{collections::HashSet, string::String, vec::Vec};
use cami_helpers::{cami_ord, cami_partial_eq, Locality};

#[test]
fn main() {}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct A /*where A: ::core::hash::Hash*/ {
    // ->  overflow evaluating the requirement issue #48214
    x: i32,
    v: Vec<i32>,
}

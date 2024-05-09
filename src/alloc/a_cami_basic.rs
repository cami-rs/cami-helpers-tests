use cami_helpers::{cami_ord, cami_partial_eq, Locality};
use rust_alloc::{string::String, vec::Vec};

#[test]
fn main() {}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct A {
    x: i32,
    v: Vec<i32>,
}

struct _CaWrap2(pub Vec<A>);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct CaWrapA1(A);
cami_partial_eq! {
    {CaWrapA1}
    (Locality::Both)
    [ /*(|this: &A, other: &A| this.x==other.x)*/ ]
    [/* .v*/]
}
cami_ord! {
    CaWrapA1 { 0 }
    [ {|a: &A| a.x} ]
    [v]
}

struct _CaTupleGen1<T>(pub T)
where
    T: Clone;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct CaTupleA2(A);
fn get_v<'a>(wrap: &'a A) -> &'a Vec<i32> {
    &wrap.v
}
cami_partial_eq! {
    {CaTupleA2}
    (Locality::Both)
    [ /*{|obj: &_| obj.x*}*/
    ]
    [ /*{get_v}*/ ]
    []
}
cami_ord! {
    CaTupleA2 { 0 }
    [( |this: &A, other: &A| this.x.cmp(&other.x) )]
    [v]
}

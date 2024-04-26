use ::std::{collections::HashSet, string::String, vec::Vec};
use camigo_helpers::{cami_ord, cami_partial_eq, cami_wrap_struct, cami_wrap_tuple, Locality};

#[test]
fn main() {}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct A
/*where A: ::core::hash::Hash*/
{ // ->  overflow evaluating the requirement issue #48214
    x: i32,
    v: Vec<i32>,
}

use ::core::hash::Hash as HashTrait;

struct _BoundWithAbsolutePath<T>
where T: ::core::hash::Hash {v: Vec<T>}

cami_wrap_struct! {
    _CaWrap2 <A>
     where A: Sized { // where A: Sized+Ord
        pub t : Vec<A>
    }
}

macro_rules! plus {
    ($left:literal + $right:literal) => {
        0
    };
}

const _PLUS: () = {
    let zero = plus!( 1 + 3);
    if zero!=0 {
        panic!("zero!");
    }
};

/*cami_wrap_struct! {
    _CaWrapHash <A>
     where A: HashTrait+Ord { // @TODO where A: ::core::hash::Hash
        pub t : HashSet<A>
    }
}*/

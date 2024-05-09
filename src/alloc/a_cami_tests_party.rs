use cami_helpers::{cami_partial_eq, Locality};
use rust_alloc::{string::String, vec::Vec};

#[test]
fn main() {}

type Amount = u16;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Food {
    name: String,
    amount: Amount,
}
impl Food {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn amount(&self) -> Amount {
        self.amount
    }
}

pub struct FoodList {
    common: Food,
    gluten_free: Food,
    dairy_free: Food,
    vegan: Food,
}
impl FoodList {
    pub fn common(&self) -> &Food {
        &self.common
    }
    pub fn gluten_free(&self) -> &Food {
        &self.gluten_free
    }
    pub fn dairy_free(&self) -> &Food {
        &self.dairy_free
    }
    pub fn vegan(&self) -> &Food {
        &self.vegan
    }
}

pub struct Table(Food, Food);

pub struct Room(Table, Table);

cami_partial_eq! {
    {Food}
    (Locality::Both)
    [amount]
    []
    [name]
}

cami_partial_eq! {
    {FoodList}
    (Locality::Both)
    [
        common.amount,
        gluten_free.amount,
        .dairy_free.amount(),
        {|food_list| food_list.vegan.amount}
    ]
    [   common.name,
        gluten_free.name(),
        dairy_free().name,
        (|this| &this.vegan.name)
    ]
    []
}
cami_partial_eq! {
    {Table}
    (Locality::Both)
    [
        .0.amount,
    ]
    [   .0.name(),
    ]
    [.1]
}
cami_partial_eq! {
    {Room}
    (Locality::Both)
    []
    []
    [.0, .1]
}

# Trait Enum

[![Build Status](https://travis-ci.com/Trangar/trait_enum.svg?branch=master)](https://travis-ci.com/Trangar/trait_enum)

An enum wrapper for types that implement the same trait

The `trait_enum` macro generates an `enum` that manages
several objects.

These objects are expected to have the same trait impl

After which the enum will have a `std::ops::Deref` impl
which returns a reference to that trait.

``` rust
#[macro_use]
extern crate trait_enum;

pub trait CommonTrait {
    fn test(&self) -> u32;
}

pub struct InnerOne;
impl CommonTrait for InnerOne {
    fn test(&self) -> u32 {
        1
    }
}

pub struct InnerTwo;
impl CommonTrait for InnerTwo {
    fn test(&self) -> u32 {
        2
    }
}

trait_enum!{
    pub enum Combined: CommonTrait {
        InnerOne,
        InnerTwo,
    }
}

fn main() {
    use std::ops::Deref;

    let combined = Combined::InnerOne(InnerOne);
    let deref: &CommonTrait = combined.deref();
    assert_eq!(deref.test(), 1);

    let combined = Combined::InnerTwo(InnerTwo);
    let deref: &CommonTrait = combined.deref();
    assert_eq!(deref.test(), 2);
}
```

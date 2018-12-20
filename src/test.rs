use crate::*;

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
        InnerTwo
    }
}

#[test]
fn test_enum() {
    use std::ops::Deref;

    let combined = Combined::InnerOne(InnerOne);
    let deref: &CommonTrait = combined.deref();
    assert_eq!(deref.test(), 1);

    let combined = Combined::InnerTwo(InnerTwo);
    let deref: &CommonTrait = combined.deref();
    assert_eq!(deref.test(), 2);
}
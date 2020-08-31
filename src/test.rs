use crate::*;

pub trait CommonTrait {
    fn test(&self) -> u32;
    fn update(&mut self) -> u32;
}

pub struct InnerOne(u32);
impl CommonTrait for InnerOne {
    fn test(&self) -> u32 {
        self.0
    }

    fn update(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

pub struct InnerTwo(u32);
impl CommonTrait for InnerTwo {
    fn test(&self) -> u32 {
        self.0
    }

    fn update(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

trait_enum! {
    pub enum Combined: CommonTrait {
        InnerOne,
        InnerTwo
    }
}

#[test]
fn test_enum() {
    #[cfg(not(feature = "std"))]
    pub use core::ops::{Deref, DerefMut};
    #[cfg(feature = "std")]
    pub use std::ops::{Deref, DerefMut};

    let mut combined = Combined::InnerOne(InnerOne(1));
    {
        let deref: &dyn CommonTrait = combined.deref();
        assert_eq!(deref.test(), 1);
    }
    {
        let deref_mut: &mut dyn CommonTrait = combined.deref_mut();
        assert_eq!(deref_mut.update(), 2);
        assert_eq!(deref_mut.test(), 2);
    }

    let mut combined = Combined::InnerTwo(InnerTwo(2));
    {
        let deref: &dyn CommonTrait = combined.deref();
        assert_eq!(deref.test(), 2);
    }
    {
        let deref_mut: &mut dyn CommonTrait = combined.deref_mut();
        assert_eq!(deref_mut.update(), 3);
        assert_eq!(deref_mut.test(), 3);
    }
}

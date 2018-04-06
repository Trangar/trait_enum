//! An enum wrapper for types that implement the same trait
//! 
//! The `trait_enum` macro generates an `enum` that manages
//! several objects.
//! 
//! These objects are expected to have the same trait impl
//! 
//! After which the enum will have a `std::ops::Deref` impl
//! which returns a reference to that trait.
//!
//! ``` rust
//! #[macro_use]
//! extern crate trait_enum;
//! 
//! pub trait CommonTrait {
//!     fn test(&self) -> u32;
//! }
//! 
//! pub struct InnerOne;
//! impl CommonTrait for InnerOne {
//!     fn test(&self) -> u32 {
//!         1
//!     }
//! }
//! 
//! pub struct InnerTwo;
//! impl CommonTrait for InnerTwo {
//!     fn test(&self) -> u32 {
//!         2
//!     }
//! }
//! 
//! trait_enum!{
//!     pub enum Combined: CommonTrait {
//!         InnerOne,
//!         InnerTwo,
//!     }
//! }
//! 
//! fn main() {
//!     use std::ops::Deref;
//! 
//!     let combined = Combined::InnerOne(InnerOne);
//!     let deref: &CommonTrait = combined.deref();
//!     assert_eq!(deref.test(), 1);
//! 
//!     let combined = Combined::InnerTwo(InnerTwo);
//!     let deref: &CommonTrait = combined.deref();
//!     assert_eq!(deref.test(), 2);
//! }
//! ```

/// An enum wrapper for types that implement the same trait
/// 
/// The `trait_enum` macro generates an `enum` that manages
/// several objects.
/// 
/// These objects are expected to have the same trait impl
/// 
/// After which the enum will have a `std::ops::Deref` impl
/// which returns a reference to that trait.
///
/// ``` rust
/// #[macro_use]
/// extern crate trait_enum;
/// 
/// pub trait CommonTrait {
///     fn test(&self) -> u32;
/// }
/// 
/// pub struct InnerOne;
/// impl CommonTrait for InnerOne {
///     fn test(&self) -> u32 {
///         1
///     }
/// }
/// 
/// pub struct InnerTwo;
/// impl CommonTrait for InnerTwo {
///     fn test(&self) -> u32 {
///         2
///     }
/// }
/// 
/// trait_enum!{
///     pub enum Combined: CommonTrait {
///         InnerOne,
///         InnerTwo,
///     }
/// }
/// 
/// fn main() {
///     use std::ops::Deref;
/// 
///     let combined = Combined::InnerOne(InnerOne);
///     let deref: &CommonTrait = combined.deref();
///     assert_eq!(deref.test(), 1);
/// 
///     let combined = Combined::InnerTwo(InnerTwo);
///     let deref: &CommonTrait = combined.deref();
///     assert_eq!(deref.test(), 2);
/// }
/// ```
#[macro_export]
macro_rules! trait_enum {
    (
        $(#[$outer:meta])*
        pub enum $EnumName:ident: $Trait:tt {
            $(
                $name:ident,
            )+
        }
    ) => {
        __trait_enum! {
            $(#[$outer])*
            (pub) $EnumName: $Trait {
                $(
                    $name,
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        enum $EnumName:ident: $Trait:tt {
            $(
                $name:ident,
            )+
        }
    ) => {
        __trait_enum! {
            $(#[$outer])*
            () $EnumName: $Trait {
                $(
                    $name,
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        pub ($($vis:tt)+) struct $EnumName:ident: $Trait:tt {
            $(
                $name:ident,
            )+
        }
    ) => {
        __trait_enum! {
            $(#[$outer])*
            (pub ($($vis)+)) $EnumName: $Trait {
                $(
                    $name,
                )+
            }
        }
    };

    (
        $(#[$outer:meta])*
        pub enum $EnumName:ident: $Trait:tt {
            $(
                $name:ident
            ),+
        }
    ) => {
        __trait_enum! {
            $(#[$outer])*
            (pub) $EnumName: $Trait {
                $(
                    $name,
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        enum $EnumName:ident: $Trait:tt {
            $(
                $name:ident
            ),+
        }
    ) => {
        __trait_enum! {
            $(#[$outer])*
            () $EnumName: $Trait {
                $(
                    $name,
                )+
            }
        }
    };
    (
        $(#[$outer:meta])*
        pub ($($vis:tt)+) struct $EnumName:ident: $Trait:tt {
            $(
                $name:ident
            ),+
        }
    ) => {
        __trait_enum! {
            $(#[$outer])*
            (pub ($($vis)+)) $EnumName: $Trait {
                $(
                    $name,
                )+
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __trait_enum {
    (
        $(#[$outer:meta])*
        ($($vis:tt)*) $EnumName:ident: $Trait:tt {
            $(
                $name:ident,
            )+
        }
    ) => {
        $(#[$outer])*
        $($vis)* enum $EnumName {
            $(
                $name($name),
            )*
        }
        #[cfg(feature = "std")]
        use std::ops::Deref;
        #[cfg(not(feature = "std"))]
        use core::ops::Deref;

        impl Deref for $EnumName {
            type Target = $Trait;

            fn deref(&self) -> &($Trait + 'static) {
                match *self {
                    $(
                        $EnumName::$name(ref v) => v as &$Trait,
                    )*
                }
            }
        }
    };
}

#[cfg(test)]
pub mod test;

//! A grab-bag of functional tools.
#![no_std]
#![cfg_attr(feature = "unstable", feature(conservative_impl_trait))]


#[macro_use]
#[cfg(feature = "use_std")]
extern crate std;

macro_rules! if_std {
    ($($i:item)*) => ($(
        #[cfg(feature = "use_std")]
        $i
    )*)
}

#[cfg(feature = "unstable")]
pub mod functor;

pub mod sequence;
pub mod empty;
pub mod monad;
pub mod misc;

/// Useful functions to glob import.
///
/// If you don't want all of them, you can also import the prelude of an
/// individual module.
pub mod prelude {
    #[cfg(feature = "unstable")]
    #[doc(inline)]
    pub use functor::prelude::*;
    #[doc(inline)]
    pub use sequence::prelude::*;
    #[doc(inline)]
    pub use empty::prelude::*;
    #[doc(inline)]
    pub use monad::prelude::*;
    #[doc(inline)]
    pub use misc::{default, id};
}

#[doc(inline)]
pub use prelude::*;

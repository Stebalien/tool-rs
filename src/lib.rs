//! A grab-bag of functional tools.
#![no_std]

#[macro_use]
#[cfg(feature = "use_std")]
extern crate std;

macro_rules! if_std {
    ($($i:item)*) => ($(
        #[cfg(feature = "use_std")]
        $i
    )*)
}

pub mod empty;
pub mod functor;
pub mod misc;
pub mod monad;
pub mod sequence;

/// Useful functions to glob import.
///
/// If you don't want all of them, you can also import the prelude of an
/// individual module.
pub mod prelude {
    #[doc(inline)]
    pub use empty::prelude::*;
    #[doc(inline)]
    pub use functor::prelude::*;
    #[doc(inline)]
    pub use misc::{default, id};
    #[doc(inline)]
    pub use monad::prelude::*;
    #[doc(inline)]
    pub use sequence::prelude::*;
}

#[doc(inline)]
pub use prelude::*;

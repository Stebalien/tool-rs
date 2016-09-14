//! A bunch of traits for working with sequences.

/// Useful functions exported by `tool::sequence`.
pub mod prelude {
    pub use super::{first, second, third, flip};
}

/// A sequence with no items.
///
/// If you ever find a use for this, please submit a PR with an example.
pub trait Empty {}

/// A sequence with one item.
pub trait Singleton: First {}

/// A sequence with two items.
pub trait Pair: First + Second {}

/// A sequence with three items.
pub trait Triple: First + Second + Third {}

/// A sequence with a first item.
pub trait First {
    type First;
    fn first(self) -> Self::First;
}

/// A sequence with a second item.
pub trait Second {
    type Second;
    fn second(self) -> Self::Second;
}

/// A sequence with a third item.
pub trait Third {
    type Third;
    fn third(self) -> Self::Third;
}

/// Get the first element of a sequence with at least one element.
pub fn first<P: First>(seq: P) -> P::First {
    seq.first()
}

/// Get the second element of a sequence with at least to elements
pub fn second<P: Second>(seq: P) -> P::Second {
    seq.second()
}

/// Get the third element of a sequence with at least three elements.
pub fn third<P: Third>(seq: P) -> P::Third {
    seq.third()
}

// TODO: Generic?
/// Flip a length two tuple.
pub fn flip<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

macro_rules! one {
    ($($t:tt)*) => {1}
}
macro_rules! count {
    ($($t:ident),*) => {
        $(one!($t) +)* 0
    }
}

macro_rules! call {
    ($mac:ident) => {
        $mac!{A, B, C, D, E, F, G, H, I, J,}
    }
}

macro_rules! impl_first {
    () => {};
    ($F:ident, $($A:ident,)*) => {
        impl_first!{$($A,)*}
        impl<$F, $($A),*> First for ($F, $($A),*) {
            type First = $F;
            fn first(self) -> Self::First {
                self.0
            }
        }

        impl<'a, $F, $($A),*> First for &'a ($F, $($A),*) {
            type First = &'a $F;
            fn first(self) -> Self::First {
                &self.0
            }
        }

        impl<'a, $F, $($A),*> First for &'a mut ($F, $($A),*) {
            type First = &'a mut $F;
            fn first(self) -> Self::First {
                &mut self.0
            }
        }
        impl<'a, $F> First for &'a [$F; count!($F $(,$A)*)] {
            type First = &'a $F;
            fn first(self) -> Self::First {
                &self[0]
            }
        }

        impl<'a, $F> First for &'a mut [$F; count!($F $(,$A)*)] {
            type First = &'a mut $F;
            fn first(self) -> Self::First {
                &mut self[0]
            }
        }
    }
}

macro_rules! impl_second {
    ($F:ident,) => {};
    ($F:ident, $S:ident, $($A:ident,)*) => {
        impl_second!{$S, $($A,)*}
        impl<$F, $S, $($A),*> Second for ($F, $S, $($A),*) {
            type Second = $S;
            fn second(self) -> Self::Second {
                self.1
            }
        }

        impl<'a, $F, $S, $($A),*> Second for &'a ($F, $S, $($A),*) {
            type Second = &'a $S;
            fn second(self) -> Self::Second {
                &self.1
            }
        }

        impl<'a, $F, $S, $($A),*> Second for &'a mut ($F, $S, $($A),*) {
            type Second = &'a mut $S;
            fn second(self) -> Self::Second {
                &mut self.1
            }
        }
        impl<'a, $S> Second for &'a [$S; count!($F, $S $(,$A)*)] {
            type Second = &'a $S;
            fn second(self) -> Self::Second {
                &self[1]
            }
        }

        impl<'a, $S> Second for &'a mut [$S; count!($F, $S $(,$A)*)] {
            type Second = &'a mut $S;
            fn second(self) -> Self::Second {
                &mut self[1]
            }
        }
    }
}

macro_rules! impl_third {
    ($F:ident, $S:ident,) => {};
    ($F:ident, $S:ident, $T:ident, $($A:ident,)*) => {
        impl_third!{$S, $T, $($A,)*}
        impl<$F, $S, $T, $($A),*> Third for ($F, $S, $T, $($A),*) {
            type Third = $T;
            fn third(self) -> Self::Third {
                self.2
            }
        }

        impl<'a, $F, $S, $T, $($A),*> Third for &'a ($F, $S, $T, $($A),*) {
            type Third = &'a $T;
            fn third(self) -> Self::Third {
                &self.2
            }
        }

        impl<'a, $F, $S, $T, $($A),*> Third for &'a mut ($F, $S, $T, $($A),*) {
            type Third = &'a mut $T;
            fn third(self) -> Self::Third {
                &mut self.2
            }
        }
        impl<'a, $T> Third for &'a [$T; count!($F, $S, $T $(,$A)*)] {
            type Third = &'a $T;
            fn third(self) -> Self::Third {
                &self[2]
            }
        }

        impl<'a, $T> Third for &'a mut [$T; count!($F, $S, $T $(,$A)*)] {
            type Third = &'a mut $T;
            fn third(self) -> Self::Third {
                &mut self[2]
            }
        }
    }
}
call!(impl_first);
call!(impl_second);
call!(impl_third);

// Slice

impl<'a, A> First for &'a [A] {
    type First = Option<&'a A>;
    fn first(self) -> Self::First {
        self.get(0)
    }
}

impl<'a, A> Second for &'a [A] {
    type Second = Option<&'a A>;
    fn second(self) -> Self::Second {
        self.get(1)
    }
}

impl<'a, A> Third for &'a [A] {
    type Third = Option<&'a A>;
    fn third(self) -> Self::Third {
        self.get(2)
    }
}

// Mut Slice

impl<'a, A> First for &'a mut [A] {
    type First = Option<&'a mut A>;
    fn first(self) -> Self::First {
        self.get_mut(0)
    }
}

impl<'a, A> Second for &'a mut [A] {
    type Second = Option<&'a mut A>;
    fn second(self) -> Self::Second {
        self.get_mut(1)
    }
}

impl<'a, A> Third for &'a mut [A] {
    type Third = Option<&'a mut A>;
    fn third(self) -> Self::Third {
        self.get_mut(2)
    }
}

// Sequence impls...
impl Empty for () {}
impl<'a> Empty for &'a () {}
impl<'a> Empty for &'a mut () {}
impl<'a, T> Empty for &'a [T; 0] {}
impl<'a, T> Empty for &'a mut [T; 0] {}

impl<A> Singleton for (A,) {}
impl<'a, A> Singleton for &'a (A,) {}
impl<'a, A> Singleton for &'a mut (A,) {}
impl<'a, A> Singleton for &'a [A; 1] {}
impl<'a, A> Singleton for &'a mut [A; 1] {}

impl<A, B> Pair for (A, B) {}
impl<'a, A, B> Pair for &'a (A, B) {}
impl<'a, A, B> Pair for &'a mut (A, B) {}
impl<'a, A> Pair for &'a [A; 2] {}
impl<'a, A> Pair for &'a mut [A; 2] {}

impl<A, B, C> Triple for (A, B, C) {}
impl<'a, A, B, C> Triple for &'a (A, B, C) {}
impl<'a, A, B, C> Triple for &'a mut (A, B, C) {}
impl<'a, A> Triple for &'a [A; 3] {}
impl<'a, A> Triple for &'a mut [A; 3] {}


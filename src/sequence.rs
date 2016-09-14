//! A bunch of traits and functions for working with sequences.

/// Useful functions exported by `tool::sequence`.
pub mod prelude {
    #[doc(inline)]
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

/// A sequence that can be represented as a cons cell.
pub trait Cons {
    type Head;
    type Tail;
    fn uncons(self) -> (Self::Head, Self::Tail);
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

/// Split a sequence of at least one item into a head and tail.
pub fn uncons<L: Cons>(list: L) -> (L::Head, L::Tail) {
    list.uncons()
}

// TODO: Generic?
/// Flip a length two tuple.
pub fn flip<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

impl<'a, A> Cons for &'a [A] {
    type Head = Option<&'a A>;
    type Tail = &'a [A];
    fn uncons(self) -> (Self::Head, Self::Tail) {
        if self.len() > 0 {
            let (a, b) = self.split_at(1);
            (Some(&a[0]), b)
        } else {
            (None, &[])
        }
    }
}

impl<'a, A> Cons for &'a mut [A] {
    type Head = Option<&'a mut A>;
    type Tail = &'a mut [A];
    fn uncons(self) -> (Self::Head, Self::Tail) {
        if self.len() > 0 {
            let (a, b) = self.split_at_mut(1);
            (Some(&mut a[0]), b)
        } else {
            (None, &mut [])
        }
    }
}

macro_rules! one {
    ($($t:tt)*) => {1}
}

macro_rules! expr {
    ($e:expr) => {$e}
}
macro_rules! count {
    ($($t:ident),*) => {
        $(one!($t) +)* 0
    }
}

macro_rules! impl_list {
    () => {};
    ($Fi:tt $Ft:ident, $($Ai:tt $At:ident,)*) => {
        impl<$Ft, $($At),*> Cons for ($Ft, $($At),*) {
            type Head = $Ft;
            type Tail = ($($At,)*);
            fn uncons(self) -> (Self::Head, Self::Tail) {
                expr!((self.$Fi, ($(self.$Ai,)*)))
            }
        }

        impl<'a, $Ft, $($At),*> Cons for &'a ($Ft, $($At),*) {
            type Head = &'a $Ft;
            type Tail = ($(&'a $At,)*);
            fn uncons(self) -> (Self::Head, Self::Tail) {
                expr!((&self.$Fi, ($(&self.$Ai,)*)))
            }
        }

        impl<'a, $Ft, $($At),*> Cons for &'a mut ($Ft, $($At),*) {
            type Head = &'a mut $Ft;
            type Tail = ($(&'a mut $At,)*);
            #[inline(always)]
            fn uncons(self) -> (Self::Head, Self::Tail) {
                expr!((&mut self.$Fi, ($(&mut self.$Ai,)*)))
            }
        }

        impl<'a, $Ft> Cons for &'a [$Ft; count!($Ft $(,$At)*)] {
            type Head = &'a $Ft;
            type Tail = &'a [$Ft; count!($($At),*)];
            fn uncons(self) -> (Self::Head, Self::Tail) {
                unsafe {
                    let ptr = self.as_ptr();
                    let a = &*ptr;
                    let b = &*(ptr.offset(1) as *const [$Ft; count!($($At),*)]);
                    (a, b)
                }
            }
        }

        impl<'a, $Ft> Cons for &'a mut [$Ft; count!($Ft $(,$At)*)] {
            type Head = &'a mut $Ft;
            type Tail = &'a mut [$Ft; count!($($At),*)];
            fn uncons(self) -> (Self::Head, Self::Tail) {
                unsafe {
                    let ptr = self.as_mut_ptr();
                    let a = &mut *ptr;
                    let b = &mut *(ptr.offset(1) as *mut [$Ft; count!($($At),*)]);
                    (a, b)
                }
            }
        }
    }
}

impl_list!{0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J,}
impl_list!{0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I,}
impl_list!{0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H,}
impl_list!{0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G,}
impl_list!{0 A, 1 B, 2 C, 3 D, 4 E, 5 F,}
impl_list!{0 A, 1 B, 2 C, 3 D, 4 E,}
impl_list!{0 A, 1 B, 2 C, 3 D,}
impl_list!{0 A, 1 B, 2 C,}
impl_list!{0 A, 1 B,}
impl_list!{0 A,}

impl<T> First for T where T: Cons {
    type First = T::Head;
    fn first(self) -> Self::First {
        self.uncons().0
    }
}

impl<T> Second for T
    where T: Cons,
          T::Tail: Cons
{
    type Second = <T::Tail as Cons>::Head;
    fn second(self) -> Self::Second {
        self.uncons().1.uncons().0
    }
}

impl<T> Third for T
    where T: Cons,
          T::Tail: Cons,
          <T::Tail as Cons>::Tail: Cons,
{
    type Third = <<T::Tail as Cons>::Tail as Cons>::Head;
    fn third(self) -> Self::Third {
        self.uncons().1.uncons().1.uncons().0
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


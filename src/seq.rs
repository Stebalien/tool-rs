pub trait Singleton {
    type First;
    fn first(self) -> Self::First;
}

pub trait Pair: Singleton {
    type Second;
    fn second(self) -> Self::Second;
}

pub trait Triple: Pair {
    type Third;
    fn third(self) -> Self::Third;
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
        impl<$F, $($A),*> Singleton for ($F, $($A),*) {
            type First = $F;
            fn first(self) -> Self::First {
                self.0
            }
        }

        impl<'a, $F, $($A),*> Singleton for &'a ($F, $($A),*) {
            type First = &'a $F;
            fn first(self) -> Self::First {
                &self.0
            }
        }

        impl<'a, $F, $($A),*> Singleton for &'a mut ($F, $($A),*) {
            type First = &'a mut $F;
            fn first(self) -> Self::First {
                &mut self.0
            }
        }
        impl<'a, $F> Singleton for &'a [$F; count!($F $(,$A)*)] {
            type First = &'a $F;
            fn first(self) -> Self::First {
                &self[0]
            }
        }

        impl<'a, $F> Singleton for &'a mut [$F; count!($F $(,$A)*)] {
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
        impl<$F, $S, $($A),*> Pair for ($F, $S, $($A),*) {
            type Second = $S;
            fn second(self) -> Self::Second {
                self.1
            }
        }

        impl<'a, $F, $S, $($A),*> Pair for &'a ($F, $S, $($A),*) {
            type Second = &'a $S;
            fn second(self) -> Self::Second {
                &self.1
            }
        }

        impl<'a, $F, $S, $($A),*> Pair for &'a mut ($F, $S, $($A),*) {
            type Second = &'a mut $S;
            fn second(self) -> Self::Second {
                &mut self.1
            }
        }
        impl<'a, $S> Pair for &'a [$S; count!($F, $S $(,$A)*)] {
            type Second = &'a $S;
            fn second(self) -> Self::Second {
                &self[1]
            }
        }

        impl<'a, $S> Pair for &'a mut [$S; count!($F, $S $(,$A)*)] {
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
        impl<$F, $S, $T, $($A),*> Triple for ($F, $S, $T, $($A),*) {
            type Third = $T;
            fn third(self) -> Self::Third {
                self.2
            }
        }

        impl<'a, $F, $S, $T, $($A),*> Triple for &'a ($F, $S, $T, $($A),*) {
            type Third = &'a $T;
            fn third(self) -> Self::Third {
                &self.2
            }
        }

        impl<'a, $F, $S, $T, $($A),*> Triple for &'a mut ($F, $S, $T, $($A),*) {
            type Third = &'a mut $T;
            fn third(self) -> Self::Third {
                &mut self.2
            }
        }
        impl<'a, $T> Triple for &'a [$T; count!($F, $S, $T $(,$A)*)] {
            type Third = &'a $T;
            fn third(self) -> Self::Third {
                &self[2]
            }
        }

        impl<'a, $T> Triple for &'a mut [$T; count!($F, $S, $T $(,$A)*)] {
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

impl<'a, A> Singleton for &'a [A] {
    type First = Option<&'a A>;
    fn first(self) -> Self::First {
        self.get(0)
    }
}

impl<'a, A> Pair for &'a [A] {
    type Second = Option<&'a A>;
    fn second(self) -> Self::Second {
        self.get(1)
    }
}

impl<'a, A> Triple for &'a [A] {
    type Third = Option<&'a A>;
    fn third(self) -> Self::Third {
        self.get(2)
    }
}

// Mut Slice

impl<'a, A> Singleton for &'a mut [A] {
    type First = Option<&'a mut A>;
    fn first(self) -> Self::First {
        self.get_mut(0)
    }
}

impl<'a, A> Pair for &'a mut [A] {
    type Second = Option<&'a mut A>;
    fn second(self) -> Self::Second {
        self.get_mut(1)
    }
}

impl<'a, A> Triple for &'a mut [A] {
    type Third = Option<&'a mut A>;
    fn third(self) -> Self::Third {
        self.get_mut(2)
    }
}

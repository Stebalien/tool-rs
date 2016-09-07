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

// Normal

impl<A> Singleton for (A,) {
    type First = A;
    fn first(self) -> Self::First {
        self.0
    }
}

impl<A, B> Singleton for (A, B) {
    type First = A;
    fn first(self) -> Self::First {
        self.0
    }
}

impl<A, B, C> Singleton for (A, B, C) {
    type First = A;
    fn first(self) -> Self::First {
        self.0
    }
}

impl<A, B> Pair for (A, B) {
    type Second = B;
    fn second(self) -> Self::Second {
        self.1
    }
}


impl<A, B, C> Pair for (A, B, C) {
    type Second = B;
    fn second(self) -> Self::Second {
        self.1
    }
}

impl<A, B, C> Triple for (A, B, C) {
    type Third = C;
    fn third(self) -> Self::Third {
        self.2
    }
}


// By Ref

impl<'a, A> Singleton for &'a (A,) {
    type First = &'a A;
    fn first(self) -> Self::First {
        &self.0
    }
}

impl<'a, A, B> Singleton for &'a (A, B) {
    type First = &'a A;
    fn first(self) -> Self::First {
        &self.0
    }
}

impl<'a, A, B, C> Singleton for &'a (A, B, C) {
    type First = &'a A;
    fn first(self) -> Self::First {
        &self.0
    }
}

impl<'a, A, B> Pair for &'a (A, B) {
    type Second = &'a B;
    fn second(self) -> Self::Second {
        &self.1
    }
}

impl<'a, A, B, C> Pair for &'a (A, B, C) {
    type Second = &'a B;
    fn second(self) -> Self::Second {
        &self.1
    }
}

impl<'a, A, B, C> Triple for &'a (A, B, C) {
    type Third = &'a C;
    fn third(self) -> Self::Third {
        &self.2
    }
}

// Tuple by mut ref

impl<'a, A> Singleton for &'a mut (A,) {
    type First = &'a mut A;
    fn first(self) -> Self::First {
        &mut self.0
    }
}

impl<'a, A, B> Singleton for &'a mut (A, B) {
    type First = &'a A;
    fn first(self) -> Self::First {
        &mut self.0
    }
}

impl<'a, A, B, C> Singleton for &'a mut (A, B, C) {
    type First = &'a mut A;
    fn first(self) -> Self::First {
        &mut self.0
    }
}

impl<'a, A, B> Pair for &'a mut (A, B) {
    type Second = &'a mut B;
    fn second(self) -> Self::Second {
        &mut self.1
    }
}

impl<'a, A, B, C> Pair for &'a mut (A, B, C) {
    type Second = &'a B;
    fn second(self) -> Self::Second {
        &mut self.1
    }
}

impl<'a, A, B, C> Triple for &'a mut (A, B, C) {
    type Third = &'a mut C;
    fn third(self) -> Self::Third {
        &mut self.2
    }
}

// Array

// TODO (drop bleh...)

// Array Ref

impl<'a, A> Singleton for &'a [A; 1] {
    type First = &'a A;
    fn first(self) -> Self::First {
        &self[0]
    }
}

impl<'a, A> Singleton for &'a [A; 2] {
    type First = &'a A;
    fn first(self) -> Self::First {
        &self[0]
    }
}

impl<'a, A> Singleton for &'a [A; 3] {
    type First = &'a A;
    fn first(self) -> Self::First {
        &self[0]
    }
}

impl<'a, A> Pair for &'a [A; 2] {
    type Second = &'a A;
    fn second(self) -> Self::Second {
        &self[1]
    }
}

impl<'a, A> Pair for &'a [A; 3] {
    type Second = &'a A;
    fn second(self) -> Self::Second {
        &self[1]
    }
}

impl<'a, A> Triple for &'a [A; 3] {
    type Third = &'a A;
    fn third(self) -> Self::Third {
        &self[2]
    }
}

// Array by mut ref

impl<'a, A> Singleton for &'a mut [A; 1] {
    type First = &'a mut A;
    fn first(self) -> Self::First {
        &mut self[0]
    }
}

impl<'a, A> Singleton for &'a mut [A; 2] {
    type First = &'a mut A;
    fn first(self) -> Self::First {
        &mut self[0]
    }
}

impl<'a, A> Singleton for &'a mut [A; 3] {
    type First = &'a mut A;
    fn first(self) -> Self::First {
        &mut self[0]
    }
}

impl<'a, A> Pair for &'a mut [A; 2] {
    type Second = &'a mut A;
    fn second(self) -> Self::Second {
        &mut self[1]
    }
}

impl<'a, A> Pair for &'a mut [A; 3] {
    type Second = &'a mut A;
    fn second(self) -> Self::Second {
        &mut self[1]
    }
}

impl<'a, A> Triple for &'a mut [A; 3] {
    type Third = &'a mut A;
    fn third(self) -> Self::Third {
        &mut self[2]
    }
}

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

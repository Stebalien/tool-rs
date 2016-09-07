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

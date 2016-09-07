#![no_std]

pub mod seq;

/// The identity function. Literally, just returns the passed-in value.
pub fn id<T>(value: T) -> T { value }

/// Return a default value.
pub fn default<T: Default>() -> T {
    Default::default()
}

/// Get the first element of a sequence with at least one element.
pub fn first<P: seq::Singleton>(seq: P) -> P::First {
    seq.first()
}

/// Get the second element of a sequence with at least to elements
pub fn second<P: seq::Pair>(seq: P) -> P::Second {
    seq.second()
}

/// Get the third element of a sequence with at least three elements.
pub fn third<P: seq::Triple>(seq: P) -> P::Third {
    seq.third()
}

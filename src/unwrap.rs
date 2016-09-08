use core::fmt::Debug;

/// Abstracts over unwrapping.
pub trait Unwrap {
    type Value;
    fn unwrap(self) -> Self::Value;
}

impl<T> Unwrap for Option<T> {
    type Value = T;
    fn unwrap(self) -> Self::Value {
        Option::unwrap(self)
    }
}

impl<T, E: Debug> Unwrap for Result<T, E> {
    type Value = T;
    fn unwrap(self) -> Self::Value {
        Result::unwrap(self)
    }
}

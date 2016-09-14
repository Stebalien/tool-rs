use core::fmt::Debug;

/// Useful functions exported by `tool::monad`.
pub mod prelude {
    pub use super::{ok, unwrap};
}

/// Abstracts over unwrapping.
pub trait Unwrap {
    type Value;
    fn unwrap(self) -> Self::Value;
}

/// Types of values that can be "Ok".
///
/// Basically, any type with a success variant and some number of non-success variants.
pub trait Ok {
    type Value;
    fn ok(self) -> Option<Self::Value>;
}

/// Converts `Result`-like values into `Option`s.
///
/// ```rust
/// use tool::ok;
/// let filtered: Vec<_> = vec![Ok(1), Err("bad")].into_iter().filter_map(ok).collect();
/// assert_eq!(filtered, vec![1]);
/// ```
pub fn ok<R: Ok>(result: R) -> Option<R::Value> {
    result.ok()
}

/// Unwraps the value.
///
/// ```rust
/// use tool::unwrap;
/// let unwrapped: Vec<_> = vec![Result::Ok::<i32, ()>(1)].into_iter().map(unwrap).collect();
/// assert_eq!(unwrapped, vec![1]);
/// let unwrapped2: Vec<_> = vec![Some(2)].into_iter().map(unwrap).collect();
/// assert_eq!(unwrapped2, vec![2]);
/// ```
pub fn unwrap<U: Unwrap>(unwrappable: U) -> U::Value {
    unwrappable.unwrap()
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

impl<T> Ok for Option<T> {
    type Value = T;
    fn ok(self) -> Self {
        self
    }
}

impl<T, E> Ok for Result<T, E> {
    type Value = T;
    fn ok(self) -> Option<T> {
        self.ok()
    }
}

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

pub mod seq;
pub mod empty;
pub mod unwrap;

/// The identity function.
///
/// Literally, just returns the passed-in value.
pub fn id<T>(value: T) -> T { value }

/// Return a default value.
pub fn default<T: Default>() -> T {
    Default::default()
}

/// Get the first element of a sequence with at least one element.
pub fn first<P: seq::First>(seq: P) -> P::First {
    seq.first()
}

/// Get the second element of a sequence with at least to elements
pub fn second<P: seq::Second>(seq: P) -> P::Second {
    seq.second()
}

/// Get the third element of a sequence with at least three elements.
pub fn third<P: seq::Third>(seq: P) -> P::Third {
    seq.third()
}

/// True if the value is "empty"
///
/// For example: `[]`, `""`, `Some([])`, `Ok([])`, `None::<T: Empty>`, etc...
pub fn empty<T: empty::IsEmpty>(value: &T) -> bool {
    value.is_empty()
}

/// False if the value is "empty"
///
/// Shortcut for `|x|{ !empty(x) }`.
///
/// Example:
///
/// ```rust
/// use tool::non_empty;
/// let strings: Vec<_> = vec!["my string", "", "asdf", ""]
///     .into_iter()
///     .filter(non_empty)
///     .collect();
/// assert_eq!(strings, vec!["my string", "asdf"]);
/// ```
pub fn non_empty<T: empty::IsEmpty>(value: &T) -> bool {
    !value.is_empty()
}

/// Same as `Result::ok`. Useful for mapping.
///
/// If you want to do the same thing with `Option`, just use `tool::id`.
///
/// ```rust
/// use tool::ok;
/// let filtered: Vec<_> = vec![Ok(1), Err("bad")].into_iter().filter_map(ok).collect();
/// assert_eq!(filtered, vec![1]);
/// ```
pub fn ok<T, E>(result: Result<T, E>) -> Option<T> {
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
pub fn unwrap<U: unwrap::Unwrap>(unwrappable: U) -> U::Value {
    unwrappable.unwrap()
}


// TODO: Generic?
/// Flip a length two tuple.
pub fn flip<A, B>((a, b): (A, B)) -> (B, A) {
    (b, a)
}

#[test]
fn test_seq() {
    assert_eq!(first((1, 2)), 1);
    assert_eq!(first(&(1, 2)), &1);
    assert_eq!(second((1, 2)), 2);
    assert_eq!(third(&[] as &[i32]), None);
    assert_eq!(second(&[1,2][..]), Some(&2));
    assert_eq!(second(&[1,2]), &2);
}

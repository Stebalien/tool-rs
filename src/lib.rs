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

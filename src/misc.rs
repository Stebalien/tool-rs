//! Miscellaneous functions that don't really have a home.

/// The identity function.
///
/// Literally, just returns the passed-in value.
pub const fn id<T>(value: T) -> T {
    value
}

/// Return a default value.
pub fn default<T: Default>() -> T {
    Default::default()
}

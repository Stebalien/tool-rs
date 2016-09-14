/// The identity function.
///
/// Literally, just returns the passed-in value.
pub fn id<T>(value: T) -> T { value }

/// Return a default value.
pub fn default<T: Default>() -> T {
    Default::default()
}

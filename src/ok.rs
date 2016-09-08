/// Types of values that can be "Ok".
///
/// Basically, any type with a success variant and some number of non-success variants.
pub trait Ok {
    type Value;
    fn ok(self) -> Option<Self::Value>;
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

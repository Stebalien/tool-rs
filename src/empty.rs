/// Things that can be "is_empty".
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl<'a, T: IsEmpty + ?Sized> IsEmpty for &'a T {
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}

impl<'a, T: IsEmpty + ?Sized> IsEmpty for &'a mut T {
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}

impl<T: IsEmpty> IsEmpty for Option<T> {
    fn is_empty(&self) -> bool {
        self.as_ref().map(|x| x.is_empty()).unwrap_or(true)
    }
}

impl<T: IsEmpty, E> IsEmpty for Result<T, E> {
    fn is_empty(&self) -> bool {
        self.as_ref().map(|x| x.is_empty()).unwrap_or(true)
    }
}

impl IsEmpty for str {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsEmpty for [T] {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

if_std! {
    use std::prelude::v1::*;
    use std::ffi::{OsStr, OsString, CString, CStr};
    use std::collections::{HashMap, BTreeMap};
    use core::hash::{Hash, BuildHasher};
    use std::rc::Rc;
    use std::sync::Arc;

    impl<K, V, S> IsEmpty for HashMap<K, V, S>
        where K: Eq + Hash,
              S: BuildHasher,
    {
        fn is_empty(&self) -> bool {
            HashMap::is_empty(self)
        }
    }

    impl<K, V> IsEmpty for BTreeMap<K, V> {
        fn is_empty(&self) -> bool {
            BTreeMap::is_empty(self)
        }
    }

    impl<T> IsEmpty for Vec<T> {
        fn is_empty(&self) -> bool {
            Vec::is_empty(self)
        }
    }

    impl IsEmpty for String {
        fn is_empty(&self) -> bool {
            str::is_empty(self)
        }
    }

    impl IsEmpty for OsStr {
        fn is_empty(&self) -> bool {
            OsStr::is_empty(self)
        }
    }

    impl IsEmpty for OsString {
        fn is_empty(&self) -> bool {
            OsStr::is_empty(self)
        }
    }

    impl IsEmpty for CStr {
        fn is_empty(&self) -> bool {
            unsafe {
                *self.as_ptr() == 0
            }
        }
    }

    impl IsEmpty for CString {
        fn is_empty(&self) -> bool {
            (&**self).is_empty()
        }
    }

    impl<T: ?Sized + IsEmpty> IsEmpty for Box<T> {
        fn is_empty(&self) -> bool {
            (&**self).is_empty()
        }
    }

    impl<T: ?Sized + IsEmpty> IsEmpty for Rc<T> {
        fn is_empty(&self) -> bool {
            (&**self).is_empty()
        }
    }

    impl<T: ?Sized + IsEmpty> IsEmpty for Arc<T> {
        fn is_empty(&self) -> bool {
            (&**self).is_empty()
        }
    }
}

/// Things that can be "empty".
pub trait Empty {
    fn empty(&self) -> bool;
}

impl<'a, T: Empty + ?Sized> Empty for &'a T {
    fn empty(&self) -> bool {
        (**self).empty()
    }
}

impl<'a, T: Empty + ?Sized> Empty for &'a mut T {
    fn empty(&self) -> bool {
        (**self).empty()
    }
}

impl<T: Empty> Empty for Option<T> {
    fn empty(&self) -> bool {
        self.as_ref().map(|x| x.empty()).unwrap_or(true)
    }
}

impl<T: Empty, E> Empty for Result<T, E> {
    fn empty(&self) -> bool {
        self.as_ref().map(|x| x.empty()).unwrap_or(true)
    }
}

impl Empty for str {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Empty for [T] {
    fn empty(&self) -> bool {
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

    impl<K, V, S> Empty for HashMap<K, V, S>
        where K: Eq + Hash,
              S: BuildHasher,
    {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl<K, V> Empty for BTreeMap<K, V> {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl<T> Empty for Vec<T> {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl Empty for String {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl Empty for OsStr {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl Empty for OsString {
        fn empty(&self) -> bool {
            self.is_empty()
        }
    }

    impl Empty for CStr {
        fn empty(&self) -> bool {
            unsafe {
                *self.as_ptr() == 0
            }
        }
    }

    impl Empty for CString {
        fn empty(&self) -> bool {
            (&**self).empty()
        }
    }

    impl<T: ?Sized + Empty> Empty for Box<T> {
        fn empty(&self) -> bool {
            (&**self).empty()
        }
    }

    impl<T: ?Sized + Empty> Empty for Rc<T> {
        fn empty(&self) -> bool {
            (&**self).empty()
        }
    }

    impl<T: ?Sized + Empty> Empty for Arc<T> {
        fn empty(&self) -> bool {
            (&**self).empty()
        }
    }
}

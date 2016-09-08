/// Types that are "truthy"
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

//macro_rules! impl_primitive {
//    ($($typ:ty),*) => {
//        $(
//            impl Truthy for $typ {
//                fn truthy(&self) -> bool {
//                    *self != 0
//                }
//            }
//        )*
//    }
//}
//impl_primitive!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

//! Higher-order functions (functions that operate on functions)

pub mod prelude {
    pub use super::compose;
}

/// Compose two functions.
///
/// Takes functions `f` and `g` and returns `f ∘ g = |a: A| f(g(a))`.
#[cfg(feature = "unstable")]
pub fn compose<A, B, C, F, G>(f: F,  g: G) -> impl Fn(A) -> C
    where G: Fn(A) -> B,
          F: Fn(B) -> C,
{
    move |a: A| { f(g(a)) }
}

// TODO: Someday
// pub struct Uncurry<F>(F);
//
// impl<T, F> FnOnce<(T,)> for Uncurry<F>
//     where F: FnOnce<T>
// {
//     type Output = F::Output;
//
//     extern "rust-call" fn call_once(self, (args,): (T,)) -> F::Output {
//         self.0.call_once(args)
//     }
// }
//
// impl<T, F> FnMut<(T,)> for Uncurry<F>
//     where F: FnMut<T>
// {
//     extern "rust-call" fn call_mut(&mut self, (args,): (T,)) -> F::Output {
//         self.0.call_mut(args)
//     }
// }
//
// impl<T, F> Fn<(T,)> for Uncurry<F>
//     where F: Fn<T>
// {
//     extern "rust-call" fn call(&self, (args,): (T,)) -> F::Output {
//         self.0.call(args)
//     }
// }
//
//
// pub fn uncurry<A, F>(f: F) -> Uncurry<F> {
//     Uncurry(f)
// }

//! Higher-order functions (functions that operate on functions)

/// Useful functions exported by `tool::functor`.
pub mod prelude {
    #[doc(inline)]
    pub use super::{compose, fix, flip};
}

/// Compose two functions.
///
/// Takes functions `f` and `g` and returns `f ∘ g = |a: A| f(g(a))`.
pub fn compose<A, B, C, F, G>(f: F,  g: G) -> impl Fn(A) -> C
    where G: Fn(A) -> B,
          F: Fn(B) -> C,
{
    move |a: A| { f(g(a)) }
}

/// Flip the argument order of a two-parameter function.
///
/// Specifically, `flip(f: Fn(a: A, b: B) -> C) = |b: B, a: A| f(a, b)`.
pub fn flip<F, A, B, R>(f: F) -> impl Fn(B, A) -> R
    where F: Fn(A, B) -> R
{
    move |a, b| f(b, a)
}

/// A Y-Combinator.
///
/// Takes a function `f` and returns a fixpoint of `f`.
///
/// In English, this allows you to define a recursive closure, something that's
/// normally quite hard to do in rust. Rather than try to explain it, here's an
/// illustration that should speak for itself (as long as you know the recursive
/// definition of Fibonacci numbers).
///
/// ```rust
/// use tool::prelude::*;
///
/// let fib = fix(|f, x| {
///     if x == 0 || x == 1 {
///         x
///     } else {
///         // `f` is `fib`
///         f(x - 1) + f(x - 2)
///     }
/// });
/// assert_eq!(55, fib(10));
/// ```
pub fn fix<A, B, F>(f: F) -> impl Fn(A) -> B
    where F: Fn(&Fn(A)-> B, A) -> B
{
    use std::cell::Cell;

    move |a: A| {
        // Hopefully optimized away. Can probably use some unsafe magic to help the optimizer...
        let tmp_fn = |_: A| -> B { panic!("Hmm... not good.") };
        let (fun_holder, fun);
        fun_holder = Cell::new(&tmp_fn as &Fn(A) -> B);
        fun = |ai: A| { f(fun_holder.get(), ai) };
        fun_holder.set(&fun as &Fn(A) -> B);
        f(fun_holder.get(), a)
    }
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

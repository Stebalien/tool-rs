//! Higher-order functions (functions that operate on functions)

/// Compose two functions.
///
/// Takes functions `f` and `g` and returns `f ∘ g = |a: A| f(g(a))`.
pub fn compose<A, B, C, F, G>(mut f: F, mut g: G) -> impl FnMut(A) -> C
    where G: FnMut(A) -> B,
          F: FnMut(B) -> C,
{
    move |a: A| { f(g(a)) }
}

pub struct Uncurry<F>(F);

impl<T, F> FnOnce<(T,)> for Uncurry<F>
    where F: FnOnce<T>
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, (args,): (T,)) -> F::Output {
        self.0.call_once(args)
    }
}

impl<T, F> FnMut<(T,)> for Uncurry<F>
    where F: FnMut<T>
{
    extern "rust-call" fn call_mut(&mut self, (args,): (T,)) -> F::Output {
        self.0.call_mut(args)
    }
}

impl<T, F> Fn<(T,)> for Uncurry<F>
    where F: Fn<T>
{
    extern "rust-call" fn call(&self, (args,): (T,)) -> F::Output {
        self.0.call(args)
    }
}


pub fn uncurry<A, F>(f: F) -> Uncurry<F> {
    Uncurry(f)
}

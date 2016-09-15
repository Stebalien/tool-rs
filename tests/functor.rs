extern crate tool;

macro_rules! if_unstable {
    ($($i:item)*) => ($(
        #[cfg(feature = "unstable")]
        $i
    )*)
}

if_unstable! {
    use tool::prelude::*;
    #[test]
    fn test_compose() {
        assert!(compose(empty, first)(("", "asdf")));
        assert!(compose(non_empty, second)(("", "asdf")));
    }

    #[test]
    fn test_flip() {
        let mut v = vec![1, 2, 3];

        flip(Vec::push)(v[0], &mut v);
        assert_eq!(v, vec![1, 2, 3, 1]);
    }

    #[test]
    fn test_fix() {
        let fib = fix(|f, x| if x == 0 || x == 1 {
            x
        } else {
            f(x - 1) + f(x - 2)
        });
        assert_eq!(55, fib(10));
    }
}

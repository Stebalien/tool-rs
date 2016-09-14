extern crate tool;
use tool::prelude::*;

macro_rules! if_unstable {
    ($($i:item)*) => ($(
        #[cfg(feature = "unstable")]
        $i
    )*)
}

if_unstable! {
    #[test]
    fn test_compose() {
        assert!(compose(empty, first)(("", "asdf")));
        assert!(compose(non_empty, second)(("", "asdf")));
    }
}

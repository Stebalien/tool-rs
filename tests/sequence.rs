extern crate tool;
use tool::sequence::prelude::*;

#[test]
fn test_index() {
    assert_eq!(first((1, 2)), 1);
    assert_eq!(first(&(1, 2)), &1);
    assert_eq!(second((1, 2)), 2);
    assert_eq!(third(&[] as &[i32]), None);
    assert_eq!(second(&[1,2][..]), Some(&2));
    assert_eq!(second(&[1,2]), &2);

    assert_eq!(first([1, 2, 3, 4]), 1);
    assert_eq!(second([1, 2, 3, 4]), 2);
    assert_eq!(third([1, 2, 3, 4]), 3);
}

#[test]
fn test_uncons() {
    let (a, b) = uncons([1, 2, 3, 4]);
    assert_eq!(a, 1);
    assert_eq!(b, [2, 3, 4]);
}

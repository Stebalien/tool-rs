#![feature(test)]
extern crate test;
extern crate tool;

use tool::sequence::*;

#[bench]
fn bench_first_slice(b: &mut test::Bencher) {
    let v = vec![1,2,3,4,5,6,7,8,9,10];
    b.iter(|| {
        test::black_box(first(test::black_box(&*v)))
    })
}

#[bench]
fn bench_third_slice(b: &mut test::Bencher) {
    let v = vec![1,2,3,4,5,6,7,8,9,10];
    b.iter(|| {
        test::black_box(third(test::black_box(&*v)).unwrap())
    })
}

#[bench]
fn bench_first_tuple(b: &mut test::Bencher) {
    let v = ("a", "b", "c", "d");
    b.iter(|| {
        test::black_box(first(test::black_box(v)))
    })
}

#[bench]
fn bench_third_tuple(b: &mut test::Bencher) {
    let v = ("a", "b", "c", "d");
    b.iter(|| {
        test::black_box(third(test::black_box(v)))
    })
}


#[bench]
fn bench_first_array_long(b: &mut test::Bencher) {
    let v = [0i32; 10];
    b.iter(|| {
        test::black_box(first(test::black_box(v)))
    })
}

#[bench]
fn bench_first_array_short(b: &mut test::Bencher) {
    let v = [0i32; 1];
    b.iter(|| {
        test::black_box(first(test::black_box(v)))
    })
}

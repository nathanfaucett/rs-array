#![feature(test)]


extern crate test;

extern crate array;


use test::Bencher;

use array::Array;


const SIZE: usize = 1024;


#[bench]
fn bench_array(b: &mut Bencher) {
    let mut array = Array::with_len(SIZE);

    array.defaults();

    b.iter(move || {
        for i in 0..SIZE {
            array[i] = i;
        }
        for i in 0..SIZE {
            assert_eq!(array[i], i);
        }
    });
}
#[bench]
fn bench_slice(b: &mut Bencher) {
    let mut array = [0usize; SIZE];

    b.iter(|| {
        for i in 0..SIZE {
            array[i] = i;
        }
        for i in 0..SIZE {
            assert_eq!(array[i], i);
        }
    });
}
#[bench]
fn bench_vec(b: &mut Bencher) {
    let mut array = [0usize; SIZE].to_vec();

    b.iter(|| {
        for i in 0..SIZE {
            array[i] = i;
        }
        for i in 0..SIZE {
            assert_eq!(array[i], i);
        }
    });
}

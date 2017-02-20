#![feature(test)]


extern crate test;

extern crate array;


use test::Bencher;

use array::Array;


const SIZE: usize = 1024;


#[bench]
fn bench_array(b: &mut Bencher) {
    b.iter(move || {
        let mut array = Array::new(SIZE);

        for i in 0..SIZE {
            array[i] = i;
        }
        for i in 0..SIZE {
            assert_eq!(array[i], i);
        }
        array
    });
}
#[bench]
fn bench_slice(b: &mut Bencher) {
    b.iter(|| {
        let mut array = [0usize; SIZE];

        for i in 0..SIZE {
            array[i] = i;
        }
        for i in 0..SIZE {
            assert_eq!(array[i], i);
        }
        array
    });
}
#[bench]
fn bench_vec(b: &mut Bencher) {
    b.iter(|| {
        let mut array = [0usize; SIZE].to_vec();

        for i in 0..SIZE {
            array[i] = i;
        }
        for i in 0..SIZE {
            assert_eq!(array[i], i);
        }
        array
    });
}

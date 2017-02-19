#![feature(test)]


extern crate test;

extern crate buffer;


use test::Bencher;

use buffer::Buffer;


const SIZE: usize = 1024;


#[bench]
fn bench_buffer(b: &mut Bencher) {
    b.iter(move || {
        let mut buffer = Buffer::new(SIZE);

        for i in 0..SIZE {
            buffer[i] = i;
        }
        for i in 0..SIZE {
            assert_eq!(buffer[i], i);
        }
        buffer
    });
}
#[bench]
fn bench_slice(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = [0usize; SIZE];

        for i in 0..SIZE {
            buffer[i] = i;
        }
        for i in 0..SIZE {
            assert_eq!(buffer[i], i);
        }
        buffer
    });
}
#[bench]
fn bench_vec(b: &mut Bencher) {
    b.iter(|| {
        let mut buffer = [0usize; SIZE].to_vec();

        for i in 0..SIZE {
            buffer[i] = i;
        }
        for i in 0..SIZE {
            assert_eq!(buffer[i], i);
        }
        buffer
    });
}

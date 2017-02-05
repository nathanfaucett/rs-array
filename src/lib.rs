#![feature(core_intrinsics)]
#![feature(dropck_parametricity)]
#![feature(heap_api)]
#![feature(oom)]
#![feature(unique)]
#![feature(alloc)]
#![no_std]


extern crate alloc;


mod array;


pub use array::Array;

#![feature(core_intrinsics)]
#![feature(dropck_parametricity)]
#![feature(heap_api)]
#![feature(oom)]
#![feature(unique)]
#![feature(alloc)]
#![no_std]


extern crate alloc;


mod fixed_array;


pub use fixed_array::FixedArray;

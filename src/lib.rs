#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![feature(inclusive_range)]
#![feature(dropck_eyepatch)]
#![feature(dropck_parametricity)]
#![feature(generic_param_attrs)]
#![feature(unique)]
#![feature(alloc)]
#![no_std]


extern crate alloc;

extern crate collection_traits;


mod array;


pub use array::Array;

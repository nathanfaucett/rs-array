rs-fixed_array [![Build Status](https://travis-ci.org/nathanfaucett/rs-fixed_array.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-fixed_array)
=====
fixed sized fixed_array

```rust
extern crate fixed_array;


use fixed_array::FixedArray;


fn main() {
    let mut fixed_array = FixedArray::new(5);

    fixed_array[0] = 1;
    fixed_array[1] = 2;
    fixed_array[2] = 3;
    fixed_array[3] = 4;
    fixed_array[4] = 5;

    assert_eq!(fixed_array[0], 1);
    assert_eq!(fixed_array[1], 2);
    assert_eq!(fixed_array[2], 3);
    assert_eq!(fixed_array[3], 4);
    assert_eq!(fixed_array[4], 5);
}
```

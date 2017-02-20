rs-array [![Build Status](https://travis-ci.org/nathanfaucett/rs-array.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-array)
=====
fixed sized array

```rust
extern crate array;


use array::Buffer;


fn main() {
    let mut array = Buffer::new(5);

    array[0] = 1;
    array[1] = 2;
    array[2] = 3;
    array[3] = 4;
    array[4] = 5;

    assert_eq!(array[0], 1);
    assert_eq!(array[1], 2);
    assert_eq!(array[2], 3);
    assert_eq!(array[3], 4);
    assert_eq!(array[4], 5);
}
```

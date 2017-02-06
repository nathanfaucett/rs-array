rs-buffer [![Build Status](https://travis-ci.org/nathanfaucett/rs-buffer.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-buffer)
=====
fixed sized buffer

```rust
extern crate buffer;


use buffer::Buffer;


fn main() {
    let mut buffer = Buffer::new(5);

    buffer[0] = 1;
    buffer[1] = 2;
    buffer[2] = 3;
    buffer[3] = 4;
    buffer[4] = 5;

    assert_eq!(buffer[0], 1);
    assert_eq!(buffer[1], 2);
    assert_eq!(buffer[2], 3);
    assert_eq!(buffer[3], 4);
    assert_eq!(buffer[4], 5);
}
```

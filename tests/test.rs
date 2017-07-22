extern crate collection_traits;

extern crate array;


use collection_traits::*;

use array::Array;


#[test]
fn test_set_len() {
    let mut array = Array::<usize>::with_len(2);
    array.defaults();

    array[0] = 1;
    array[1] = 2;

    assert_eq!(array[0], 1);
    assert_eq!(array[1], 2);
    assert_eq!(array.len(), 2);

    array.set_len(4);

    array[2] = 3;
    array[3] = 4;

    assert_eq!(array[0], 1);
    assert_eq!(array[1], 2);
    assert_eq!(array[2], 3);
    assert_eq!(array[3], 4);
    assert_eq!(array.len(), 4);

    array.set_len(2);

    assert_eq!(array[0], 1);
    assert_eq!(array[1], 2);
    assert_eq!(array.len(), 2);
}

#[test]
fn test_get() {
    let mut array = Array::<usize>::with_len(5);
    array.defaults();

    assert_eq!(array[0], 0);
    assert_eq!(array[1], 0);
    assert_eq!(array[2], 0);
    assert_eq!(array[3], 0);
    assert_eq!(array[4], 0);
}
#[test]
fn test_get_mut() {
    let mut array = Array::<usize>::with_len(5);
    array.defaults();

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

#[test]
fn test_get_clone_mut() {
    let mut a = Array::<usize>::with_len(3);
    a.defaults();

    let mut b = a.clone();

    a[0] = 1;
    a[1] = 2;
    a[2] = 3;

    b[0] = 4;
    b[1] = 5;
    b[2] = 6;

    assert_eq!(a[0], 1);
    assert_eq!(a[1], 2);
    assert_eq!(a[2], 3);

    assert_eq!(b[0], 4);
    assert_eq!(b[1], 5);
    assert_eq!(b[2], 6);
}

#[derive(Debug, PartialEq, Eq)]
enum Enum {
    Empty,
    Full
}

impl Default for Enum {
    fn default() -> Self {
        Enum::Empty
    }
}

#[test]
fn test_empty_get() {
    let mut array = Array::<Enum>::with_len(3);
    array.defaults();

    assert_eq!(array[0], Enum::Empty);
    assert_eq!(array[1], Enum::Empty);
    assert_eq!(array[2], Enum::Empty);
}
#[test]
fn test_empty_get_mut() {
    let mut array = Array::<Enum>::with_len(5);
    array.defaults();

    array[0] = Enum::Full;
    array[1] = Enum::Full;
    array[2] = Enum::Full;

    assert_eq!(array[0], Enum::Full);
    assert_eq!(array[1], Enum::Full);
    assert_eq!(array[2], Enum::Full);
}

#[test]
fn test_iter() {
    let mut array = Array::<usize>::with_len(5);
    array.defaults();

    for value in array.iter() {
        assert_eq!(*value, 0);
    }
}
#[test]
fn test_iter_mut() {
    let mut array = Array::<usize>::with_len(5);
    array.defaults();

    for value in array.iter_mut() {
        *value = 1;
    }
    for value in array.iter() {
        assert_eq!(*value, 1);
    }
}

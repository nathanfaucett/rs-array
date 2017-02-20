extern crate array;


use array::Array;



#[test]
fn test_resize() {
    let mut array = Array::<usize>::new(2);

    array[0] = 1;
    array[1] = 2;

    assert_eq!(array[0], 1);
    assert_eq!(array[1], 2);
    assert_eq!(array.len(), 2);

    array.resize(4);

    array[2] = 3;
    array[3] = 4;

    assert_eq!(array[0], 1);
    assert_eq!(array[1], 2);
    assert_eq!(array[2], 3);
    assert_eq!(array[3], 4);
    assert_eq!(array.len(), 4);

    array.resize(2);

    assert_eq!(array[0], 1);
    assert_eq!(array[1], 2);
    assert_eq!(array.len(), 2);
}
#[test]
fn test_get() {
    let array = Array::<usize>::new(5);

    assert_eq!(array[0], 0);
    assert_eq!(array[1], 0);
    assert_eq!(array[2], 0);
    assert_eq!(array[3], 0);
    assert_eq!(array[4], 0);
}
#[test]
fn test_get_mut() {
    let mut array = Array::<usize>::new(5);

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
    let mut a = Array::<usize>::new(3);
    let b = a.clone();

    a[0] = 1;
    a[1] = 2;
    a[2] = 3;

    assert_eq!(a[0], 1);
    assert_eq!(a[1], 2);
    assert_eq!(a[2], 3);

    assert_eq!(b[0], 0);
    assert_eq!(b[1], 0);
    assert_eq!(b[2], 0);
}

#[derive(Debug, PartialEq, Eq)]
struct EMPTY;

#[test]
fn test_empty_get() {
    let array = Array::<EMPTY>::new(3);

    assert_eq!(array[0], EMPTY);
    assert_eq!(array[1], EMPTY);
    assert_eq!(array[2], EMPTY);
}
#[test]
fn test_empty_get_mut() {
    let mut array = Array::<EMPTY>::new(5);

    array[0] = EMPTY;
    array[1] = EMPTY;
    array[2] = EMPTY;

    assert_eq!(array[0], EMPTY);
    assert_eq!(array[1], EMPTY);
    assert_eq!(array[2], EMPTY);
}
#[test]
fn test_empty_get_mut_resize() {
    let mut array = Array::<EMPTY>::new(3);

    array.resize(1);
    assert_eq!(array[0], EMPTY);

    array.resize(3);
    assert_eq!(array[0], EMPTY);
    assert_eq!(array[1], EMPTY);
    assert_eq!(array[2], EMPTY);
}

#[test]
fn test_iter() {
    let array = Array::<usize>::new(5);

    for value in array.iter() {
        assert_eq!(*value, 0);
    }
}
#[test]
fn test_iter_mut() {
    let mut array = Array::<usize>::new(5);

    for value in array.iter_mut() {
        *value = 1;
    }
    for value in array.iter() {
        assert_eq!(*value, 1);
    }
}

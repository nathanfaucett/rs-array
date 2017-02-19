extern crate buffer;


use buffer::Buffer;


#[test]
fn test_resize() {
    let mut buffer = Buffer::<usize>::new(2);

    buffer[0] = 1;
    buffer[1] = 2;

    assert_eq!(buffer[0], 1);
    assert_eq!(buffer[1], 2);
    assert_eq!(buffer.len(), 2);

    buffer.resize(4);

    buffer[2] = 3;
    buffer[3] = 4;

    assert_eq!(buffer[0], 1);
    assert_eq!(buffer[1], 2);
    assert_eq!(buffer[2], 3);
    assert_eq!(buffer[3], 4);
    assert_eq!(buffer.len(), 4);

    buffer.resize(2);

    assert_eq!(buffer[0], 1);
    assert_eq!(buffer[1], 2);
    assert_eq!(buffer.len(), 2);
}
#[test]
fn test_get() {
    let buffer = Buffer::<usize>::new(5);

    assert_eq!(buffer[0], 0);
    assert_eq!(buffer[1], 0);
    assert_eq!(buffer[2], 0);
    assert_eq!(buffer[3], 0);
    assert_eq!(buffer[4], 0);
}
#[test]
fn test_get_mut() {
    let mut buffer = Buffer::<usize>::new(5);

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

#[test]
fn test_get_clone_mut() {
    let mut a = Buffer::<usize>::new(3);
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
    let buffer = Buffer::<EMPTY>::new(3);

    assert_eq!(buffer[0], EMPTY);
    assert_eq!(buffer[1], EMPTY);
    assert_eq!(buffer[2], EMPTY);
}
#[test]
fn test_empty_get_mut() {
    let mut buffer = Buffer::<EMPTY>::new(5);

    buffer[0] = EMPTY;
    buffer[1] = EMPTY;
    buffer[2] = EMPTY;

    assert_eq!(buffer[0], EMPTY);
    assert_eq!(buffer[1], EMPTY);
    assert_eq!(buffer[2], EMPTY);
}
#[test]
fn test_empty_get_mut_resize() {
    let mut buffer = Buffer::<EMPTY>::new(3);

    buffer.resize(1);
    assert_eq!(buffer[0], EMPTY);

    buffer.resize(3);
    assert_eq!(buffer[0], EMPTY);
    assert_eq!(buffer[1], EMPTY);
    assert_eq!(buffer[2], EMPTY);
}

#[test]
fn test_iter() {
    let buffer = Buffer::<usize>::new(5);

    for value in buffer.iter() {
        assert_eq!(*value, 0);
    }
}
#[test]
fn test_iter_mut() {
    let mut buffer = Buffer::<usize>::new(5);

    for value in buffer.iter_mut() {
        *value = 1;
    }
    for value in buffer.iter() {
        assert_eq!(*value, 1);
    }
}

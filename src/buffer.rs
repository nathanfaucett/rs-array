use alloc::{oom, heap};
use alloc::boxed::Box;

use core::isize;
use core::ptr::{self, Unique};
use core::{mem, slice};
use core::intrinsics::assume;
use core::ops::{Deref, DerefMut};


pub struct Buffer<T> {
    len: usize,
    ptr: Unique<T>,
}

impl<T> Buffer<T> {

    pub fn new(len: usize) -> Self {
        unsafe {
            let elem_size = mem::size_of::<T>();

            let alloc_size = len.checked_mul(elem_size).expect("lenacity overflow");
            alloc_guard(alloc_size);

            let ptr = if alloc_size == 0 {
                heap::EMPTY as *mut u8
            } else {
                let align = mem::align_of::<T>();
                let ptr = heap::allocate(alloc_size, align);
                if ptr.is_null() {
                    oom()
                }
                ptr
            };

            Buffer {
                ptr: Unique::new(ptr as *mut _),
                len: len,
            }
        }
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize) -> Self {
        Buffer {
            ptr: Unique::new(ptr),
            len: len,
        }
    }

    #[inline]
    pub fn from_box(mut slice: Box<[T]>) -> Self {
        unsafe {
            let result = Buffer::from_raw_parts(slice.as_mut_ptr(), slice.len());
            mem::forget(slice);
            result
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn ptr(&self) -> *mut T {
        *self.ptr
    }

    #[inline(never)]
    #[cold]
    pub fn resize(&mut self, new_len: usize) {
        unsafe {
            let elem_size = mem::size_of::<T>();

            assert!(elem_size != 0, "capacity overflow");

            let align = mem::align_of::<T>();

            let new_alloc_size = new_len * elem_size;
            alloc_guard(new_alloc_size);
            let ptr = heap::reallocate(
                self.ptr() as *mut _,
                self.len * elem_size,
                new_alloc_size,
                align
            );

            if ptr.is_null() {
                oom()
            }

            self.ptr = Unique::new(ptr as *mut _);
            self.len = new_len;
        }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();

        if elem_size != 0 && self.len != 0 {
            let align = mem::align_of::<T>();

            let num_bytes = elem_size * self.len;
            unsafe {
                heap::deallocate(*self.ptr as *mut _, num_bytes, align);
            }
        }
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            let p = *self.ptr;
            assume(!p.is_null());
            slice::from_raw_parts(p, self.len)
        }
    }
}
impl<T> DerefMut for Buffer<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let p = *self.ptr;
            assume(!p.is_null());
            slice::from_raw_parts_mut(p, self.len)
        }
    }
}

impl<T: Clone> Clone for Buffer<T> {
    fn clone(&self) -> Self {
        let cloned = Buffer::<T>::new(self.len);
        unsafe {
            ptr::copy(*self.ptr as *const _, *cloned.ptr, self.len);
        }
        cloned
    }
}

#[inline]
fn alloc_guard(alloc_size: usize) {
    if mem::size_of::<usize>() < 8 {
        assert!(alloc_size <= isize::MAX as usize, "len overflow");
    }
}


#[cfg(test)]
mod test {
    use super::*;


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
}

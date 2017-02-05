use alloc::{oom, heap};

use core::isize;
use core::ptr::Unique;
use core::mem;
use core::slice;
use core::ops::{Index, IndexMut};


pub struct Array<T> {
    size: usize,
    ptr: Unique<T>,
}

impl<T> Array<T> {
    pub fn new(size: usize) -> Self {
        unsafe {
            let elem_size = mem::size_of::<T>();

            let alloc_size = size.checked_mul(elem_size).expect("size overflow");
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

            Array {
                ptr: Unique::new(ptr as *mut _),
                size: size,
            }
        }
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        &slice::from_raw_parts(*self.ptr, self.size)[index]
    }
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        &mut slice::from_raw_parts_mut(*self.ptr, self.size)[index]
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.size {
            Some(unsafe {
                self.get_unchecked(index)
            })
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.size {
            Some(unsafe {
                self.get_unchecked_mut(index)
            })
        } else {
            None
        }
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();

        if elem_size != 0 && self.size != 0 {
            let align = mem::align_of::<T>();

            let num_bytes = elem_size * self.size;
            unsafe {
                heap::deallocate(*self.ptr as *mut _, num_bytes, align);
            }
        }
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.get_unchecked(index) }
    }
}
impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(index) }
    }
}


#[inline]
fn alloc_guard(alloc_size: usize) {
    if mem::size_of::<usize>() < 8 {
        assert!(alloc_size <= isize::MAX as usize, "size overflow");
    }
}


#[cfg(test)]
mod test {
    use super::*;


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
}

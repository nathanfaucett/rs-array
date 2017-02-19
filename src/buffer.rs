use alloc::{oom, heap};
use alloc::boxed::Box;

use core::isize;
use core::ptr::{self, Unique};
use core::{mem, slice, fmt};
use core::ops::{Deref, DerefMut};


pub struct Buffer<T> {
    len: usize,
    ptr: Unique<T>,
}

impl<T> Buffer<T> {

    pub fn new(len: usize) -> Self {
        unsafe {
            let elem_size = mem::size_of::<T>();

            let alloc_size = len.checked_mul(elem_size).expect("capacity overflow");
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

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn ptr(&self) -> *mut T {
        *self.ptr
    }

    #[inline(never)]
    #[cold]
    pub fn resize(&mut self, new_len: usize) {
        unsafe {
            let elem_size = mem::size_of::<T>();

            if elem_size != 0 {
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
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();

        if elem_size != 0 && self.len != 0 {
            unsafe {
                heap::deallocate(
                    *self.ptr as *mut _,
                    elem_size * self.len,
                    mem::align_of::<T>()
                );
            }
        }
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(*self.ptr, self.len)
        }
    }
}
impl<T> DerefMut for Buffer<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            slice::from_raw_parts_mut(*self.ptr, self.len)
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

impl<T: fmt::Debug> fmt::Debug for Buffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[inline]
fn alloc_guard(alloc_size: usize) {
    if mem::size_of::<usize>() < 8 {
        assert!(alloc_size <= isize::MAX as usize, "capacity overflow");
    }
}

use alloc::{oom, heap};
use alloc::boxed::Box;

use core::isize;
use core::intrinsics::assume;
use core::ptr::{self, Unique};
use core::{mem, slice, fmt};
use core::ops::{Deref, DerefMut};


pub struct Array<T> {
    len: usize,
    ptr: Unique<T>,
}

impl<T: Default> Array<T> {
    #[inline]
    pub fn new(len: usize) -> Self {
        let array = Array::uninitialized(len);
        unsafe {
            memdefault(*array.ptr as *mut T, 0, len);
        }
        array
    }
    #[inline]
    pub fn resize(&mut self, new_len: usize) {
        let old_len = self.len;

        self.resize_uninitialized(new_len);

        if new_len > old_len {
            let offset = new_len - old_len;

            unsafe {
                memdefault(*self.ptr, offset, new_len);
            }
        }
    }
}

impl<T> Array<T> {
    #[inline(never)]
    pub fn uninitialized(len: usize) -> Self {
        unsafe {
            let alloc_size = len.checked_mul(mem::size_of::<T>()).expect("capacity overflow");
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
                len: len,
            }
        }
    }
    #[inline]
    pub fn zeroed(len: usize) -> Self {
        let array = Array::uninitialized(len);
        unsafe {
            memdefault(*array.ptr as *mut u8, 0, len * mem::size_of::<T>());
        }
        array
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize) -> Self {
        Array {
            ptr: Unique::new(ptr),
            len: len,
        }
    }

    #[inline]
    pub fn from_box(mut slice: Box<[T]>) -> Self {
        unsafe {
            let result = Array::from_raw_parts(slice.as_mut_ptr(), slice.len());
            mem::forget(slice);
            result
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        if mem::size_of::<T>() == 0 {
            !0
        } else {
            self.len
        }
    }

    #[inline(always)]
    pub fn ptr(&self) -> *mut T {
        *self.ptr
    }

    #[inline(never)]
    #[cold]
    pub fn resize_uninitialized(&mut self, new_len: usize) {
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
    #[inline]
    pub fn resize_zeroed(&mut self, new_len: usize) {
        let elem_size = mem::size_of::<T>();
        let new_alloc_size = new_len * elem_size;
        let old_alloc_size = self.len * elem_size;

        self.resize_uninitialized(new_len);

        if new_alloc_size > old_alloc_size {
            let offset = new_alloc_size - old_alloc_size;
            unsafe {
                memdefault(*self.ptr as *mut u8, offset, new_alloc_size);
            }
        }
    }
}

impl<T> Drop for Array<T> {
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

impl<T> Deref for Array<T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {
            let p = *self.ptr;
            assume(!p.is_null());
            slice::from_raw_parts(p, self.len)
        }
    }
}
impl<T> DerefMut for Array<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let p = *self.ptr;
            assume(!p.is_null());
            slice::from_raw_parts_mut(p, self.len)
        }
    }
}

impl<T: Clone> Clone for Array<T> {
    fn clone(&self) -> Self {
        let cloned = Array::uninitialized(self.len);
        unsafe {
            ptr::copy(*self.ptr as *const _, *cloned.ptr, self.len);
        }
        cloned
    }
}

impl<T: fmt::Debug> fmt::Debug for Array<T> {
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

#[inline]
unsafe fn memdefault<T: Default>(ptr: *mut T, offset: usize, len: usize) -> *mut T {
    let mut slice = slice::from_raw_parts_mut(ptr, len);

    for i in offset..len {
        *slice.get_unchecked_mut(i) = T::default();
    }

    ptr
}

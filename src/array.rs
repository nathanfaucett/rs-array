use alloc::{oom, heap};
use alloc::boxed::Box;

use core::isize;
use core::intrinsics::assume;
use core::ptr::Unique;
use core::{mem, slice, fmt};
use core::ops::{
    Index, IndexMut, Deref, DerefMut,
    Range, RangeFrom, RangeTo, RangeFull, RangeInclusive, RangeToInclusive
};

use collection_traits::*;


pub struct Array<T> {
    len: usize,
    ptr: Unique<T>,
}

impl<T: Default> Array<T> {
    #[inline]
    pub fn new(len: usize) -> Self {
        let array = Array::uninitialized(len);
        unsafe {
            memdefault(array.ptr.as_ptr() as *mut T, 0, len);
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
                memdefault(self.ptr.as_ptr(), offset, new_len);
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
                Unique::empty().as_ptr() as *mut u8
            } else {
                let align = mem::align_of::<T>();
                let ptr = heap::allocate(alloc_size, align);
                if ptr.is_null() {
                    oom()
                }
                ptr
            };

            Array {
                ptr: Unique::new(ptr as *mut T),
                len: len,
            }
        }
    }
    #[inline]
    pub fn zeroed(len: usize) -> Self {
        let array = Array::uninitialized(len);
        unsafe {
            memdefault(array.ptr.as_ptr() as *mut u8, 0, len * mem::size_of::<T>());
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
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
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
                    self.ptr.as_ptr() as *mut u8,
                    self.len * elem_size,
                    new_alloc_size,
                    align
                );

                if ptr.is_null() {
                    oom()
                }

                self.ptr = Unique::new(ptr as *mut T);
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
                memdefault(self.ptr.as_ptr() as *mut u8, offset, new_alloc_size);
            }
        }
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();

        if elem_size != 0 && self.len != 0 {
            let align = mem::align_of::<T>();

            unsafe {
                heap::deallocate(self.ptr.as_ptr() as *mut u8, elem_size * self.len, align);
            }
        }
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {
            let p = self.ptr.as_ptr();
            assume(!p.is_null());
            slice::from_raw_parts(p, self.len)
        }
    }
}
impl<T> DerefMut for Array<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let p = self.ptr.as_ptr();
            assume(!p.is_null());
            slice::from_raw_parts_mut(p, self.len)
        }
    }
}

impl<T: Clone> Clone for Array<T> {
    #[inline]
    fn clone(&self) -> Self {
        let cloned = Array::uninitialized(self.len);
        unsafe {
            let mut slice = slice::from_raw_parts_mut(cloned.as_ptr(), self.len);

            for i in 0..self.len {
                *slice.get_unchecked_mut(i) = self[i].clone();
            }
        }
        cloned
    }
}

impl<T> Collection for Array<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> CollectionMut for Array<T> {
    #[inline]
    fn clear(&mut self) {
        unsafe {
            memdefault(self.ptr.as_ptr() as *mut u8, 0, self.len * mem::size_of::<T>());
        }
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &(**self)[index]
    }
}
impl<T> IndexMut<usize> for Array<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (**self)[index]
    }
}

impl<T> Index<Range<usize>> for Array<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: Range<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}
impl<T> Index<RangeTo<usize>> for Array<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeTo<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}
impl<T> Index<RangeFrom<usize>> for Array<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeFrom<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}
impl<T> Index<RangeFull> for Array<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, _index: RangeFull) -> &[T] {
        self
    }
}
impl<T> Index<RangeInclusive<usize>> for Array<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeInclusive<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}
impl<T> Index<RangeToInclusive<usize>> for Array<T> {
    type Output = [T];

    #[inline(always)]
    fn index(&self, index: RangeToInclusive<usize>) -> &[T] {
        Index::index(&**self, index)
    }
}

impl<T> IndexMut<Range<usize>> for Array<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: Range<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}
impl<T> IndexMut<RangeTo<usize>> for Array<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}
impl<T> IndexMut<RangeFrom<usize>> for Array<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}
impl<T> IndexMut<RangeFull> for Array<T> {
    #[inline(always)]
    fn index_mut(&mut self, _index: RangeFull) -> &mut [T] {
        self
    }
}
impl<T> IndexMut<RangeInclusive<usize>> for Array<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}
impl<T> IndexMut<RangeToInclusive<usize>> for Array<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut [T] {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl<'a, T: 'a> Iterable<'a, &'a T> for Array<T> {
    type Iter = slice::Iter<'a, T>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        (&**self).iter()
    }
}

impl<'a, T: 'a> IterableMut<'a, &'a mut T> for Array<T> {
    type IterMut = slice::IterMut<'a, T>;

    #[inline(always)]
    fn iter_mut(&'a mut self) -> Self::IterMut {
        (&mut **self).iter_mut()
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

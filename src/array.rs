use alloc::heap::Heap;
use alloc::allocator::{Alloc, Layout};
use alloc::boxed::Box;

use core::intrinsics::assume;
use core::ptr::Unique;
use core::{mem, slice, fmt};
use core::ops::*;

use collection_traits::*;


pub struct Array<T, A: Alloc = Heap> {
    ptr: Unique<T>,
    len: usize,
    a: A,
}

impl<T, A: Alloc> Array<T, A> {

    #[inline]
    pub fn new_in(a: A) -> Self {
        let len = if mem::size_of::<T>() == 0 { !0 } else { 0 };

        Array {
            ptr: Unique::empty(),
            len: len,
            a: a,
        }
    }

    #[inline(always)]
    pub fn with_len_in(len: usize, a: A) -> Self {
        Array::allocate_in(len, false, a)
    }

    #[inline(always)]
    pub fn with_len_zeroed_in(len: usize, a: A) -> Self {
        Array::allocate_in(len, true, a)
    }

    #[inline]
    fn allocate_in(len: usize, zeroed: bool, mut a: A) -> Self {
        unsafe {
            let elem_size = mem::size_of::<T>();

            let alloc_size = len.checked_mul(elem_size).expect("capacity overflow");
            alloc_guard(alloc_size);

            let ptr = if alloc_size == 0 {
                mem::align_of::<T>() as *mut u8
            } else {
                let align = mem::align_of::<T>();
                let result = if zeroed {
                    a.alloc_zeroed(Layout::from_size_align(alloc_size, align).unwrap())
                } else {
                    a.alloc(Layout::from_size_align(alloc_size, align).unwrap())
                };
                match result {
                    Ok(ptr) => ptr,
                    Err(err) => a.oom(err),
                }
            };

            Array {
                ptr: Unique::new(ptr as *mut _),
                len: len,
                a: a,
            }
        }
    }
}

impl<T> Array<T, Heap> {

    #[inline(always)]
    pub fn new() -> Self {
        Self::new_in(Heap)
    }

    #[inline(always)]
    pub fn with_len(len: usize) -> Self {
        Array::allocate_in(len, false, Heap)
    }

    #[inline(always)]
    pub fn with_len_zeroed(len: usize) -> Self {
        Array::allocate_in(len, true, Heap)
    }

    #[inline]
    pub fn set_len(&mut self, new_len: usize) {
        unsafe {
            let elem_size = mem::size_of::<T>();

            assert!(elem_size != 0, "capacity overflow");

            let ptr_res = if self.len == 0 {
                let ptr_res = self.a.alloc_array::<T>(new_len);
                ptr_res
            } else {
                let new_alloc_size = new_len * elem_size;
                alloc_guard(new_alloc_size);
                let ptr_res = self.a.realloc_array(self.ptr, self.len, new_len);
                ptr_res
            };

            let uniq = match ptr_res {
                Err(err) => self.a.oom(err),
                Ok(uniq) => uniq,
            };

            self.ptr = uniq;
            self.len = new_len;
        }
    }
}

impl<T: Default, A: Alloc> Array<T, A> {

    #[inline(always)]
    pub fn defaults(&mut self) {
        unsafe {
           memdefault(self.ptr.as_ptr() as *mut T, 0, self.len);
       }
    }
}

impl<T, A: Alloc> Array<T, A> {

    #[inline(always)]
    pub unsafe fn from_raw_parts_in(ptr: *mut T, len: usize, a: A) -> Self {
        Array {
            ptr: Unique::new(ptr),
            len: len,
            a: a,
        }
    }
}

impl<T> Array<T, Heap> {

    #[inline(always)]
    pub unsafe fn from_raw_parts(ptr: *mut T, len: usize) -> Self {
        Array {
            ptr: Unique::new(ptr),
            len: len,
            a: Heap,
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
}

impl<T, A: Alloc> Array<T, A> {

    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
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
    pub fn alloc(&self) -> &A {
        &self.a
    }

    #[inline(always)]
    pub fn alloc_mut(&mut self) -> &mut A {
        &mut self.a
    }
}

impl<T> Array<T, Heap> {

    #[inline]
    pub unsafe fn into_box(self) -> Box<[T]> {
        let slice = slice::from_raw_parts_mut(self.as_ptr(), self.len);
        let output: Box<[T]> = Box::from_raw(slice);
        mem::forget(self);
        output
    }
}

impl<T, A: Alloc> Array<T, A> {

    #[inline]
    pub unsafe fn dealloc_buffer(&mut self) {
        let elem_size = mem::size_of::<T>();

        if elem_size != 0 && self.len != 0 {
            let ptr = self.as_ptr() as *mut u8;
            let layout = Layout::new::<T>().repeat(self.len).unwrap().0;
            self.a.dealloc(ptr, layout);
        }
    }
}

unsafe impl<#[may_dangle] T, A: Alloc> Drop for Array<T, A> {

    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.dealloc_buffer();
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
        let mut cloned = Array::with_len(self.len);
        {
            let mut slice: &mut [T] = &mut *cloned;

            for i in 0..self.len {
                slice[i] = self[i].clone();
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
    #[inline(always)]
    fn clear(&mut self) {
        self.set_len(0);
    }
}

impl<T> Get<usize> for Array<T> {
    type Output = T;

    #[inline(always)]
    fn get(&self, index: usize) -> Option<&Self::Output> {
        (**self).get(index)
    }
}
impl<T> GetMut<usize> for Array<T> {
    #[inline(always)]
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        (**self).get_mut(index)
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
        assert!(alloc_size <= ::core::isize::MAX as usize, "capacity overflow");
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

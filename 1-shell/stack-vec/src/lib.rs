#![cfg_attr(test, feature(inclusive_range_syntax))]
#![no_std]

#[cfg(test)]
mod tests;

extern crate std;

use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;
use std::ops::IndexMut;
use std::iter::IntoIterator;
use std::cmp;

/// A contiguous array type backed by a slice.
///
/// `StackVec`'s functionality is similar to that of `std::Vec`. You can `push`
/// and `pop` and iterate over the vector. Unlike `Vec`, however, `StackVec`
/// requires no memory allocation as it is backed by a user-supplied slice. As a
/// result, `StackVec`'s capacity is _bounded_ by the user-supplied slice. This
/// results in `push` being fallible: if `push` is called when the vector is
/// full, an `Err` is returned.
#[derive(Debug)]
pub struct StackVec<'a, T: 'a> {
    storage: &'a mut [T],
    len: usize,
    _head: usize,
}

impl<'a, T: 'a> StackVec<'a, T> {
    /// Constructs a new, empty `StackVec<T>` using `storage` as the backing
    /// store. The returned `StackVec` will be able to hold `storage.len()`
    /// values.
    pub fn new(storage: &'a mut [T]) -> StackVec<'a, T> {
        let len = storage.len();
        StackVec { storage: storage, len: len, _head: 0 }
    }

    /// Constructs a new `StackVec<T>` using `storage` as the backing store. The
    /// first `len` elements of `storage` are treated as if they were `push`ed
    /// onto `self.` The returned `StackVec` will be able to hold a total of
    /// `storage.len()` values.
    ///
    /// # Panics
    ///
    /// Panics if `len > storage.len()`.
    pub fn with_len(storage: &'a mut [T], len: usize) -> StackVec<'a, T> {
        let storage_len = storage.len();
        if len > storage_len {
            panic!();
        }
        StackVec { storage: storage, len: storage_len, _head: len }
    }

    /// Returns the number of elements this vector can hold.
    pub fn capacity(&self) -> usize {
        // (self.len - self._head)
        self.len
    }

    /// Shortens the vector, keeping the first `len` elements. If `len` is
    /// greater than the vector's current length, this has no effect. Note that
    /// this method has no effect on the capacity of the vector.
    pub fn truncate(&mut self, len: usize) {
        let new_len = len.checked_sub(1).unwrap_or(0);
        self._head = cmp::min(self.len, new_len);
    }

    /// Extracts a slice containing the entire vector, consuming `self`.
    ///
    /// Note that the returned slice's length will be the length of this vector,
    /// _not_ the length of the original backing storage.
    pub fn into_slice(self) -> &'a mut [T] {
        &mut self.storage[0..self._head]
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        &self.storage[0..self._head]
    }

    /// Extracts a mutable slice of the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.storage[0..self._head]
    }

    /// Returns the number of elements in the vector, also referred to as its
    /// 'length'.
    pub fn len(&self) -> usize {
        self._head
    }

    /// Returns true if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self._head == 0
    }

    /// Returns true if the vector is at capacity.
    pub fn is_full(&self) -> bool {
        self._head == self.len
    }

    /// Appends `value` to the back of this vector if the vector is not full.
    ///
    /// # Error
    ///
    /// If this vector is full, an `Err` is returned. Otherwise, `Ok` is
    /// returned.
    pub fn push(&mut self, value: T) -> Result<(), ()> {
        if self.is_full() {
            Err(())
        } else {
            self.storage[self._head] = value;
            self._head += 1;
            Ok(())
        }
    }
}

impl<'a, T: Clone + 'a> StackVec<'a, T> {
    /// If this vector is not empty, removes the last element from this vector
    /// by cloning it and returns it. Otherwise returns `None`.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None
        }
        // println!("Head is {}", self._head);
        let last = &self.storage[self._head];
        self._head -= 1;
        Some(last.clone())
    }
}

impl<'a, T> Deref for StackVec<'a, T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        // let boxed: Box<[T]> = &self.storage.into_boxed_slice()
        &self.storage
    }
}

impl<'a, T> Index<usize> for StackVec<'a, T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len() {
            panic!("Index OOB");
        }
        &self.storage[index]
    }
}

impl<'a, T> IndexMut<usize> for StackVec<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len() {
            panic!("Index OOB");
        }
        &mut self.storage[index]
    }
}

pub struct StackIterator<'a, T: 'a> {
    _vec: StackVec<'a, T>,
    next: usize,
}

impl<'a, T> Iterator for StackIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self._vec.len() {
            return None
        }
        let ref item = &self._vec[self.next];
        self.next += 1;
        Some(&item)
    }
}

impl<'a, T> IntoIterator for StackVec<'a, T> {
    type Item = &'a T;
    type IntoIter = StackIterator<'a, T>;

    fn into_iter(self) -> StackIterator<'a, T> {
        StackIterator { _vec: self, next: 0 }
    }
}

impl<'a, T> IntoIterator for &'a StackVec<'a, T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> core::slice::Iter<'a, T> {
        self.storage.iter()
    }
}

impl<'a, T> DerefMut for StackVec<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.storage
    }
}

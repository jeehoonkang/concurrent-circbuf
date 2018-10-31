use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use epoch::{self, AtomicTmpl, OwnedTmpl, Storage};

pub type AtomicArray<T> = AtomicTmpl<Array<T>, ArrayBox<T>>;
pub type OwnedArray<T> = OwnedTmpl<Array<T>, ArrayBox<T>>;

#[derive(Debug)]
pub struct Slot<T> {
    index: AtomicUsize,
    data: UnsafeCell<T>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Array<T> {
    inner: epoch::Array<Slot<T>>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ArrayBox<T> {
    inner: epoch::ArrayBox<Slot<T>>,
}

impl<T> Array<T> {
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    pub unsafe fn at(&self, index: usize) -> *mut Slot<T> {
        // `array.size()` is always a power of two.
        &*self.inner.at(index & (self.inner.size() - 1)) as &Slot<T> as *const _ as *mut Slot<T>
    }

    /// Reads a value from the specified `index`.
    ///
    /// Returns `Some(v)` if `v` is at `index`, and `None` if there is no valid value for `index`.
    pub unsafe fn read(&self, index: usize) -> Option<T> {
        let ptr = self.at(index);

        // Read the index with `Acquire`.
        let i = (*ptr).index.load(Ordering::Acquire);

        // If the index written in the array mismatches with the queried index, there's no valid
        // value.
        if index != i {
            return None;
        }

        // Read the value.
        Some((*ptr).data.get().read())
    }

    /// Writes `value` into the specified `index`.
    pub unsafe fn write(&self, index: usize, value: T) {
        let ptr = self.at(index);

        // Write the value.
        (*ptr).data.get().write(value);

        // Write the index with `Release`.
        (*ptr).index.store(index, Ordering::Release);
    }
}

impl<T> ArrayBox<T> {
    pub fn new(cap: usize) -> Self {
        // `cap` should be a power of two.
        debug_assert_eq!(cap, cap.next_power_of_two());

        // Creates an array.
        let inner = epoch::ArrayBox::<Slot<T>>::new(cap);

        // Mark all entries invalid. Concretely, for each entry at `i`, we put the index `i + 1`, which
        // is invalid. This is because the `i`-th entry can contain only elements with the index `i + N
        // * cap`, where `N` is an integer.
        unsafe {
            for i in 0..cap {
                let index = &(*inner.at(i)).index as *const _ as *mut AtomicUsize;
                ptr::write(index, AtomicUsize::new(i + 1));
            }
        }

        Self { inner }
    }
}

unsafe impl<T> Storage<Array<T>> for ArrayBox<T> {
    fn into_raw(self) -> *mut Array<T> {
        let ptr = self.inner.into_raw();
        ptr as *mut Array<T>
    }

    unsafe fn from_raw(ptr: *mut Array<T>) -> Self {
        Self {
            inner: epoch::ArrayBox::from_raw(ptr as *mut epoch::Array<Slot<T>>),
        }
    }
}

impl<T> Deref for ArrayBox<T> {
    type Target = Array<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.inner.deref() as *const epoch::Array<Slot<T>> as *const Array<T>) }
    }
}

impl<T> DerefMut for ArrayBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *(self.inner.deref() as *const epoch::Array<Slot<T>> as *const Array<T>
                as *mut Array<T>)
        }
    }
}

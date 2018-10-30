//! Array underlying circular buffers.

use std::mem;
use std::ptr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// C++'s std::pair<T, U>.
#[repr(C)]
pub struct CPair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> CPair<T, U> {
    pub fn fst_mut(ptr: *mut Self) -> *mut T {
        unsafe {
            let ptr = ptr as *mut u8;
            ptr.add(offset_of!(Self, first)) as *mut T
        }
    }

    pub fn snd_mut(ptr: *mut Self) -> *mut U {
        unsafe {
            let ptr = ptr as *mut u8;
            ptr.add(offset_of!(Self, second)) as *mut U
        }
    }
}

/// An array that holds elements in a circular buffer.
pub struct Array<T> {
    /// Pointer to the allocated memory. The value `(i, v)` means the `i`-th value `v` of the
    /// buffer.
    ptr: *mut CPair<AtomicUsize, T>,

    /// Capacity of the array. Always a power of two.
    pub cap: usize,
}

unsafe impl<T> Send for Array<T> {}

impl<T> Array<T> {
    /// Returns a new array with the specified capacity.
    pub fn new(cap: usize) -> Self {
        // `cap` should be a power of two.
        debug_assert_eq!(cap, cap.next_power_of_two());

        // Creates an array and gets its raw pointer.
        let mut v = Vec::with_capacity(cap);
        let ptr: *mut CPair<AtomicUsize, T> = v.as_mut_ptr();
        mem::forget(v);

        // Mark all entries invalid. Concretely, for each entry at `i`, we put the index `i + 1`,
        // which is invalid. This is because the `i`-th entry can contain only elements with the
        // index `i + N * cap`, where `N` is an integer.
        unsafe {
            for i in 0..cap {
                // FIXME: use `MaybeUninit` when it's stabilized:
                // https://github.com/rust-lang/rust/issues/53491
                let mut slot: CPair<AtomicUsize, _> = mem::uninitialized();
                slot.first = AtomicUsize::new(i + 1);
                ptr::write(ptr.add(i), slot);
            }
        }

        Array { ptr, cap }
    }

    /// Returns a pointer to the element at the specified `index`.
    pub unsafe fn at(&self, index: usize) -> *mut CPair<AtomicUsize, T> {
        // `self.cap` is always a power of two.
        self.ptr.add(index & (self.cap - 1))
    }

    /// Writes `value` into the specified `index`.
    pub unsafe fn write(&self, index: usize, value: T) {
        let ptr = self.at(index);

        // Write the value.
        ptr::write(CPair::snd_mut(ptr), value);

        // Write the index with `Release`.
        (*CPair::fst_mut(ptr)).store(index, Ordering::Release);
    }

    /// Reads a value from the specified `index`.
    ///
    /// Returns `Some(v)` if `v` is at `index`, and `None` if there is no valid value for `index`.
    pub unsafe fn read(&self, index: usize) -> Option<T> {
        let ptr = self.at(index);

        // Read the index with `Acquire`.
        let i = (*CPair::fst_mut(ptr)).load(Ordering::Acquire);

        // If the index written in the array mismatches with the queried index, there's no valid
        // value.
        if index != i {
            return None;
        }

        // Read the value.
        Some(ptr::read(CPair::snd_mut(ptr)))
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        // Array itself doesn't claim any ownership of the values.
        unsafe {
            drop(Vec::from_raw_parts(self.ptr, 0, self.cap));
        }
    }
}

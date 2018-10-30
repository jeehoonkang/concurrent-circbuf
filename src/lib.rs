//! Concurrent channels based on circular buffer.
//!
//! Currently, this crate provides the following flavors of channels:
//!
//! - bounded/unbounded SPSC (single-producer single-consumer)
//! - bounded/unbounded SPMC (single-producer multiple-consumer)
//! - bounded MPMC (multiple-producer multiple-consumer)

#![warn(missing_docs, missing_debug_implementations)]

extern crate crossbeam_epoch as epoch;
extern crate crossbeam_utils as utils;
#[macro_use]
extern crate memoffset;

mod array;
#[doc(hidden)] // for doc-tests
pub mod mp;
#[doc(hidden)] // for doc-tests
pub mod sp;

pub use sp::mc as spmc;
pub use sp::sc as spsc;

/// The return type for [`CircBuf::try_recv`], [`DynamicCircBuf::try_recv`], and
/// [`Receiver::try_recv`].
///
/// [`CircBuf::try_recv`]: struct.CircBuf.html#method.try_recv
/// [`DynamicCircBuf::try_recv`]: struct.DynamicCircBuf.html#method.try_recv
/// [`Receiver::try_recv`]: struct.Receiver.html#method.try_recv
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryRecv<T> {
    /// `try_recv` received an element.
    Data(T),
    /// The circular buffer is empty.
    Empty,
    /// Lost the race for receiving data to another concurrent operation. Try again.
    Retry,
}

impl<T> TryRecv<T> {
    /// Apply a function to the content of `TryRecv::Data`.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> TryRecv<U> {
        match self {
            TryRecv::Data(v) => TryRecv::Data(f(v)),
            TryRecv::Empty => TryRecv::Empty,
            TryRecv::Retry => TryRecv::Retry,
        }
    }
}

//! Bounded and unbounded, SPSC and SPMC channels based on concurrent circular buffer.

#![deny(missing_docs, warnings, missing_debug_implementations)]

extern crate crossbeam_epoch as epoch;
extern crate crossbeam_utils as utils;
#[macro_use]
extern crate memoffset;

pub mod base;
pub use base::TryRecv;

pub mod bounded;
pub mod unbounded;

//! SPSC and SPMC channels based on dyamically growable and shrinkable circular buffer.

#![deny(missing_docs, warnings, missing_debug_implementations)]

extern crate crossbeam_epoch as epoch;
extern crate crossbeam_utils as utils;

pub mod base;
pub use base::RecvError;

pub mod spsc;
pub mod spmc;

//! Concurrent queues based on circular buffer.
//!
//! Currently, this crate provides the following flavors of queues:
//!
//! - bounded/unbounded SPSC (single-producer single-consumer)
//! - bounded/unbounded SPMC (single-producer multiple-consumer)
//! - bounded MPMC (multiple-producer multiple-consumer)

#![warn(missing_docs, missing_debug_implementations)]

extern crate crossbeam_epoch as epoch;
extern crate crossbeam_utils as utils;
#[macro_use]
extern crate memoffset;

pub mod base;
pub use base::TryRecv;

pub mod bounded;
pub mod unbounded;

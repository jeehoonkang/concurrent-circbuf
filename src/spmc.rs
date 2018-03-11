//! An SPMC channel based on dynamically growbale and shrinkable circular buffer.
//!
//! # Examples
//!
//! ```
//! use concurrent_circbuf::spmc::{Channel, Receiver};
//! use std::thread;
//!
//! let c = Channel::new();
//! let r = c.receiver();
//!
//! c.send('a');
//! c.send('b');
//! c.send('c');
//!
//! assert_eq!(c.try_recv(), Ok(Some('a')));
//! drop(c);
//!
//! thread::spawn(move || {
//!     assert_eq!(r.try_recv(), Ok(Some('b')));
//!     assert_eq!(r.try_recv(), Ok(Some('c')));
//! }).join().unwrap();
//! ```

use base;

#[derive(Debug)]
pub struct Channel<T>(base::CircBuf<T>);

#[derive(Debug)]
pub struct Receiver<T>(base::Receiver<T>);

impl<T> Channel<T> {
    pub fn new() -> Self {
        Channel { 0: base::CircBuf::new() }
    }

    pub fn with_min_capacity(min_cap: usize) -> Self {
        Channel { 0: base::CircBuf::with_min_capacity(min_cap) }
    }

    pub fn send(&self, value: T) {
        self.0.send(value)
    }

    pub fn try_recv(&self) -> Result<Option<T>, base::RecvError> {
        self.0.try_recv()
    }

    pub fn receiver(&self) -> Receiver<T> {
        Receiver { 0: self.0.receiver() }
    }
}

impl<T> Receiver<T> {
    pub fn try_recv(&self) -> Result<Option<T>, base::RecvError> {
        self.0.try_recv()
    }
}

impl<T> Clone for Receiver<T> {
    /// Creates another receiver.
    fn clone(&self) -> Receiver<T> {
        Receiver { 0: self.0.clone() }
    }
}

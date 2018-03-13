//! An SPMC channel based on dynamically growbale and shrinkable circular buffer.
//!
//! # Examples
//!
//! ```
//! use concurrent_circbuf::spmc::{Channel, Receiver};
//! use std::thread;
//!
//! let c = Channel::<char>::new();
//! let r = c.receiver();
//!
//! c.send('a');
//! c.send('b');
//! c.send('c');
//!
//! assert_eq!(c.recv(), Some('a'));
//! drop(c);
//!
//! thread::spawn(move || {
//!     assert_eq!(r.recv(), Some('b'));
//!     assert_ne!(r.try_recv(), Ok(None)); // Ok(Some('c')) or Err(RecvError::Retry)
//! }).join().unwrap();
//! ```

use base::{self, RecvError};

/// An SPMC channel.
#[derive(Debug)]
pub struct Channel<T>(base::CircBuf<T>);

/// The receiver of an SPMC channel.
#[derive(Debug)]
pub struct Receiver<T>(base::Receiver<T>);

impl<T> Channel<T> {
    /// Creates an SPMC channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::spmc::{Channel, Receiver};
    ///
    /// let c = Channel::<u32>::new();
    /// ```
    pub fn new() -> Self {
        Channel { 0: base::CircBuf::new() }
    }

    /// Creates an SPMC channel with the specified minimal capacity.
    ///
    /// If the capacity is not a power of two, it will be rounded up to the next one.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::spmc::{Channel, Receiver};
    ///
    /// // The minimum capacity will be rounded up to 1024.
    /// let c = Channel::<u32>::with_min_capacity(1000);
    /// ```
    pub fn with_min_capacity(min_cap: usize) -> Self {
        Channel { 0: base::CircBuf::with_min_capacity(min_cap) }
    }

    /// Sends an element to the channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::spmc::{Channel, Receiver};
    ///
    /// let c = Channel::<u32>::new();
    /// c.send(1);
    /// c.send(2);
    /// ```
    pub fn send(&self, value: T) {
        self.0.send(value)
    }

    /// Receives an element from the channel.
    ///
    /// It returns `Ok(Some(v))` if a value `v` is received, and `Ok(None)` if the channel is
    /// empty. Unlike most methods in concurrent data structures, if another operation gets in the
    /// way while attempting to receive data, this method will bail out immediately with
    /// [`RecvError::Retry`] instead of retrying.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::spmc::{Channel, Receiver};
    ///
    /// let c = Channel::<u32>::new();
    /// c.send(1);
    /// c.send(2);
    ///
    /// assert_ne!(c.try_recv(), Ok(None));
    /// assert_ne!(c.try_recv(), Ok(None));
    /// ```
    ///
    /// [`RecvError::Retry`]: enum.RecvError.html#variant.Retry
    pub fn try_recv(&self) -> Result<Option<T>, RecvError> {
        self.0.try_recv()
    }

    /// Receives an element from the channel.
    ///
    /// It returns `Some(v)` if a value `v` is received, and `None` if the circular buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::spmc::{Channel, Receiver};
    ///
    /// let c = Channel::<u32>::new();
    /// c.send(1);
    /// c.send(2);
    ///
    /// assert_eq!(c.recv(), Some(1));
    /// assert_eq!(c.recv(), Some(2));
    /// ```
    pub fn recv(&self) -> Option<T> {
        self.0.recv()
    }

    /// Creates a receiver for the channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::spmc::{Channel, Receiver};
    ///
    /// let c = Channel::<u32>::new();
    /// let r = c.receiver();
    /// ```
    pub fn receiver(&self) -> Receiver<T> {
        Receiver { 0: self.0.receiver() }
    }
}

impl<T> Receiver<T> {
    /// Receives an element from the channel.
    ///
    /// It returns `Ok(Some(v))` if a value `v` is received, and `Ok(None)` if the channel is
    /// empty. Unlike most methods in concurrent data structures, if another operation gets in the
    /// way while attempting to receive data, this method will bail out immediately with
    /// [`RecvError::Retry`] instead of retrying.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::spmc::{Channel, Receiver};
    ///
    /// let c = Channel::<u32>::new();
    /// c.send(1);
    /// c.send(2);
    ///
    /// let r = c.receiver();
    /// assert_ne!(r.try_recv(), Ok(None));
    /// assert_ne!(r.try_recv(), Ok(None));
    /// ```
    ///
    /// [`RecvError::Retry`]: enum.RecvError.html#variant.Retry
    pub fn try_recv(&self) -> Result<Option<T>, RecvError> {
        self.0.try_recv()
    }

    /// Receives an element from the channel.
    ///
    /// It returns `Some(v)` if a value `v` is received, and `None` if the circular buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::spmc::{Channel, Receiver};
    ///
    /// let c = Channel::<u32>::new();
    /// c.send(1);
    /// c.send(2);
    ///
    /// let r = c.receiver();
    /// assert_eq!(r.recv(), Some(1));
    /// assert_eq!(r.recv(), Some(2));
    /// ```
    pub fn recv(&self) -> Option<T> {
        self.0.recv()
    }
}

impl<T> Clone for Receiver<T> {
    /// Creates another receiver.
    fn clone(&self) -> Receiver<T> {
        Receiver { 0: self.0.clone() }
    }
}

//! Unbounded SPSC channel based on dynamically growable and shrinkable concurrent circular buffer.
//!
//! # Examples
//!
//! ```
//! use concurrent_circbuf::unbounded::spsc;
//! use std::thread;
//!
//! let (tx, rx) = spsc::new::<char>();
//!
//! tx.send('a');
//! tx.send('b');
//! tx.send('c');
//!
//! assert_eq!(rx.recv(), Some('a'));
//! drop(tx);
//!
//! thread::spawn(move || {
//!     assert_eq!(rx.recv(), Some('b'));
//!     assert_eq!(rx.recv(), Some('c'));
//! }).join().unwrap();
//! ```

use sp;

/// The sender of an unbounded SPSC channel.
#[derive(Debug)]
pub struct Sender<T>(sp::DynamicCircBuf<T>);

/// The receiver of an unbounded SPSC channel.
#[derive(Debug)]
pub struct Receiver<T>(sp::Receiver<T>);

unsafe impl<T> Send for Receiver<T> {}

/// Creates an unbounded SPSC channel, and returns its sender and receiver.
///
/// # Examples
///
/// ```
/// use concurrent_circbuf::unbounded::spsc;
///
/// let (tx, rx) = spsc::new::<u32>();
/// ```
pub fn new<T>() -> (Sender<T>, Receiver<T>) {
    let circbuf = sp::DynamicCircBuf::new();
    let receiver = circbuf.receiver();
    let sender = Sender { 0: circbuf };
    let receiver = Receiver { 0: receiver };
    (sender, receiver)
}

/// Creates an unbounded SPSC channel with the specified minimal capacity, and returns its sender and
/// receiver.
///
/// If the capacity is not a power of two, it will be rounded up to the next one.
///
/// # Examples
///
/// ```
/// use concurrent_circbuf::unbounded::spsc;
///
/// // The minimum capacity will be rounded up to 1024.
/// let (tx, rx) = spsc::with_min_capacity::<u32>(1000);
/// ```
pub fn with_min_capacity<T>(min_cap: usize) -> (Sender<T>, Receiver<T>) {
    let circbuf = sp::DynamicCircBuf::with_min_capacity(min_cap);
    let receiver = circbuf.receiver();
    let sender = Sender { 0: circbuf };
    let receiver = Receiver { 0: receiver };
    (sender, receiver)
}

impl<T> Sender<T> {
    /// Sends an element to the channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::unbounded::spsc;
    ///
    /// let (tx, rx) = spsc::new::<u32>();
    /// tx.send(1);
    /// tx.send(2);
    /// ```
    pub fn send(&self, value: T) {
        self.0.send(value)
    }
}

impl<T> Receiver<T> {
    /// Receives an element from the channel.
    ///
    /// It returns `Some(v)` if `v` is received, and `None` if the channel is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::unbounded::spsc;
    ///
    /// let (tx, rx) = spsc::new::<u32>();
    /// tx.send(32);
    /// assert_eq!(rx.recv(), Some(32));
    /// ```
    pub fn recv(&self) -> Option<T> {
        // It is safe to call `recv_exclusive()`, because `Sender` doesn't receive at all, and I'm
        // the only receiver and `Receiver` is not `Sync`.
        unsafe { self.0.recv_exclusive() }
    }
}

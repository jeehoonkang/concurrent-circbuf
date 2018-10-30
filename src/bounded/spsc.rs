//! Bounded SPSC channel based on fixed-sized concurrent circular buffer.
//!
//! # Examples
//!
//! ```
//! use concurrent_circbuf::bounded::spsc;
//! use std::thread;
//!
//! let (tx, rx) = spsc::new::<char>(16);
//!
//! tx.send('a').unwrap();
//! tx.send('b').unwrap();
//! tx.send('c').unwrap();
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

/// The sender of a bounded SPSC channel.
#[derive(Debug)]
pub struct Sender<T>(sp::CircBuf<T>);

/// The receiver of a bounded SPSC channel.
#[derive(Debug)]
pub struct Receiver<T>(sp::Receiver<T>);

unsafe impl<T> Send for Receiver<T> {}

/// Creates a bounded SPSC channel with the specified capacity, and returns its sender and
/// receiver.
///
/// If the capacity is not a power of two, it will be rounded up to the next one.
///
/// # Examples
///
/// ```
/// use concurrent_circbuf::bounded::spsc;
///
/// let (tx, rx) = spsc::new::<u32>(16);
/// ```
pub fn new<T>(cap: usize) -> (Sender<T>, Receiver<T>) {
    let circbuf = sp::CircBuf::new(cap);
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
    /// use concurrent_circbuf::bounded::spsc;
    ///
    /// let (tx, rx) = spsc::new::<u32>(16);
    /// tx.send(1).unwrap();
    /// tx.send(2).unwrap();
    /// ```
    pub fn send(&self, value: T) -> Result<(), T> {
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
    /// use concurrent_circbuf::bounded::spsc;
    ///
    /// let (tx, rx) = spsc::new::<u32>(16);
    /// tx.send(32).unwrap();
    /// assert_eq!(rx.recv(), Some(32));
    /// ```
    pub fn recv(&self) -> Option<T> {
        // It is safe to call `recv_exclusive()`, because `Sender` doesn't receive at all, and I'm
        // the only receiver and `Receiver` is not `Sync`.
        unsafe { self.0.recv_exclusive() }
    }
}

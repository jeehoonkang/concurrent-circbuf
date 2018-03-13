//! An SPSC channel based on dynamically growbale and shrinkable circular buffer.
//!
//! # Examples
//!
//! ```
//! use concurrent_circbuf::spsc;
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

use std::marker::PhantomData;

use base;

/// The sender of an SPSC channel.
#[derive(Debug)]
pub struct Sender<T>(base::CircBuf<T>);

/// The receiver of an SPSC channel.
#[derive(Debug)]
pub struct Receiver<T> {
    receiver: base::Receiver<T>,
    _marker: PhantomData<*mut ()>, // !Send + !Sync
}

unsafe impl<T> Send for Receiver<T> {}

/// Creates an SPSC channel, and returns its sender and receiver.
///
/// # Examples
///
/// ```
/// use concurrent_circbuf::spsc;
///
/// let (tx, rx) = spsc::new::<u32>();
/// ```
pub fn new<T>() -> (Sender<T>, Receiver<T>) {
    let circbuf = base::CircBuf::new();
    let receiver = circbuf.receiver();
    let sender = Sender { 0: circbuf };
    let receiver = Receiver { receiver: receiver, _marker: PhantomData };
    (sender, receiver)
}

/// Creates an SPSC channel with the specified minimal capacity, and returns its sender and
/// receiver.
///
/// If the capacity is not a power of two, it will be rounded up to the next one.
///
/// # Examples
///
/// ```
/// use concurrent_circbuf::spsc;
///
/// // The minimum capacity will be rounded up to 1024.
/// let (tx, rx) = spsc::with_min_capacity::<u32>(1000);
/// ```
pub fn with_min_capacity<T>(min_cap: usize) -> (Sender<T>, Receiver<T>) {
    let circbuf = base::CircBuf::with_min_capacity(min_cap);
    let receiver = circbuf.receiver();
    let sender = Sender { 0: circbuf };
    let receiver = Receiver { receiver: receiver, _marker: PhantomData };
    (sender, receiver)
}

impl<T> Sender<T> {
    /// Sends an element to the channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use concurrent_circbuf::spsc;
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
    /// use concurrent_circbuf::spsc;
    ///
    /// let (tx, rx) = spsc::new::<u32>();
    /// tx.send(32);
    /// assert_eq!(rx.recv(), Some(32));
    /// ```
    pub fn recv(&self) -> Option<T> {
        // It is safe to call `recv_exclusive()`, because `Sender` doesn't receive at all, and I'm
        // the only receiver and `Receiver` is not `Sync`.
        unsafe { self.receiver.recv_exclusive() }
    }
}

//! Unbounded SPSC and SPMC channels based on dynamically growable and shrinkable concurrent
//! circular buffer.

/// Unbounded SPSC channel based on dynamically growable and shrinkable concurrent circular buffer.
///
/// # Examples
///
/// ```
/// use concurrent_circbuf::unbounded::spsc;
/// use std::thread;
///
/// let (tx, rx) = spsc::new::<char>();
///
/// tx.send('a');
/// tx.send('b');
/// tx.send('c');
///
/// assert_eq!(rx.recv(), Some('a'));
/// drop(tx);
///
/// thread::spawn(move || {
///     assert_eq!(rx.recv(), Some('b'));
///     assert_eq!(rx.recv(), Some('c'));
/// }).join().unwrap();
/// ```
pub mod spsc {
    use base;

    /// The sender of an unbounded SPSC channel.
    #[derive(Debug)]
    pub struct Sender<T>(base::DynamicCircBuf<T>);

    /// The receiver of an unbounded SPSC channel.
    #[derive(Debug)]
    pub struct Receiver<T>(base::Receiver<T>);

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
        let circbuf = base::DynamicCircBuf::new();
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
        let circbuf = base::DynamicCircBuf::with_min_capacity(min_cap);
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
}

/// Unbounded SPMC channel based on dynamically growable and shrinkable concurrent circular buffer.
///
/// # Examples
///
/// ```
/// use concurrent_circbuf::unbounded::spmc::{Channel, Receiver, TryRecv};
/// use std::thread;
///
/// let c = Channel::<char>::new();
/// let r = c.receiver();
///
/// c.send('a');
/// c.send('b');
/// c.send('c');
///
/// assert_ne!(c.try_recv(), TryRecv::Empty); // TryRecv::Data('a') or TryRecv::Retry
/// drop(c);
///
/// thread::spawn(move || {
///     assert_ne!(r.try_recv(), TryRecv::Empty);
///     assert_ne!(r.try_recv(), TryRecv::Empty);
/// }).join().unwrap();
/// ```
pub mod spmc {
    use base;
    pub use base::TryRecv;

    /// an unbounded SPMC channel.
    #[derive(Debug)]
    pub struct Channel<T>(base::DynamicCircBuf<T>);

    /// The receiver of an unbounded SPMC channel.
    #[derive(Debug)]
    pub struct Receiver<T>(base::Receiver<T>);

    impl<T> Channel<T> {
        /// Creates an unbounded SPMC channel.
        ///
        /// # Examples
        ///
        /// ```
        /// use concurrent_circbuf::unbounded::spmc::{Channel, Receiver};
        ///
        /// let c = Channel::<u32>::new();
        /// ```
        pub fn new() -> Self {
            Channel {
                0: base::DynamicCircBuf::new(),
            }
        }

        /// Creates an unbounded SPMC channel with the specified minimal capacity.
        ///
        /// If the capacity is not a power of two, it will be rounded up to the next one.
        ///
        /// # Examples
        ///
        /// ```
        /// use concurrent_circbuf::unbounded::spmc::{Channel, Receiver};
        ///
        /// // The minimum capacity will be rounded up to 1024.
        /// let c = Channel::<u32>::with_min_capacity(1000);
        /// ```
        pub fn with_min_capacity(min_cap: usize) -> Self {
            Channel {
                0: base::DynamicCircBuf::with_min_capacity(min_cap),
            }
        }

        /// Sends an element to the channel.
        ///
        /// # Examples
        ///
        /// ```
        /// use concurrent_circbuf::unbounded::spmc::{Channel, Receiver};
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
        /// It returns [`TryRecv::Data`] if a value is received, and [`TryRecv::Empty`] if the channel
        /// is empty. Unlike most methods in concurrent data structures, if another operation gets in
        /// the way while attempting to receive data, this method will bail out immediately with
        /// [`TryRecv::Retry`] instead of retrying.
        ///
        /// # Examples
        ///
        /// ```
        /// use concurrent_circbuf::unbounded::spmc::{Channel, Receiver, TryRecv};
        ///
        /// let c = Channel::<u32>::new();
        /// c.send(1);
        /// c.send(2);
        ///
        /// assert_ne!(c.try_recv(), TryRecv::Empty);
        /// assert_ne!(c.try_recv(), TryRecv::Empty);
        /// ```
        ///
        /// [`TryRecv::Data`]: enum.TryRecv.html#variant.Data
        /// [`TryRecv::Empty`]: enum.TryRecv.html#variant.Empty
        /// [`TryRecv::Retry`]: enum.TryRecv.html#variant.Retry
        pub fn try_recv(&self) -> TryRecv<T> {
            self.0.try_recv()
        }

        /// Creates a receiver for the channel.
        ///
        /// # Examples
        ///
        /// ```
        /// use concurrent_circbuf::unbounded::spmc::{Channel, Receiver};
        ///
        /// let c = Channel::<u32>::new();
        /// let r = c.receiver();
        /// ```
        pub fn receiver(&self) -> Receiver<T> {
            Receiver {
                0: self.0.receiver(),
            }
        }
    }

    impl<T> Default for Channel<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> Receiver<T> {
        /// Receives an element from the channel.
        ///
        /// It returns [`TryRecv::Data`] if a value is received, and [`TryRecv::Empty`] if the
        /// channel is empty. Unlike most methods in concurrent data structures, if another
        /// operation gets in the way while attempting to receive data, this method will bail out
        /// immediately with [`TryRecv::Retry`] instead of retrying.
        ///
        /// # Examples
        ///
        /// ```
        /// use concurrent_circbuf::unbounded::spmc::{Channel, Receiver, TryRecv};
        ///
        /// let c = Channel::<u32>::new();
        /// c.send(1);
        /// c.send(2);
        ///
        /// let r = c.receiver();
        /// assert_ne!(r.try_recv(), TryRecv::Empty);
        /// assert_ne!(r.try_recv(), TryRecv::Empty);
        /// ```
        ///
        /// [`TryRecv::Data`]: enum.TryRecv.html#variant.Data
        /// [`TryRecv::Empty`]: enum.TryRecv.html#variant.Empty
        /// [`TryRecv::Retry`]: enum.TryRecv.html#variant.Retry
        pub fn try_recv(&self) -> TryRecv<T> {
            self.0.try_recv()
        }

        /// Receives half the elements from the channel.
        ///
        /// It returns [`TryRecv::Data`] if a value is received, and [`TryRecv::Empty`] if the
        /// channel is empty. Unlike most methods in concurrent data structures, if another
        /// operation gets in the way while attempting to receive data, this method will bail out
        /// immediately with [`TryRecv::Retry`] instead of retrying.
        ///
        /// # Examples
        ///
        /// ```
        /// use concurrent_circbuf::unbounded::spmc::{Channel, Receiver, TryRecv};
        ///
        /// let c = Channel::<u32>::new();
        /// c.send(1);
        /// c.send(2);
        ///
        /// let r = c.receiver();
        /// assert_eq!(r.try_recv_half(), TryRecv::Data(vec![1]));
        /// assert_eq!(r.try_recv_half(), TryRecv::Data(vec![2]));
        /// ```
        ///
        /// [`TryRecv::Data`]: enum.TryRecv.html#variant.Data
        /// [`TryRecv::Empty`]: enum.TryRecv.html#variant.Empty
        /// [`TryRecv::Retry`]: enum.TryRecv.html#variant.Retry
        pub fn try_recv_half(&self) -> TryRecv<Vec<T>> {
            self.0.try_recv_half()
        }
    }

    impl<T> Clone for Receiver<T> {
        /// Creates another receiver.
        fn clone(&self) -> Receiver<T> {
            Receiver { 0: self.0.clone() }
        }
    }
}

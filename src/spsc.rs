//! An SPSC channel based on dynamically growbale and shrinkable circular buffer.

use base;

#[derive(Debug)]
pub struct Sender<T>(base::CircBuf<T>);

#[derive(Debug)]
pub struct Receiver<T>(base::Receiver<T>);

pub fn new<T>() -> (Sender<T>, Receiver<T>) {
    let circbuf = base::CircBuf::new();
    let receiver = circbuf.receiver();
    (Sender { 0: circbuf }, Receiver { 0: receiver })
}

pub fn with_min_capacity<T>(min_cap: usize) -> (Sender<T>, Receiver<T>) {
    let circbuf = base::CircBuf::with_min_capacity(min_cap);
    let receiver = circbuf.receiver();
    (Sender { 0: circbuf }, Receiver { 0: receiver })
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        self.0.send(value)
    }
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Option<T> {
        unsafe { self.0.recv_exclusive() }
    }
}

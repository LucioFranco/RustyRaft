use futures::sync::mpsc::UnboundedSender;
use std::net::ToSocketAddrs;

pub struct Connection<T> {
    tx: UnboundedSender<T>,
}

impl<T> Connection<T> {
    pub fn new(tx: UnboundedSender<T>) -> Self {
        Connection { tx }
    }

    pub fn connect<A>(peer: A)
    where
        A: ToSocketAddrs,
    {
        unimplemented!()
    }
}

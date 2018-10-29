use futures::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::{Future, Sink, Stream};
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, RwLock};
use tokio::codec::{Decoder, Encoder};
use tokio::net::TcpListener;
use uuid::Uuid;

use super::{codec::MessageCodec, NetError};
use messages::Message;

/// Node is the main networking interface for the Raft Server.
pub struct Node {
    addr: SocketAddr,
    inner: Arc<NodeInner<Message>>,
}

impl Node {
    /// Create a new Node with a set of peers
    pub fn new<A>(addr: SocketAddr, _peers: A) -> Self
    where
        A: ToSocketAddrs,
    {
        // TODO: do something with the peers
        let inner = NodeInner::new();

        Node {
            addr,
            inner: Arc::new(inner),
        }
    }

    /// Returns a future to be run on the tokio runtime.
    pub fn run(&self) -> impl Future<Item = (), Error = NetError> {
        let listener = TcpListener::bind(&self.addr).unwrap();

        let inner = self.inner.clone();
        listener
            .incoming()
            .for_each(move |stream| {
                let (sink, stream) = MessageCodec.framed(stream).split();
                let (tx, rx) = mpsc::unbounded();

                // let inner_clone = inner.clone();
                // let read =
                //     stream.for_each(move |msg| Node::process(inner_clone.clone(), msg, tx.clone()));
                // tokio::spawn(read.map_err(|_| ()));

                let write = sink.send_all(rx.map_err(|()| {
                    std::io::Error::new(std::io::ErrorKind::Other, "rx shouldn't have an error")
                }));
                tokio::spawn(write.map(|_| ()).map_err(|_| ()));

                Ok(())
            }).map_err(|err| NetError::from(err))
    }

    fn process(
        inner: Arc<NodeInner<Message>>,
        msg: Message,
        tx: UnboundedSender<Message>,
    ) -> Result<(), std::io::Error> {
        match msg {
            _ => unimplemented!(),
        }
    }
}

struct NodeInner<T> {
    id: Uuid,
    peers: RwLock<HashMap<Uuid, UnboundedSender<T>>>,
}

impl<T> NodeInner<T> {
    pub fn new() -> Self {
        NodeInner {
            id: Uuid::new_v4(),
            // The peers list starts empty to be filled once clients start to connect
            peers: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_peer(&self, id: Uuid, tx: UnboundedSender<T>) -> Result<Uuid, NetError> {
        let mut peers = self.peers.write().expect("unable to acquire write lock");

        peers.insert(id, tx);

        Ok(id)
    }

    // pub fn get_peer(&self, id: Uuid) -> Result<Option<&UnboundedSender<T>>, NetError> {
    //     Ok(self.peers.read().expect("unable to get peer id").get(&id))
    // }
}

pub enum NodeError {
    Io(std::io::Error),
}

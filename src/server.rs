use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::{Future, Stream};

use tokio::net::{TcpListener, TcpStream};

use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::{Arc, RwLock};

use connection::Connection;
use messages::{Message, MessageType};

pub type ServerId = u8;

pub struct Server {
    inner: Arc<ServerInner>,
}

impl Server {
    fn new(id: ServerId, peers: &HashMap<ServerId, SocketAddr>) -> Self {
        let inner = ServerInner {
            id,
            peers: peers.clone(),
        };

        Server {
            inner: Arc::new(inner),
        }
    }

    pub fn run(&self) -> impl Future<Item = (), Error = std::io::Error> {
        let inner = self.inner.clone();
        let addr = inner.peers.get(&inner.id).unwrap();

        let listener = TcpListener::bind(&addr).unwrap();

        listener.incoming().for_each(move |stream| Ok(()))
    }
}

pub struct ServerInner {
    id: ServerId,
    peers: HashMap<ServerId, SocketAddr>,
}

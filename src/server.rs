use futures::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::Future;

use tokio::net::TcpStream;

use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};

use connection::Connection;
use messages::{Message, MessageType};

pub type ServerId = u8;
pub type ClientId = u8;

pub struct Server {
    id: ServerId,
    peers: HashMap<ServerId, Connection<MessageType>>,
    clients: HashMap<ClientId, Connection<MessageType>>,
}

impl Server {
    fn new(id: ServerId, peers: &HashMap<u8, SocketAddr>) {
        unimplemented!()
    }

    fn new_client(&mut self, socket: TcpStream, tx: UnboundedSender<MessageType>) {
        unimplemented!()
    }

    pub fn run(id: ServerId, peers: &HashMap<ServerId, SocketAddr>) {
        unimplemented!();
    }

    fn connect_to_peers(&mut self, peers: &HashMap<ServerId, SocketAddr>) {
        unimplemented!();
    }

    fn get_connection(
        peer: &SocketAddr,
        rx: UnboundedReceiver<MessageType>,
        inital_message: Message,
    ) {
        unimplemented!()
    }
}
